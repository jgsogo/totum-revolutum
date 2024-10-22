from datetime import date

from django.core.exceptions import ObjectDoesNotExist, ValidationError
from django.db.models import ProtectedError
from django.test import TestCase
from finances_data.models import (
    Account,
    AccountType,
    Custodian,
    Fx,
    MovementDividend,
    MovementNonNumerable,
    MovementNumerable,
    MovementType,
    SnapshotNumerable,
    Transaction,
)
from finances_data.models.movement import Direction

from .test_model__amount import AmountNonNumerableTestMixin, AmountNumerableTestsMixin


class BaseMovementTestCase(TestCase):
    def setUp(self):
        self.transaction = Transaction.objects.create(name="transaction")
        self.acctype = AccountType.objects.create(name="acctype")

        self.custodian = Custodian.objects.create(name="custodian", country="ES")
        self.account1 = Account.objects.create(
            name="acc1",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
            ccy="EUR",
        )
        self.account2 = Account.objects.create(
            name="acc2",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
            ccy="EUR",
        )

        self.movtype = MovementType.objects.create(name="movtype")


class MovementNonNumerableTestCase(AmountNonNumerableTestMixin, BaseMovementTestCase):
    def _create_instance(self, amount: float):
        return MovementNonNumerable(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1900, 1, 1),
            amount=amount,
        )

    def test_default_ordering(self):
        mov1 = MovementNonNumerable.objects.create(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1999, 1, 1),
            amount=1000.50,
        )
        mov2 = MovementNonNumerable.objects.create(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(2000, 1, 1),
            amount=1000.50,
        )

        all_movs = MovementNonNumerable.objects.all()
        self.assertListEqual(list(all_movs), [mov2, mov1])

    """
    The following tests only run in one of the variants, as they share the source code logic
    """

    def test_transaction_cascade(self):
        # If we remove a Transaction, all its movements are removed as well
        t = Transaction.objects.create(name="transaction")
        mov1 = MovementNonNumerable.objects.create(
            transaction=t,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1900, 1, 1),
            amount=1000.50,
        )
        mov2 = MovementNonNumerable.objects.create(
            transaction=t,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1900, 1, 1),
            amount=1000.50,
        )

        self.assertTrue(MovementNonNumerable.objects.filter(pk=mov1.pk).exists())
        self.assertTrue(MovementNonNumerable.objects.filter(pk=mov2.pk).exists())

        t.delete()
        self.assertFalse(MovementNonNumerable.objects.filter(pk=mov1.pk).exists())
        self.assertFalse(MovementNonNumerable.objects.filter(pk=mov2.pk).exists())

    def test_type_protect(self):
        self._create_instance(amount=10).save()
        with self.assertRaises(ProtectedError) as cm:
            self.movtype.delete()

        self.assertEqual(
            str(cm.exception),
            "(\"Cannot delete some instances of model 'MovementType' because they are referenced"
            " through protected foreign keys: 'MovementNonNumerable.type'.\","
            " {<MovementNonNumerable: MovementNonNumerable object (1)>})",
        )

    def test_account_protect(self):
        self._create_instance(amount=10).save()
        with self.assertRaises(ProtectedError) as cm:
            self.account1.delete()

        self.assertEqual(
            str(cm.exception),
            "(\"Cannot delete some instances of model 'Account' because they are referenced"
            " through protected foreign keys: 'MovementNonNumerable.account'.\","
            " {<MovementNonNumerable: MovementNonNumerable object (1)>})",
        )

    def test_fx_protect(self):
        d = date(1900, 1, 1)
        fx = Fx.objects.create(local="EUR", foreign="USD", date_value=d, rate=1.25)
        MovementNonNumerable.objects.create(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=d,
            amount=1000.50,
            fx=fx,
        )

        with self.assertRaises(ProtectedError) as cm:
            fx.delete()

        self.assertEqual(
            str(cm.exception),
            "(\"Cannot delete some instances of model 'Fx' because they are referenced"
            " through protected foreign keys: 'MovementNonNumerable.fx'.\","
            " {<MovementNonNumerable: MovementNonNumerable object (1)>})",
        )

    def test_validation_fx_date_movement(self):
        # FX date_value has to match movement date
        fx = Fx.objects.create(local="EUR", foreign="USD", date_value=date(1900, 1, 1), rate=1.25)
        m = MovementNonNumerable.objects.create(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1900, 1, 2),
            amount=1000.50,
            fx=fx,
        )

        with self.assertRaises(ValidationError) as cm:
            m.full_clean()

        self.assertListEqual(cm.exception.messages, ["Date_value for FX and Movement didn't match"])

    def test_validation_fx_ccy(self):
        # FX foreign has to match account.ccy
        fx = Fx.objects.create(local="USD", foreign="EUR", date_value=date(1900, 1, 1), rate=1.25)
        m = MovementNonNumerable.objects.create(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1900, 1, 1),
            amount=1000.50,
            fx=fx,
        )

        with self.assertRaises(ValidationError) as cm:
            m.full_clean()

        self.assertListEqual(
            cm.exception.messages, ["FX local currency and Account currency didn't match"]
        )

    def test_validate_date(self):
        # Movement date_value inside [account.open, account.close] dates
        account = Account.objects.create(
            name="acc2",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
            close=date(1901, 1, 1),
            ccy="EUR",
        )

        m = MovementNonNumerable(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=account,
            date_value=date(1899, 1, 1),
            amount=1000.50,
        )

        with self.assertRaises(ValidationError) as cm:
            m.full_clean()

        self.assertListEqual(
            cm.exception.messages, ["Movement date_value should be after Account open"]
        )

        m = MovementNonNumerable(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=account,
            date_value=date(1902, 1, 1),
            amount=1000.50,
        )

        with self.assertRaises(ValidationError) as cm:
            m.full_clean()

        self.assertListEqual(
            cm.exception.messages, ["Movement date_vale cannot be after account is closed"]
        )


class MovementNumerableTestCase(AmountNumerableTestsMixin, BaseMovementTestCase):

    def _create_instance(self, quantity: float, unit_value: float):
        return MovementNumerable(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1900, 1, 1),
            quantity=quantity,
            unit_value=unit_value,
        )


class MovementDividendTestCase(BaseMovementTestCase):
    def test_get_amount(self):
        mov = MovementDividend(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1900, 1, 1),
            ex_dividend_date=date(1910, 1, 1),
            unit_value=2.2,
        )

        with self.assertRaisesMessage(
            ObjectDoesNotExist,
            "Associated account doesn't have any Snapshot previous to the dividend ex_date",
        ):
            mov.get_amount()

        # Add a snapshot AFTER the dividend ex-date
        SnapshotNumerable.objects.create(
            account=self.account1, date_value=date(1910, 1, 2), quantity=10.5, unit_value=2.2
        )
        with self.assertRaisesMessage(
            ObjectDoesNotExist,
            "Associated account doesn't have any Snapshot previous to the dividend ex_date",
        ):
            mov.get_amount()

        # Now a snapshot before the ex-date
        SnapshotNumerable.objects.create(
            account=self.account1, date_value=date(1910, 1, 1), quantity=10.5, unit_value=2.2
        )
        self.assertEqual(mov.get_amount(), 23.10)

    def test_validate_unit_value(self):
        mov = MovementDividend(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1910, 1, 1),
            ex_dividend_date=date(1910, 1, 1),
            unit_value=-2,
        )
        with self.assertRaises(ValidationError) as cm:
            mov.full_clean()

        self.assertListEqual(
            cm.exception.messages, ["Unit value should be equal or greater than 0"]
        )

    def test_validate_ex_date(self):
        # ex-date inside [account.open, account.close]
        account = Account.objects.create(
            name="acc2",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
            close=date(1901, 1, 1),
            ccy="EUR",
        )

        m = MovementDividend(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=account,
            date_value=date(1900, 1, 1),
            ex_dividend_date=date(1899, 1, 1),
            unit_value=1,
        )

        with self.assertRaises(ValidationError) as cm:
            m.full_clean()

        self.assertListEqual(
            cm.exception.messages,
            ["MovementDividend ex_dividend_date should be after Account open"],
        )

        m = MovementDividend(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=account,
            date_value=date(1900, 1, 1),
            ex_dividend_date=date(1902, 1, 1),
            unit_value=1,
        )

        with self.assertRaises(ValidationError) as cm:
            m.full_clean()

        self.assertListEqual(
            cm.exception.messages,
            ["MovementDividend ex_dividend_date cannot be after account is closed"],
        )

        # ex-dividen-date before date_value
        m = MovementDividend(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1900, 1, 1),
            ex_dividend_date=date(1902, 1, 1),
            unit_value=1,
        )

        with self.assertRaises(ValidationError) as cm:
            m.full_clean()

        self.assertListEqual(
            cm.exception.messages,
            ["MovementDividend ex_dividend_date should be before date_value"],
        )
