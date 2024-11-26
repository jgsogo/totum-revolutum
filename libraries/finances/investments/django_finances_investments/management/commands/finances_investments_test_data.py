from django.core.management.base import BaseCommand  # , CommandError
from django.utils.translation import gettext_lazy as _
from django_finances_accounts.models import (
    Account,
    AccountType,
    Custodian,
    MovementType,
    Transaction,
)
from django_finances_investments.constants import (
    AccountTypeConstants,
    MovementTypeConstants,
)
from django_finances_investments.models import MovementNumerable


class Command(BaseCommand):
    help = "Populates the database with some data (only use for testing!)"

    def add_arguments(self, parser):
        pass

    def handle(self, *args, **options):
        accounts = self.populate_accounts()
        transactions = Transaction.objects.all()
        self.populate_movements_numerable(account=accounts[0], transaction=transactions[0])
        self.populate_movements_numerable(account=accounts[2], transaction=transactions[0])

    def populate_accounts(self):
        activos_corrientes_inversion = AccountType.objects.get(
            unique_name=AccountTypeConstants.ASSETS_CURRENT_INVESTMENT
        )
        stocks = AccountType.objects.get(name=_("Acciones"), tn_parent=activos_corrientes_inversion)
        funds = AccountType.objects.get(
            name=_("Fondo de inversión"), tn_parent=activos_corrientes_inversion
        )
        plan_pensiones = AccountType.objects.get(
            unique_name=AccountTypeConstants.ASSETS_NON_CURRENT_RETIREMENTPLAN
        )
        retirement_plan = AccountType.objects.get(name=_("Individual"), tn_parent=plan_pensiones)

        custodians = Custodian.objects.all()

        return Account.objects.bulk_create(
            [
                Account(
                    pk=4,
                    name="IBM",
                    identifier="2345",
                    ccy="USD",
                    open="2024-09-07",
                    type=stocks,
                    custodian=custodians[1],
                    is_numerable=True,
                ),
                Account(
                    pk=5,
                    name="Netflix",
                    identifier="345 - closed",
                    ccy="USD",
                    open="2024-09-08",
                    close="2024-10-08",
                    type=stocks,
                    custodian=custodians[2],
                    is_numerable=True,
                ),
                Account(
                    pk=6,
                    name="Indexa Capital",
                    identifier="fondo de inversión 123",
                    ccy="EUR",
                    open="2024-09-09",
                    type=funds,
                    custodian=custodians[2],
                    is_numerable=False,
                ),
                Account(
                    pk=7,
                    name="Plan de pensiones",
                    identifier="plan pensiones 123",
                    ccy="EUR",
                    open="2024-09-10",
                    type=retirement_plan,
                    custodian=custodians[0],
                    is_numerable=False,
                ),
            ]
        )

    def populate_movements_numerable(self, account: Account, transaction: Transaction):
        expense = MovementType.objects.get(unique_name=MovementTypeConstants.EXPENSE)

        for i in range(3):
            mov = MovementNumerable(
                amount=(i * 100),
                direction=0,
                date_value="2024-09-06",
                account=account,
                transaction=transaction,
                type=expense,
                quantity=i,
                unit_value=100,
            )
            mov.save()
