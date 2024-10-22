from datetime import date

from django.core.exceptions import ObjectDoesNotExist, ValidationError
from django.test import TestCase
from finances_data.models import (
    Account,
    AccountType,
    Custodian,
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
        acctype = AccountType.objects.create(name="acctype")

        custodian = Custodian.objects.create(name="custodian", country="ES")
        self.account1 = Account.objects.create(
            name="acc1", custodian=custodian, type=acctype, open=date(1900, 1, 1), ccy="EUR"
        )
        self.account2 = Account.objects.create(
            name="acc2", custodian=custodian, type=acctype, open=date(1900, 1, 1), ccy="EUR"
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
        movtype = MovementType.objects.create(name="movtype")
        mov1 = MovementNonNumerable.objects.create(
            transaction=self.transaction,
            type=movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1900, 1, 1),
            amount=1000.50,
        )
        mov2 = MovementNonNumerable.objects.create(
            transaction=self.transaction,
            type=movtype,
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
        # TODO:
        pass

    def test_type_protect(self):
        # TODO:
        pass

    def test_account_protect(self):
        # TODO:
        pass

    def test_fx_cascade(self):
        # TODO:
        pass

    def test_validation_fx(self):
        # TODO:  1) FX date_value has to match movement date
        # TODO:  2) FX foreign has to match account.ccy
        # TODO:  3) date_value inside [account.open, account.close] dates
        pass


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
            date_value=date(1900, 1, 1),
            ex_dividend_date=date(1910, 1, 1),
            unit_value=-2,
        )
        with self.assertRaises(ValidationError) as cm:
            mov.full_clean()

        self.assertListEqual(
            cm.exception.messages, ["Unit value should be equal or greater than 0"]
        )

    def test_validate_ex_date(self):
        # TODO: 3) ex-date inside [account.open, account.close]
        pass
