from datetime import date

from django.core.exceptions import ValidationError
from django.db.models import ProtectedError
from django.test import TestCase
from django_finances_accounts.models import (
    Account,
    AccountType,
    Custodian,
    Fx,
    MovementNonNumerable,
    MovementType,
    Transaction,
)
from django_finances_accounts.models.movement import Direction

from .test_model__amount import AmountNonNumerableTestMixin


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
        )
        self.account2 = Account.objects.create(
            name="acc2",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
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

    def test_local_amount(self):
        account_usd = Account.objects.create(
            name="acc1",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
            ccy="USD",
        )
        fx = Fx.objects.create(foreign="USD", date_value=date(1900, 12, 31), rate=2)
        mov = MovementNonNumerable.objects.create(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=account_usd,
            date_value=date(1900, 12, 31),
            amount=1000,
            fx=fx,
        )

        self.assertEqual(mov.get_amount(), 1000)
        self.assertEqual(mov.local_amount(), 500)

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
            " through protected foreign keys: 'Movement.type'.\","
            " {<Movement: Movement object (1)>})",
        )

    def test_account_protect(self):
        self._create_instance(amount=10).save()
        with self.assertRaises(ProtectedError) as cm:
            self.account1.delete()

        self.assertEqual(
            str(cm.exception),
            "(\"Cannot delete some instances of model 'Account' because they are referenced"
            " through protected foreign keys: 'Movement.account'.\","
            " {<Movement: Movement object (1)>})",
        )

    def test_fx_protect(self):
        d = date(1900, 1, 1)
        fx = Fx.objects.create(foreign="USD", date_value=d, rate=1.25)
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
            " through protected foreign keys: 'Movement.fx'.\","
            " {<Movement: Movement object (1)>})",
        )

    def test_validation_fx_date_movement(self):
        # FX date_value has to match movement date
        fx = Fx.objects.create(foreign="USD", date_value=date(1900, 1, 1), rate=1.25)
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
        fx = Fx.objects.create(foreign="USD", date_value=date(1900, 1, 1), rate=1.25)
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
            cm.exception.messages, ["FX foreign currency and Account currency didn't match"]
        )

    def test_validate_date(self):
        # Movement date_value inside [account.open, account.close] dates
        account = Account.objects.create(
            name="acc2",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
            close=date(1901, 1, 1),
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

    def test_validate_xccy_account(self):
        account_usd = Account.objects.create(
            name="acc2",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
            ccy="USD",
        )
        m = MovementNonNumerable.objects.create(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=account_usd,
            date_value=date(1900, 1, 1),
            amount=1000,
        )
        with self.assertRaises(ValidationError) as cm:
            m.full_clean()

        self.assertListEqual(
            cm.exception.messages, ["FX is required if the account is not in base currency"]
        )
