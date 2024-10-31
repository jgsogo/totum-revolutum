from datetime import date

from django.core.exceptions import ObjectDoesNotExist, ValidationError
from django.test import TestCase
from django_finances_accounts.models import (
    Account,
    AccountType,
    Custodian,
    Fx,
    MovementType,
    Transaction,
)
from django_finances_accounts.models.movement import Direction
from django_finances_investments.models import MovementDividend, SnapshotNumerable


class MovementDividendTestCase(TestCase):
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

    def test_local_amount(self):
        account_usd = Account.objects.create(
            name="acc1",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
            ccy="USD",
        )
        SnapshotNumerable.objects.create(
            account=account_usd, date_value=date(1910, 1, 1), quantity=100, unit_value=1
        )

        fx = Fx.objects.create(foreign="USD", date_value=date(1900, 12, 31), rate=2)
        mov = MovementDividend.objects.create(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=account_usd,
            date_value=date(1900, 1, 1),
            ex_dividend_date=date(1910, 1, 1),
            unit_value=2,
            fx=fx,
        )

        self.assertEqual(mov.get_amount(), 200)
        self.assertEqual(mov.local_amount(), 100)

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
