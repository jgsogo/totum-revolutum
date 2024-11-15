import logging
from typing import Dict

from django.core.management.base import BaseCommand  # , CommandError
from django_finances_accounts.constants import MovementTypeConstants
from django_finances_accounts.models import (
    Account,
    AccountHolder,
    AccountType,
    Custodian,
    Fx,
    MovementType,
    Snapshot,
)
from django_finances_investments.models import SnapshotNumerable
from sqlalchemy import create_engine
from sqlalchemy.sql import text
from tqdm import tqdm

log = logging.getLogger(__name__)


class Command(BaseCommand):
    help = "Migrates legacy DB"

    def add_arguments(self, parser):
        parser.add_argument("--DATABASE_URL", type=str)

    def handle(self, *args, **options):
        database_url = options["DATABASE_URL"]
        engine = create_engine(f"postgresql+psycopg:{database_url}")

        with engine.connect() as c:
            _account_holders: Dict[int, AccountHolder] = self.migrate_accountholders(c)
            _account_types: Dict[int, AccountType] = self.migrate_accounttype(c)
            _accounts: Dict[int, Account] = self.migrate_account(
                c, account_types=_account_types, account_holders=_account_holders
            )
            _fxs: Dict[int, Fx] = self.migrate_fx(c)
            _movement_types: Dict[int, MovementType] = self.migrate_movementtype(c)
            self.migrate_snapshots(c, accounts=_accounts)

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

    def migrate_accounttype(self, conn):
        total = conn.execute(text("SELECT COUNT(*) FROM data_accounttype")).scalar()

        statement = text("SELECT * FROM data_accounttype")
        results = conn.execute(statement)

        to_be_removed, _1 = AccountType.objects.get_or_create(name="to-be-removed")
        logacy_name_to_account_type = {
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

        mapping = {}
        for pk, name in tqdm(results, total=total, desc="data/accounttype"):
            try:
                account_type = logacy_name_to_account_type[name]
                mapping[pk] = account_type
            except KeyError:
                self.stderr.write(f"Legacy AccountType '{name}' not mapped yet")

        return mapping

    def migrate_account(self, conn, account_types, account_holders):
        total = conn.execute(text("SELECT COUNT(*) FROM data_account")).scalar()

        statement = text(
            "SELECT id, type_id, holder_id, identifier, name, is_numerable, ccy, open, close "
            "FROM data_account"
        )
        results = conn.execute(statement)

        fake_custodian, _ = Custodian.objects.get_or_create(name="fake", defaults={"country": "es"})
        mapping = {}
        for account in tqdm(results, total=total, desc="data/accounttype"):
            (
                pk,
                acc_type,
                acc_holder,
                acc_identifier,
                acc_name,
                is_numerable,
                ccy,
                acc_open,
                acc_close,
            ) = account
            acc_type = account_types[acc_type]
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

    def migrate_movementtype(self, conn):
        total = conn.execute(text("SELECT COUNT(*) FROM data_movementtype")).scalar()
        statement = text("SELECT id,name FROM data_movementtype")
        results = conn.execute(statement)

        to_be_removed, _ = MovementType.objects.get_or_create(name="to-be-removed")
        renta_things, _ = MovementType.objects.get_or_create(
            name="renta-things",
            tn_parent=to_be_removed,
            description="This item in the classification if more for Declaración de la Renta",
        )
        inversiones_alquiler = MovementType.objects.get(
            name="Alquiler", tn_parent=MovementType.objects.get(name="Inversiones")
        )
        gasto_alquiler = MovementType.objects.get(
            name="Alquiler", tn_parent=MovementType.objects.get(name="Vivienda")
        )
        salario = MovementType.objects.get(
            name="Salario",
            tn_parent=MovementType.objects.get(unique_name=MovementTypeConstants.INCOME),
        )
        logacy_name_to_movement_type = {
            "Transferencia": MovementType.objects.get(name="Transferencia"),
            "Rendimientos de bienes inmuebles": inversiones_alquiler,
            "Rendimientos del trabajo": salario,
            "Lope de Haro": MovementType.objects.get(
                name="Alquiler", tn_parent=MovementType.objects.get(name="Vivienda")
            ),
            "Suministros": MovementType.objects.get(name="Suministros"),
            "Electricidad": MovementType.objects.get(name="Luz/Electricidad"),
            "Agua": MovementType.objects.get(name="Agua"),
            "IBI": MovementType.objects.get(name="IBI"),
            "Comunidad": MovementType.objects.get(name="Comunidad"),
            "Rendimientos del capital": MovementType.objects.get(name="Inversiones"),
            "Dividendo": MovementType.objects.get(name="Dividendos"),
            "Intereses": MovementType.objects.get(name="Intereses"),
            "Tributación rendimiento capital mobiliario": MovementType.objects.get(
                name="Rendimientos del capital"
            ),
            "Movimiento de efectivo": MovementType.objects.get(name="Transferencia"),
            "Retirada cajero": MovementType.objects.get(name="Transferencia"),
            "Ingreso en cuenta": MovementType.objects.get(name="Transferencia"),
            "Alquiler": inversiones_alquiler,
            "Fianza": MovementType.objects.get(name="Transferencia"),
            "Comisiones": MovementType.objects.get(name="Comisiones"),
            "Ocio": MovementType.objects.get(name="Diversión/Ocio"),
            "Vacaciones": MovementType.objects.get(name="Vacaciones"),
            "Impuestos": MovementType.objects.get(name="Tributos"),
            "IVA 4%": MovementType.objects.get(name="4% (tarifa superreducida)"),
            "IVA 10%": MovementType.objects.get(name="10% (tarifa reducida)"),
            "IVA 21%": MovementType.objects.get(name="21% (tarifa general)"),
            "Supermercado": MovementType.objects.get(name="Supermercado"),
            "Internet": MovementType.objects.get(name="Internet"),
            "Teléfono": MovementType.objects.get(name="Teléfono"),
            "Viajes": MovementType.objects.get(name="Vacaciones"),
            "Acciones": MovementType.objects.get(name="Operaciones"),
            "Stock split": MovementType.objects.get(name="Stock split"),
            "Compra/Venta acciones": MovementType.objects.get(
                name="Acciones", tn_parent=MovementType.objects.get(name="Operaciones")
            ),
            "Retención USA": MovementType.objects.get(name="Dividend tax (15%)"),
            "Compra cosas": MovementType.objects.get(unique_name=MovementTypeConstants.EXPENSE),
            "Emisión derechos": MovementType.objects.get(name="Emisión derechos"),
            "Rdtos capital (19%)": MovementType.objects.get(
                name="Rendimientos del capital"
            ),  # Most of the time it is 19%, that is then adjusted in the Tax Declaration
            "Adrián": MovementType.objects.get(name="Hijos"),
            "Ayudas, subvenciones": MovementType.objects.get(
                name="Subvenciones (y ayudas públicas)"
            ),
            "RDL 6/2022 (gasolina)": MovementType.objects.get(
                name="RDL 6/2022 (gasolina) [Bonificación]"
            ),
            "Muebles": MovementType.objects.get(name="Muebles"),
            "Guardería": MovementType.objects.get(name="Guardería"),
            "Au-pair": MovementType.objects.get(name="Niñera/Au-pair/Canguro"),
            "LEGO (move to Movimiento de efectivo::Compra cosas::LEGO)": MovementType.objects.get(
                name="LEGO"
            ),
            "LEGO": MovementType.objects.get(name="LEGO"),
            "Garage Molinos": inversiones_alquiler,
            "Medicina y salud": MovementType.objects.get(name="Salud e higiene"),
            "Chalet Atyka": to_be_removed,
            "Alquiler - Atyka": gasto_alquiler,
            "Comunidad - Atyka": MovementType.objects.get(name="Cuota de comunidad"),
            "Suministros - Atyka": MovementType.objects.get(name="Suministros"),
            "Seguro médico": MovementType.objects.get(name="Seguro médico"),
            "ADR pass-through fees": MovementType.objects.get(name="ADR pass-through fees"),
            "Alquiler - Lope de Haro": inversiones_alquiler,
            "Farmacia": MovementType.objects.get(name="Farmacia"),
            "Gasolina": MovementType.objects.get(name="Gasolina"),
            "IVA 5% (aceites, pasta - enero 23)": MovementType.objects.get(
                name="5% (RDL 20/2022 y RDL 4/2024)"
            ),
            "Fianza - Lope de Haro": MovementType.objects.get(name="Transferencia"),
            "Cupones y ofertas": MovementType.objects.get(name="Transferencia"),
            "Seguro de hogar": MovementType.objects.get(name="Seguros"),
            "Derramas": MovementType.objects.get(name="Derrama"),
            "Propinas": MovementType.objects.get(name="Regalos/Propinas"),
            "Limpieza": MovementType.objects.get(name="Limpieza"),
            "Colegio": MovementType.objects.get(name="Colegio"),
            "Material escolar": MovementType.objects.get(name="Libros y material escolar"),
            "Cuotas/Aportaciones": MovementType.objects.get(name="Cuotas/Aportaciones"),
            "Uniforme": MovementType.objects.get(name="Uniforme"),
            "AMPA": MovementType.objects.get(name="AMPA"),
            "Comedor": MovementType.objects.get(name="Comedor"),
            "Extraescolares": MovementType.objects.get(name="Extraescolares"),
            "Ropa": MovementType.objects.get(
                name="Ropa y calzado", tn_parent=MovementType.objects.get(name="Hijos")
            ),
            "Deporte": MovementType.objects.get(name="Gimnasio/Deporte"),
            "Regalo": MovementType.objects.get(name="Regalos/Propinas"),
            # Salario
            "(995) Cotización contingencias comunes": MovementType.objects.get(
                name="Contingencias comunes (4,70%)"
            ),
            "(996) Cotización formación": MovementType.objects.get(
                name="Formación profesional (0,10%)"
            ),
            "(997) Cotización desempleo": MovementType.objects.get(name="Desempleo"),
            "(999) Tributación IRPF": MovementType.objects.get(name="Rendimientos del trabajo"),
            "Cotización Régimen General (4,70%)": MovementType.objects.get(
                name="Contingencias comunes (4,70%)"
            ),
            "Cotización D+F+P+S (1,65%)": MovementType.objects.get(
                name="Desempleo + Formación profesional"
            ),
            "(1) Salario base": MovementType.objects.get(name="Salario base"),
            "(62) Plus exclusividad": MovementType.objects.get(name="Plus exclusividad"),
            "(78) Parte proporcional pagas extraordinarias": MovementType.objects.get(
                name="Parte proporcional pagas extraordinarias"
            ),
            "(147) Regularización salarial": MovementType.objects.get(
                name="Regularización salarial"
            ),
            "(606) Dietas sin pernocta": MovementType.objects.get(name="Dietas sin pernocta"),
            "(607) Gastos locomoción": MovementType.objects.get(name="Gastos locomoción"),
            "(615) Dietas pernocta extranjero": MovementType.objects.get(
                name="Dietas pernocta extranjero"
            ),
            "(612) Dietas pernocta España": MovementType.objects.get(name="Dietas pernocta España"),
            "(620) Gastos estancia": MovementType.objects.get(name="Gastos estancia"),
            "(642) Gastos exentos": MovementType.objects.get(name="Gastos exentos"),
            "(320) Ayuda gimnasio": MovementType.objects.get(name="Ayuda gimnasio"),
            "Premio nupcialidad": MovementType.objects.get(name="Premio nupcialidad"),
            "Bonus": MovementType.objects.get(name="Bonus"),
            "Prima": MovementType.objects.get(name="Prima"),
            "Blue Points": MovementType.objects.get(name="Blue Points"),
            "GDP": MovementType.objects.get(name="GDP"),
            "Premio": MovementType.objects.get(name="Premio"),
            "Paga Extra": MovementType.objects.get(name="Paga extra"),
            "Paga Extra Prorrateada": MovementType.objects.get(
                name="Parte proporcional pagas extraordinarias"
            ),
            "Finiquito": MovementType.objects.get(name="Finiquito"),
            "Indemnización": MovementType.objects.get(name="Indemnización"),
            "Liquidación vacaciones": MovementType.objects.get(name="Liquidación vacaciones"),
            "Complemento dedicación sede": MovementType.objects.get(
                name="Complemento dedicación sede"
            ),
            "(3) Mejora voluntaria - Complemento personal": MovementType.objects.get(
                name="Mejora voluntaria - Complemento personal"
            ),
            "(2) Plus convenio - Complemento de puesto": MovementType.objects.get(
                name="Plus convenio - Complemento de puesto"
            ),
            "ESPP": MovementType.objects.get(name="ESPP"),
            "Aportaciones empleado": MovementType.objects.get(name="Aportaciones empleado"),
            "Complemento empresa": MovementType.objects.get(name="Complemento empresa"),
            "Deducción ESPP": MovementType.objects.get(name="ESPP"),
            "Guardería (R. Flex)": MovementType.objects.get(name="Guardería (R. Flex)"),
            "Seguro médico (R. Flex)": MovementType.objects.get(name="Seguro médico (R. Flex)"),
            "Deducción ESPP Variable": MovementType.objects.get(name="ESPP"),
            "Plan Dental (Cotización)": MovementType.objects.get(name="Plan Dental (Cotización)"),
            "Cotización MEI (Mecanismo Equidad Intergeneracional)": MovementType.objects.get(
                name="Cotización MEI (Mecanismo Equidad Intergeneracional)"
            ),
            "(708) Descuento valor especies": MovementType.objects.get(name="Total Especies"),
            "Stock options": MovementType.objects.get(name="Stock options"),
            "Retribución flexible": MovementType.objects.get(
                name="Deducciones Salario"
            ),  # Not really, probably remove
            "Total especies": MovementType.objects.get(name="Total Especies"),
            "Pagos especie (no repercutidos)": MovementType.objects.get(
                name="Retención IRPF Pagos Esp. No Reperc."
            ),
        }

        mapping = {}
        for pk, name in tqdm(results, total=total, desc="data/data_movementtype"):
            try:
                movement_type = logacy_name_to_movement_type[name]
                mapping[pk] = movement_type
            except KeyError:
                self.stderr.write(f"Legacy MovmentType '{name}' not mapped yet")

        return mapping

    def migrate_snapshots(self, conn, accounts: Dict[int, Account]):
        total = conn.execute(text("SELECT COUNT(*) FROM data_snapshot")).scalar()

        statement = text(
            "SELECT id, account_id, date_value, amount, quantity, unit_value " "FROM data_snapshot"
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


#  public | data_movement                 | table | finances
#  public | data_transfer                 | table | finances
