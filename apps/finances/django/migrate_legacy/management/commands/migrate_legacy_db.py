from itertools import groupby
from typing import Dict

from django.core.management.base import BaseCommand, CommandError
from django_finances_accounts.models import (
    Account,
    AccountHolder,
    AccountHolderRole,
    Custodian,
    Fx,
    Movement,
    MovementType,
    Snapshot,
    Transaction,
    TransactionGroup,
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


class Command(BaseCommand):
    help = "Migrates legacy DB data"

    def add_arguments(self, parser):
        parser.add_argument("--LEGACY_DATABASE_URL", type=str)
        parser.add_argument(
            "--obliterate",
            action="store_true",
            help="Remove all existing data before running",
        )

    def handle(self, *args, **options):
        obliterate = options["obliterate"]
        if obliterate:
            self.stdout.write("Remove everything before migration")
            self.stdout.write(" - Movement")
            Movement.objects.all().delete()
            self.stdout.write(" - MovementDividend")
            MovementDividend.objects.all().delete()
            self.stdout.write(" - MovementNumerable")
            MovementNumerable.objects.all().delete()
            self.stdout.write(" - Transaction & TransactionGroup")
            Transaction.objects.all().delete()
            TransactionGroup.objects.all().delete()
            self.stdout.write(" - Snapshot & SnapshotNumerable")
            Snapshot.objects.all().delete()
            SnapshotNumerable.objects.all().delete()
            self.stdout.write(" - Fx")
            Fx.objects.all().delete()
            self.stdout.write(" - Account")
            Account.objects.all().delete()
            self.stdout.write(" - AccountHolder")
            AccountHolder.objects.all().delete()
            self.stdout.write(" - Custodian")
            Custodian.objects.all().delete()

        database_url = options["LEGACY_DATABASE_URL"]
        database_url = database_url.lstrip("postgres:")
        engine = create_engine(f"postgresql+psycopg:{database_url}")

        with engine.connect() as c:
            fxs: Dict[int, Fx] = self.migrate_fx(c)

            account_holders: Dict[int, AccountHolder] = self.migrate_accountholders(
                c, obliterated=obliterate
            )
            accounts: Dict[int, Account] = self.migrate_account(
                c, account_holders=account_holders, obliterated=obliterate
            )
            self.migrate_snapshots(c, accounts=accounts, obliterated=obliterate)
            self.migrate_movements(c, accounts=accounts, fxs=fxs, obliterated=obliterate)

    def migrate_accountholders(self, conn, obliterated: bool = False):
        total = conn.execute(text("SELECT COUNT(*) FROM data_accountholder")).scalar()
        statement = text("SELECT id,name,owner FROM data_accountholder")
        results = conn.execute(statement)

        mapping = {}
        for pk, name, owner in tqdm(results, total=total, desc="data/accountholder"):
            owner = "ME" if owner == 0 else "OUT"
            account_holder, created = AccountHolder.objects.get_or_create(
                name=name, defaults={"is_company": owner == "ME"}  # TODO: no way to know this
            )
            if obliterated and not created:
                self.stderr.write(f"AccountHolder '{account_holder}' was not created!")
            if created and not obliterated:
                self.stdout.write(f"AccountHolder '{account_holder}' was created!")
            mapping[pk] = account_holder

        return mapping

    def migrate_account(self, conn, account_holders, obliterated: bool = False):
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
        for account in tqdm(results, total=total, desc="data/account"):
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

            try:
                # Try to get
                acc_holder_role = AccountHolderRole.objects.get(
                    account__name=acc_name, holder=acc_holder
                )
                account = acc_holder_role.account
                created = False
            except AccountHolderRole.DoesNotExist:
                # Create otherwise
                account = Account.objects.create(
                    name=acc_name,
                    description=None,
                    custodian=fake_custodian,
                    type=acc_type,
                    identifier=acc_identifier,
                    ccy=ccy,
                    open=acc_open,
                    close=acc_close,
                    is_numerable=is_numerable,
                )
                account.holders.set([acc_holder])
                created = True

            if obliterated and not created:
                self.stderr.write(f"Account '{account}' was not created!")
            if created and not obliterated:
                self.stdout.write(f"Account '{account}' was created!")

            mapping[pk] = account
        return mapping

    def migrate_fx(self, conn):
        total = conn.execute(text("SELECT COUNT(*) FROM data_fx")).scalar()

        statement = text("SELECT id, data_fx.foreign, local, rate, date_value FROM data_fx")
        results = conn.execute(statement)

        mapping = {}
        for fx in tqdm(results, total=total, desc="data/data_fx"):
            pk, foreign, local, rate, date_value = fx
            # Note.- This is deduplicating Fx, I don't really care about obliterated or not
            fx, _ = Fx.objects.get_or_create(
                foreign=foreign,
                local=local,
                rate=rate,
                date_value=date_value,
            )
            mapping[pk] = fx
        return mapping

    def migrate_snapshots(self, conn, accounts: Dict[int, Account], obliterated: bool = False):
        total = conn.execute(text("SELECT COUNT(*) FROM data_snapshot")).scalar()

        statement = text(
            "SELECT id, account_id, date_value, amount, quantity, unit_value FROM data_snapshot"
        )
        results = conn.execute(statement)

        for snapshot in tqdm(results, total=total, desc="data/snapshots"):
            pk, account, date_value, amount, quantity, unit_value = snapshot
            account = accounts[account]

            if account.is_numerable:
                snapshot, created = SnapshotNumerable.objects.get_or_create(
                    account=account,
                    date_value=date_value,
                    defaults={
                        "quantity": quantity,
                        "unit_value": unit_value,
                    },
                )
            else:
                snapshot, created = Snapshot.objects.get_or_create(
                    account=account,
                    date_value=date_value,
                    defaults={
                        "amount": amount,
                    },
                )

            if obliterated and not created:
                self.stderr.write(f"Snapshot[Numerable] '{snapshot}' was not created!")
            if created and not obliterated:
                self.stdout.write(f"Snapshot[Numerable] '{snapshot}' was created!")

    def _create_one_transaction(self, conn, description):
        name = description.splitlines()[0][:80]
        transaction = Transaction.objects.create(name=name, description=description)
        return transaction

    def _create_transactions(self, conn):
        statement = text("SELECT id, description FROM data_transfer")
        results = conn.execute(statement)

        for transfer in results:
            pk, description = transfer
            transaction = self._create_one_transaction(conn, description)
            yield (transaction, pk)

    def migrate_movements(self, conn, accounts, fxs, obliterated: bool = False):
        total_movs = conn.execute(text("SELECT COUNT(*) FROM data_movement")).scalar()
        total_transfers = conn.execute(text("SELECT COUNT(*) FROM data_transfer")).scalar()
        movtypes = get_movement_types_mapping()

        pbar_transfers = tqdm(total=total_transfers, desc="data/transfers")
        pbar_movs = tqdm(total=total_movs, desc="data/movements")

        if obliterated:
            # Iterate all the transactions creating them
            for transaction, transfer_pk in self._create_transactions(conn):
                pbar_transfers.update(1)

                # Iterate all the movements for each transaction, and created them
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
                        WHERE
                            data_movement.transfer_id={transfer_pk}
                    """.format(
                        transfer_pk=transfer_pk
                    )
                )
                results = conn.execute(statement)
                for movement in results:
                    (
                        _1,
                        amount,
                        quantity,
                        unit_value,
                        account,
                        date_value,
                        transfer_id,
                        mov_type_name,
                        fx,
                        direction,
                        date,
                    ) = movement
                    assert transfer_id == transfer_pk
                    account = accounts[account]
                    mov_type = movtypes[mov_type_name]
                    if fx:
                        fx = fxs[fx]
                    self.create_movement(
                        conn=conn,
                        amount=amount,
                        quantity=quantity,
                        unit_value=unit_value,
                        account=account,
                        date_value=date_value,
                        transaction=transaction,
                        mov_type=mov_type,
                        fx=fx,
                        direction=direction,
                        date=date,
                    )

                    pbar_movs.update(1)
        else:
            # Iterate all the movements. If one is created, then create the transaction and
            # the other movements in it
            statement = text(
                """
                    SELECT
                        data_movement.id,
                        data_movement.amount,
                        data_movement.quantity,
                        data_movement.unit_value,
                        data_movement.account_id,
                        data_movement.date_value,
                        data_transfer.id,
                        data_transfer.description,
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

                    LEFT OUTER JOIN
                        data_transfer
                    ON
                        data_movement.transfer_id=data_transfer.id

                    ORDER BY
                        data_transfer.id
                """
            )
            results = conn.execute(statement)

            for transfer_id, movements in groupby(results, key=lambda data: data[6]):
                pbar_transfers.update(1)
                movements = list(movements)

                movs_created_or_not = []
                for movement in movements:
                    (
                        _1,
                        amount,
                        quantity,
                        unit_value,
                        account,
                        date_value,
                        _2,
                        transfer_description,
                        mov_type_name,
                        fx,
                        direction,
                        date,
                    ) = movement
                    account = accounts[account]
                    mov_type = movtypes[mov_type_name]
                    if fx:
                        fx = fxs[fx]
                    mov = self._movement_exists(
                        conn=conn,
                        amount=amount,
                        quantity=quantity,
                        unit_value=unit_value,
                        account=account,
                        date_value=date_value,
                        transaction_description=transfer_description,
                        mov_type=mov_type,
                        fx=fx,
                        direction=direction,
                        date=date,
                    )
                    movs_created_or_not.append(mov)

                if len(set(movs_created_or_not)) != 1:
                    for exists, mov in zip(movs_created_or_not, movements):
                        self.stderr.write(f"{exists} - movement {mov[1]}")
                    raise CommandError(
                        f"Error with transfer_id {transfer_id} ('{transfer_description.strip()}'),"
                        f" some movements ({movs_created_or_not}) exists and others doesn't!"
                    )

                # If all movements exist, then skip this transfer
                if all(movs_created_or_not):
                    pbar_movs.update(len(movements))
                    continue

                # If all NOT exist, create the transaction and the movements
                transaction = self._create_one_transaction(conn, transfer_description)

                for movement in movements:
                    (
                        _1,
                        amount,
                        quantity,
                        unit_value,
                        account,
                        date_value,
                        _2,
                        _3,
                        mov_type_name,
                        fx,
                        direction,
                        date,
                    ) = movement
                    account = accounts[account]
                    mov_type = movtypes[mov_type_name]
                    if fx:
                        fx = fxs[fx]

                    movement, created = self._get_or_create_movement(
                        conn=conn,
                        amount=amount,
                        quantity=quantity,
                        unit_value=unit_value,
                        account=account,
                        date_value=date_value,
                        transaction=transaction,
                        mov_type=mov_type,
                        fx=fx,
                        direction=direction,
                        date=date,
                    )

                    if not created:
                        self.stderr.write(
                            f"Movement '{movement}' (transfer_id: {transfer_id}) was not created!"
                        )

                    pbar_movs.update(1)

    def _movement_exists(
        self,
        conn,
        amount,
        quantity,
        unit_value,
        account,
        date_value,
        transaction_description,
        mov_type,
        fx,
        direction,
        date,
    ):

        uniqueness = {
            "account": account,
            "transaction__description": transaction_description,
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
            return Movement.objects.filter(**uniqueness, amount=amount).exists()
        elif account.is_numerable:
            return MovementNumerable.objects.filter(
                **uniqueness, unit_value=unit_value, quantity=quantity
            ).exists()
        else:
            return Movement.objects.filter(**uniqueness, amount=amount).exists()

    def _get_or_create_movement(
        self,
        conn,
        amount,
        quantity,
        unit_value,
        account,
        date_value,
        transaction,
        mov_type,
        fx,
        direction,
        date,
    ):

        uniqueness = {
            "account": account,
            "transaction": transaction,
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
            movement, created = Movement.objects.get_or_create(
                **uniqueness, amount=amount, defaults={"fx": fx}
            )
        elif account.is_numerable:
            movement, created = MovementNumerable.objects.get_or_create(
                **uniqueness, unit_value=unit_value, quantity=quantity, defaults={"fx": fx}
            )
        else:
            movement, created = Movement.objects.get_or_create(
                **uniqueness, amount=amount, defaults={"fx": fx}
            )

        return movement, created

    def create_movement(
        self,
        conn,
        amount,
        quantity,
        unit_value,
        account,
        date_value,
        transaction,
        mov_type,
        fx,
        direction,
        date,
    ):

        uniqueness = {
            "account": account,
            "transaction": transaction,
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
            movement = Movement.objects.create(**uniqueness, amount=amount, fx=fx)
        elif account.is_numerable:
            movement = MovementNumerable.objects.create(
                **uniqueness, unit_value=unit_value, quantity=quantity, fx=fx
            )
        else:
            movement = Movement.objects.create(**uniqueness, amount=amount, fx=fx)

        return movement
