from typing import Dict

from django.core.management.base import BaseCommand  # , CommandError
from django_finances_accounts.models import Custodian, AccountType, Account, Snapshot, Transaction, Fx, MovementType, Movement
from django_finances_investments.models import MovementNumerable
from django_finances_accounts.constants import AccountTypeConstants, MovementTypeConstants

class Command(BaseCommand):
    help = "Populates the database with some data (only use for testing!)"

    def add_arguments(self, parser):
        pass

    def handle(self, *args, **options):
        account0 = Account.objects.get(pk=0)
        account1 = Account.objects.get(pk=1)
        
        self.populate_movements_numerable(account=account0)
        self.populate_movements_numerable(account=account1)

    def populate_movements_numerable(self, account: Account):
        movs = account.movement_set.all()

        for mov in movs:
            mov_numerable = MovementNumerable(movement_ptr=mov, quantity=1, unit_value=100)
            mov_numerable.save_base(raw=True)
