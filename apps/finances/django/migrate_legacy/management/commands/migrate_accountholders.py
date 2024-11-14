from django.core.management.base import BaseCommand  # , CommandError

# from polls.models import Question as Poll
from sqlalchemy import create_engine

# from sqlalchemy.orm import Session
from sqlalchemy.sql import text


class Command(BaseCommand):
    help = "Migrates AccountHolder instances"

    def add_arguments(self, parser):
        parser.add_argument("--DATABASE_URL", type=str)

    def handle(self, *args, **options):
        database_url = options["DATABASE_URL"]
        engine = create_engine(f"postgresql+psycopg:{database_url}")
        # session = Session(engine)

        with engine.connect() as c:
            statement = text("SELECT * FROM data_accountholder")
            results = c.execute(statement)
            for row in results:
                self.stdout.write(str(row))

        # rs = session.query(statement)
        # for row in rs:
        #     self.stdout.write(row)


#  public | data_account                  | table | finances
#  public | data_accountholder            | table | finances
#  public | data_accounttype              | table | finances
#  public | data_fx                       | table | finances
#  public | data_movement                 | table | finances
#  public | data_movementtype             | table | finances
#  public | data_snapshot                 | table | finances
#  public | data_transfer                 | table | finances
