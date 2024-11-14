from typing import Dict

from django.core.management.base import BaseCommand  # , CommandError
from django_finances_accounts.models import AccountHolder
from sqlalchemy import create_engine
from sqlalchemy.sql import text
from tqdm import tqdm


class Command(BaseCommand):
    help = "Migrates AccountHolder instances"

    def add_arguments(self, parser):
        parser.add_argument("--DATABASE_URL", type=str)

    def handle(self, *args, **options):
        database_url = options["DATABASE_URL"]
        engine = create_engine(f"postgresql+psycopg:{database_url}")

        with engine.connect() as c:
            account_holders: Dict[int, AccountHolder] = self.migrate_accountholders(c)
            assert account_holders

    def migrate_accountholders(self, conn):
        tqdm.file = self.stdout

        total = conn.execute(text("SELECT COUNT(*) FROM data_accountholder")).scalar()
        tqdm.write("Migrate AccountHolders")

        statement = text("SELECT * FROM data_accountholder")
        results = conn.execute(statement)

        mapping = {}
        total_created = 0
        for pk, name, owner in tqdm(results, total=total, desc="data/accountholder"):
            owner = "ME" if owner == 0 else "OUT"
            tqdm.write(f"{name} - {owner}")

            account_holder, created = AccountHolder.objects.get_or_create(
                name=name, defaults={"is_company": owner == 0}
            )  # TODO: is_company random?
            mapping[pk] = account_holder
            if created:
                total_created += 1

        tqdm.write(f"Total: {total}, Created: {total_created}")
        return mapping


#  public | data_account                  | table | finances
#  public | data_accountholder            | table | finances
#  public | data_accounttype              | table | finances
#  public | data_fx                       | table | finances
#  public | data_movement                 | table | finances
#  public | data_movementtype             | table | finances
#  public | data_snapshot                 | table | finances
#  public | data_transfer                 | table | finances
