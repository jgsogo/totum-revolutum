from django.core.management.base import BaseCommand  # , CommandError
from django.utils.translation import gettext_lazy as _
from django_finances_accounts.constants import (
    AccountTypeConstants,
    MovementTypeConstants,
)
from django_finances_accounts.models import (
    Account,
    AccountHolder,
    AccountHolderRole,
    AccountType,
    Custodian,
    Fx,
    Movement,
    MovementType,
    Snapshot,
    Transaction,
    TransactionGroup,
)


class Command(BaseCommand):
    help = "Populates the database with some data (only use for testing!)"

    def add_arguments(self, parser):
        pass

    def handle(self, *args, **options):
        custodians = self.populate_custodians()
        accounts = self.populate_accounts(custodians=custodians)
        self.populate_snapshots(account=accounts[0])
        self.populate_transaction_groups()
        transactions = self.populate_transactions()
        self.populate_movements(account=accounts[0], transactions=transactions)
        holders = self.populate_holders()
        self.populate_accountholder_roles(holders=holders, accounts=accounts)

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

    def populate_transaction_groups(self):
        return TransactionGroup.objects.bulk_create(
            [
                TransactionGroup(pk=0, name="transaction_group0", start="2024-09-06"),
                TransactionGroup(
                    pk=1, name="transaction_group1", cadence="year", start="2024-09-06"
                ),
                TransactionGroup(
                    pk=2, name="transaction_group2", start="2024-09-06", end="2024-10-10"
                ),
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

    def populate_holders(self):
        return AccountHolder.objects.bulk_create(
            [
                AccountHolder(pk=0, name="holder0", is_company=False),
                AccountHolder(pk=1, name="holder1", is_company=False),
                AccountHolder(pk=2, name="holder2", is_company=True),
            ]
        )

    def populate_accountholder_roles(self, holders: list[AccountHolder], accounts: list[Account]):
        gastos_compartidos, depo, hipoteca, old_account = accounts
        holder0, holder1, holder2 = holders

        AccountHolderRole.objects.bulk_create(
            [
                # Holder0 owns money in all accounts
                AccountHolderRole(
                    holder=holder0,
                    account=gastos_compartidos,
                    owns_money=True,
                ),
                AccountHolderRole(
                    holder=holder0,
                    account=depo,
                    owns_money=True,
                ),
                AccountHolderRole(
                    holder=holder0,
                    account=hipoteca,
                    owns_money=True,
                ),
                AccountHolderRole(
                    holder=holder0,
                    account=old_account,
                    owns_money=True,
                ),
                # Holder1 owns money in some accounts
                AccountHolderRole(
                    holder=holder1,
                    account=gastos_compartidos,
                    owns_money=True,
                ),
                AccountHolderRole(
                    holder=holder1,
                    account=depo,
                    owns_money=True,
                ),
                AccountHolderRole(
                    holder=holder1,
                    account=old_account,
                    owns_money=True,
                ),
                # Holder2 plays a role (can view), but doesn't own the money
                AccountHolderRole(
                    holder=holder2,
                    account=gastos_compartidos,
                    owns_money=False,
                ),
                AccountHolderRole(
                    holder=holder2,
                    account=depo,
                    owns_money=False,
                ),
            ]
        )
