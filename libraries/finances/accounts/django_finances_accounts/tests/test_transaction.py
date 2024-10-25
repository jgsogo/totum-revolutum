from datetime import date

from django.core.exceptions import ValidationError
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
from django_finances_accounts.models.transaction import validate_movements


class TransactionTestCase(TestCase):
    def setUp(self):

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

    def test_movements_mismatch(self):
        t = Transaction.objects.create(name="transaction")
        MovementNonNumerable.objects.create(
            transaction=t,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1999, 1, 1),
            amount=1000,
        )
        MovementNonNumerable.objects.create(
            transaction=t,
            type=self.movtype,
            direction=Direction.IN,
            account=self.account2,
            date_value=date(1999, 1, 1),
            amount=500,
        )

        with self.assertRaises(ValidationError) as cm:
            t.full_clean()

        self.assertListEqual(
            cm.exception.messages,
            ["INs (500.00) has to be equal to OUTs (1000.00) with tolerance '0'"],
        )

    def test_movements_xccy(self):
        t = Transaction.objects.create(name="transaction")
        account_USD = Account.objects.create(
            name="acc1",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
            ccy="USD",
        )

        MovementNonNumerable.objects.create(
            transaction=t,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1999, 1, 1),
            amount=1000,
        )
        fx = Fx.objects.create(foreign="USD", date_value=date(1999, 1, 1), rate=2)
        MovementNonNumerable.objects.create(
            transaction=t,
            type=self.movtype,
            direction=Direction.IN,
            account=account_USD,
            date_value=date(1999, 1, 1),
            amount=500,
            fx=fx,
        ).full_clean()

        with self.assertRaises(ValidationError) as cm:
            t.full_clean()

        self.assertListEqual(
            cm.exception.messages,
            ["INs (250.000) has to be equal to OUTs (1000.00) with tolerance '0'"],
        )

    def test_validate_movements_tolerance(self):
        t = Transaction.objects.create(name="transaction")
        account_USD = Account.objects.create(
            name="acc1",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
            ccy="USD",
        )

        m1 = MovementNonNumerable.objects.create(
            transaction=t,
            type=self.movtype,
            direction=Direction.OUT,
            account=self.account1,
            date_value=date(1999, 1, 1),
            amount=1000,
        )
        fx = Fx.objects.create(foreign="USD", date_value=date(1999, 1, 1), rate=0.5)
        m2 = MovementNonNumerable.objects.create(
            transaction=t,
            type=self.movtype,
            direction=Direction.IN,
            account=account_USD,
            date_value=date(1999, 1, 1),
            amount=499,
            fx=fx,
        )

        with self.assertRaises(ValidationError) as cm:
            validate_movements([m1, m2], money_tolerance=0)

        self.assertListEqual(
            cm.exception.messages,
            ["INs (998.0) has to be equal to OUTs (1000) with tolerance '0'"],
        )

        # If I increase the tolerance, it validates
        validate_movements([m1, m2], money_tolerance=2)
