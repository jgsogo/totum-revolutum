import logging
from typing import Dict

from django.core.management.base import BaseCommand  # , CommandError
from django_finances_accounts.models import (
    Account,
    AccountHolder,
    Custodian,
    Fx,
    Movement,
    MovementType,
    Snapshot,
    Transaction,
)
from django_finances_investments.models import (
    MovementDividend,
    MovementNumerable,
    SnapshotNumerable,
)
from sqlalchemy import create_engine
from sqlalchemy.sql import text
from tqdm import tqdm

from .check_account_types import get_account_types_mapping
from .check_movement_types import get_movement_types_mapping

log = logging.getLogger(__name__)


class Command(BaseCommand):
    help = "Migrates legacy DB data"

    def add_arguments(self, parser):
        parser.add_argument("--DATABASE_URL", type=str)

    def handle(self, *args, **options):
        database_url = options["DATABASE_URL"]
        engine = create_engine(f"postgresql+psycopg:{database_url}")

        with engine.connect() as c:
            account_holders: Dict[int, AccountHolder] = self.migrate_accountholders(c)
            accounts: Dict[int, Account] = self.migrate_account(c, account_holders=account_holders)
            fxs: Dict[int, Fx] = self.migrate_fx(c)
            self.migrate_snapshots(c, accounts=accounts)
            transfers = self.migrate_transfers(c)
            self.migrate_movements(c, accounts=accounts, transfers=transfers, fxs=fxs)

    def migrate_accountholders(self, conn):
        total = conn.execute(text("SELECT COUNT(*) FROM data_accountholder")).scalar()
        statement = text("SELECT * FROM data_accountholder")
        results = conn.execute(statement)

        mapping = {}
        for pk, name, owner in tqdm(results, total=total, desc="data/accountholder"):
            owner = "ME" if owner == 0 else "OUT"
            log.debug(f"\t{name} - {owner}")

            account_holder, created = AccountHolder.objects.get_or_create(
                name=name, defaults={"is_company": owner == 0}  # TODO: no way to know this
            )
            mapping[pk] = account_holder

        return mapping

    def migrate_account(self, conn, account_holders):
        account_types = get_account_types_mapping()

        total = conn.execute(text("SELECT COUNT(*) FROM data_account")).scalar()

        statement = text(
            """
            SELECT
                data_account.id,
                data_accounttype.name,
                data_account.holder_id,
                data_account.identifier,
                data_account.name,
                data_account.is_numerable,
                data_account.ccy,
                data_account.open,
                data_account.close
            FROM
                data_account
            LEFT OUTER JOIN
                data_accounttype
            ON
                data_account.type_id=data_accounttype.id
        """
        )
        results = conn.execute(statement)

        fake_custodian, _ = Custodian.objects.get_or_create(name="fake", defaults={"country": "es"})
        mapping = {}
        for account in tqdm(results, total=total, desc="data/accounttype"):
            (
                pk,
                acc_type_name,
                acc_holder,
                acc_identifier,
                acc_name,
                is_numerable,
                ccy,
                acc_open,
                acc_close,
            ) = account
            acc_type = account_types[acc_type_name]
            acc_holder = account_holders[acc_holder]

            account, created = Account.objects.get_or_create(
                name=acc_name,
                defaults={
                    "description": None,
                    "custodian": fake_custodian,
                    "type": acc_type,
                    "identifier": acc_identifier,
                    "ccy": ccy,
                    "open": acc_open,
                    "close": acc_close,
                    "is_numerable": is_numerable,
                },
            )
            account.holders.set([acc_holder])

            mapping[pk] = account
        return mapping

    def migrate_fx(self, conn):
        total = conn.execute(text("SELECT COUNT(*) FROM data_fx")).scalar()

        statement = text("SELECT id, data_fx.foreign, local, rate, date_value FROM data_fx")
        results = conn.execute(statement)

        mapping = {}
        for fx in tqdm(results, total=total, desc="data/data_fx"):
            pk, foreign, local, rate, date_value = fx
            # Note.- This is deduplicating Fx
            fx, _ = Fx.objects.get_or_create(
                foreign=foreign,
                local=local,
                rate=rate,
                date_value=date_value,
            )
            mapping[pk] = fx
        return mapping

    def migrate_snapshots(self, conn, accounts: Dict[int, Account]):
        total = conn.execute(text("SELECT COUNT(*) FROM data_snapshot")).scalar()

        statement = text(
            "SELECT id, account_id, date_value, amount, quantity, unit_value FROM data_snapshot"
        )
        results = conn.execute(statement)

        for snapshot in tqdm(results, total=total, desc="data/snapshots"):
            pk, account, date_value, amount, quantity, unit_value = snapshot
            account = accounts[account]

            if account.is_numerable:
                SnapshotNumerable.objects.get_or_create(
                    account=account,
                    date_value=date_value,
                    defaults={
                        "quantity": quantity,
                        "unit_value": unit_value,
                    },
                )
            else:
                Snapshot.objects.get_or_create(
                    account=account,
                    date_value=date_value,
                    defaults={
                        "amount": amount,
                    },
                )

    def migrate_transfers(self, conn):
        total = conn.execute(text("SELECT COUNT(*) FROM data_transfer")).scalar()

        statement = text("SELECT id, description FROM data_transfer")
        results = conn.execute(statement)

        mapping = {}
        for transfer in tqdm(results, total=total, desc="data/transfers"):
            pk, description = transfer
            name = description.splitlines()[0][:80]
            transaction, _1 = Transaction.objects.get_or_create(
                name=name,
                defaults={
                    "description": description,
                },
            )
            mapping[pk] = transaction
        return mapping

    def migrate_movements(self, conn, accounts, transfers, fxs):
        Movement.objects.all().delete()  # FIXME: Make this an option
        MovementDividend.objects.all().delete()  # FIXME: Make this an option
        MovementNumerable.objects.all().delete()  # FIXME: Make this an option

        total = conn.execute(text("SELECT COUNT(*) FROM data_movement")).scalar()

        statement = text(
            """
            SELECT
                data_movement.id,
                data_movement.amount,
                data_movement.quantity,
                data_movement.unit_value,
                data_movement.account_id,
                data_movement.date_value,
                data_movement.transfer_id,
                data_movementtype.name,
                data_movement.fx_id,
                data_movement.direction,
                data_movement.date
            FROM
                data_movement
            LEFT OUTER JOIN
                data_movementtype
            ON
                data_movement.type_id=data_movementtype.id
        """
        )
        results = conn.execute(statement)

        movtypes = get_movement_types_mapping()
        for movement in tqdm(results, total=total, desc="data/movements"):
            (
                pk,
                amount,
                quantity,
                unit_value,
                account,
                date_value,
                transfer,
                mov_type_name,
                fx,
                direction,
                date,
            ) = movement
            account = accounts[account]
            transfer = transfers[transfer]
            mov_type = movtypes[mov_type_name]
            if fx:
                fx = fxs[fx]

            uniqueness = {
                "account": account,
                "transaction": transfer,
                "type": mov_type,
                "direction": direction,
                "date_value": date_value,
                # "amount": amount,
            }

            dividendos_mov_type = MovementType.objects.get(name="Dividendos")
            if mov_type == dividendos_mov_type:
                # FIXME: We don't have information about 'ex_dividend_date' and 'unit_value', so
                # we are creating this as regular movements and, afterwards, I can go through all
                # them and add this information
                _1, created = Movement.objects.get_or_create(
                    **uniqueness, amount=amount, defaults={"fx": fx}
                )
                # assert not created, f"Movement[Divident] created: {uniqueness}"
                assert created, f"Movement[Divident] not created: {uniqueness}"
            elif account.is_numerable:
                _1, created = MovementNumerable.objects.get_or_create(
                    **uniqueness, unit_value=unit_value, quantity=quantity, defaults={"fx": fx}
                )
                # assert not created, (f"MovementNumerable created: {uniqueness},"
                # f"unit_value: {unit_value}, quantity = {quantity}")
                assert created, (
                    f"MovementNumerable not created: {uniqueness},"
                    f" unit_value: {unit_value}, quantity = {quantity}"
                )
            else:
                _1, created = Movement.objects.get_or_create(
                    **uniqueness, amount=amount, defaults={"fx": fx}
                )
                # assert not created, f"Movement created: {uniqueness}"
                assert created, f"Movement not created: {uniqueness}"
