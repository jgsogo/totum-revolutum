from typing import Dict

from django.core.management.base import BaseCommand  # , CommandError
from django_finances_accounts.models import AccountType
from sqlalchemy import create_engine
from sqlalchemy.sql import text
from tqdm import tqdm


def get_account_types_mapping() -> Dict[str, AccountType]:
    to_be_removed, _1 = AccountType.objects.get_or_create(name="to-be-removed")
    legacy_name_to_account_type = {
        "Cuenta corriente": AccountType.objects.get(name="Cuenta bancaria"),
        "Plan de pensiones": AccountType.objects.get(name="Individual"),
        "Fondo de inversión": AccountType.objects.get(name="Fondo de inversión"),
        "Acciones": AccountType.objects.get(name="Acciones"),
        "Metálico": AccountType.objects.get(name="Efectivo/Metálico/Cash"),
        "Suscripciones": to_be_removed,
        "Impuestos": AccountType.objects.get(name="Flujo de caja"),
        "Online": to_be_removed,
        "Cuenta crédito": AccountType.objects.get(
            name="Tarjeta de crédito"
        ),  # i.e.: American Express
        "Depósito": AccountType.objects.get(name="Depósito"),
        "Vivienda": AccountType.objects.get(name="Vivienda"),
    }

    return legacy_name_to_account_type


class Command(BaseCommand):
    help = "Checks that the AccountType mapping contains all required entries"

    def add_arguments(self, parser):
        parser.add_argument("--LEGACY_DATABASE_URL", type=str)

    def handle(self, *args, **options):
        database_url = options["LEGACY_DATABASE_URL"]
        database_url = database_url.lstrip("postgres:")
        engine = create_engine(f"postgresql+psycopg:{database_url}")

        with engine.connect() as conn:
            self.check_accounttypes(conn)

    def check_accounttypes(self, conn):
        total = conn.execute(text("SELECT COUNT(*) FROM data_accounttype")).scalar()

        statement = text("SELECT * FROM data_accounttype")
        results = conn.execute(statement)

        logacy_name_to_account_type = get_account_types_mapping()

        for pk, name in tqdm(results, total=total, desc="check/accounttype"):
            try:
                logacy_name_to_account_type[name]
            except KeyError:
                self.stderr.write(f"Legacy AccountType '{name}' not mapped yet")
