from datetime import date

from django.core.exceptions import ObjectDoesNotExist
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


class MovementNonNumerableTestCase(BaseMovementTestCase):
    def test_get_amount(self):
        movtype = MovementType.objects.create(name="movtype")
        mov = MovementNonNumerable(
            transaction=self.transaction,
            type=movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1900, 1, 1),
            amount=1000.50,
        )

        self.assertEqual(mov.get_amount(), 1000.50)


class MovementNumerableTestCase(BaseMovementTestCase):
    def test_get_amount(self):
        movtype = MovementType.objects.create(name="movtype")
        mov = MovementNumerable(
            transaction=self.transaction,
            type=movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1900, 1, 1),
            quantity=10.5,
            unit_value=2.2,
        )

        self.assertEqual(mov.get_amount(), 23.10)


class MovementDividendTestCase(BaseMovementTestCase):
    def test_get_amount(self):
        movtype = MovementType.objects.create(name="movtype")
        mov = MovementDividend(
            transaction=self.transaction,
            type=movtype,
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
