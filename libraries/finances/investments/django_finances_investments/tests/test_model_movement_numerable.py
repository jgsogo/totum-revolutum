from datetime import date

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
from django_finances_investments.models import MovementNumerable

from .test_model__amount_numerable import AmountNumerableTestsMixin


class MovementNumerableTestCase(AmountNumerableTestsMixin, TestCase):
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

    def test_local_amount(self):
        account_usd = Account.objects.create(
            name="acc1",
            custodian=self.custodian,
            type=self.acctype,
            open=date(1900, 1, 1),
            ccy="USD",
        )
        fx = Fx.objects.create(foreign="USD", date_value=date(1900, 12, 31), rate=2)
        mov = MovementNumerable.objects.create(
            transaction=self.transaction,
            type=self.movtype,
            direction=Direction.OUT,
            account=account_usd,
            date_value=date(1900, 12, 31),
            quantity=1000,
            unit_value=1,
            fx=fx,
        )

        self.assertEqual(mov.get_amount(), 1000)
        self.assertEqual(mov.local_amount(), 500)
