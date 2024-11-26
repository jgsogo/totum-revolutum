from django.core.management.base import BaseCommand  # , CommandError
from django.utils.translation import gettext_lazy as _
from django_finances_accounts.constants import (
    AccountTypeConstants,
    MovementTypeConstants,
)
from django_finances_accounts.models import (
    Account,
    AccountType,
    Custodian,
    Fx,
    Movement,
    MovementType,
    Snapshot,
    Transaction,
)


class Command(BaseCommand):
    help = "Populates the database with some data (only use for testing!)"

    def add_arguments(self, parser):
        pass

    def handle(self, *args, **options):
        custodians = self.populate_custodians()
        accounts = self.populate_accounts(custodians=custodians)
        self.populate_snapshots(account=accounts[0])
        transactions = self.populate_transactions()
        self.populate_movements(account=accounts[0], transactions=transactions)

    def populate_custodians(self):
        return Custodian.objects.bulk_create(
            [
                Custodian(pk=0, name="custodian0", country="es"),
                Custodian(pk=1, name="custodian1", country="us"),
                Custodian(pk=2, name="custodian2", country="nl"),
            ]
        )

    def populate_accounts(self, custodians: list):
        ahorro = AccountType.objects.get(unique_name=AccountTypeConstants.ASSETS_CURRENT_SAVINGS)
        depo = AccountType.objects.get(name=_("Depósito"), tn_parent=ahorro)
        bank_account = AccountType.objects.get(name=_("Cuenta bancaria"), tn_parent=ahorro)
        hipoteca = AccountType.objects.get(name=_("Hipoteca"))

        return Account.objects.bulk_create(
            [
                Account(
                    pk=0,
                    name="Gastos compartidos",
                    identifier="1234",
                    ccy="EUR",
                    open="2024-09-06",
                    type=bank_account,
                    custodian=custodians[0],
                    is_numerable=False,
                ),
                Account(
                    pk=1,
                    name="Depósito 3M",
                    identifier="1234",
                    ccy="EUR",
                    open="2024-09-07",
                    close="9999-01-01",
                    type=depo,
                    custodian=custodians[1],
                    is_numerable=False,
                ),
                Account(
                    pk=2,
                    name="Hipoteca casa NY",
                    identifier="1234",
                    ccy="EUR",
                    open="2024-09-08",
                    type=hipoteca,
                    custodian=custodians[2],
                    is_numerable=False,
                ),
                Account(
                    pk=3,
                    name="Old account",
                    identifier="oldies",
                    ccy="EUR",
                    open="2024-09-06",
                    close="2024-10-01",
                    type=bank_account,
                    custodian=custodians[0],
                    is_numerable=False,
                ),
            ]
        )

    def populate_snapshots(self, account: Account):
        return Snapshot.objects.bulk_create(
            [
                Snapshot(amount=0, date_value="2024-09-06", account=account),
                Snapshot(amount=1, date_value="2024-09-30", account=account),
            ]
        )

    def populate_transactions(self):
        return Transaction.objects.bulk_create(
            [
                Transaction(pk=0, name="transaction0"),
                Transaction(pk=1, name="transaction1"),
                Transaction(pk=2, name="transaction2"),
            ]
        )

    def populate_fxs(self, start_id: int):
        return Fx.objects.bulk_create(
            [
                Fx(pk=start_id, foreign="USD", local="EUR", rate=1, date_value="2024-09-06"),
                Fx(pk=start_id + 1, foreign="USD", local="EUR", rate=2, date_value="2024-09-07"),
                Fx(pk=start_id + 2, foreign="USD", local="EUR", rate=3, date_value="2024-09-08"),
                Fx(pk=start_id + 3, foreign="USD", local="EUR", rate=4, date_value="2024-09-09"),
            ]
        )

    def populate_movements(self, account: Account, transactions: list):
        fxs = self.populate_fxs(start_id=account.pk * 10)

        expense = MovementType.objects.get(unique_name=MovementTypeConstants.EXPENSE)

        Movement.objects.bulk_create(
            [
                Movement(
                    amount=0,
                    direction=0,
                    date_value="2024-09-06",
                    account=account,
                    fx=fxs[0],
                    transaction=transactions[0],
                    type=expense,
                ),
                Movement(
                    amount=0,
                    direction=0,
                    date_value="2024-09-07",
                    account=account,
                    fx=fxs[1],
                    transaction=transactions[0],
                    type=expense,
                ),
            ]
        )
