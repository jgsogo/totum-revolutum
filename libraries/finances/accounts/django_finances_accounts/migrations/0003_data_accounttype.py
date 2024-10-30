from django.conf import settings
from django.db import migrations
from django.utils.translation import gettext_lazy as _

from ..constants import AccountTypeConstants


def populate_required_accounttypes(apps, schema_editor):
    AccountType = apps.get_model("finances_accounts", "AccountType")

    # /
    activos, _ = AccountType.objects.bulk_create(
        [
            AccountType(
                name=_("Activos"),
                unique_name=AccountTypeConstants.ASSETS,
                description=_(
                    "Los activos son los recursos que dispone una empresa, ya sean tangibles o intangibles, que permitan a esta obtener beneficios económicos a futuro."
                ),
            ),
            AccountType(
                name=_("Pasivos"),
                unique_name=AccountTypeConstants.LIABILITIES,
                description=_(
                    "Los pasivos son los gastos o deudas que la empresa posee a terceros, estos pueden ser pagos a bancos, salarios a empleados, entre otros."
                ),
            ),
        ]
    )
    # / Activos
    AccountType.objects.bulk_create(
        [
            AccountType(
                tn_parent=activos,
                name=_("Corrientes"),
                unique_name=AccountTypeConstants.ASSETS_CURRENT,
                description=_(
                    "También llamados activos líquidos son los bienes o posesiones que se pueden convertir rápidamente en dinero (efectivo)."
                ),
            ),
            AccountType(
                tn_parent=activos,
                name=_("No corrientes"),
                unique_name=AccountTypeConstants.ASSETS_NONCURRENT,
                description=_(
                    "Llamados también activos fijos y son aquellos pertenecientes al ente económico y adquiridos con la intención de utilizarlos en la producción de bienes y servicios propios de la actividad económica desarrollada. No están destinados para la venta y su vida útil excede de un año."
                ),
            ),
        ]
    )


def populate_optional_accounttypes(apps, schema_editor):
    if settings.FINANCES_MIGRATE_ONLY_REQUIRED_ACCOUNTTYPES:
        return

    AccountType = apps.get_model("finances_accounts", "AccountType")

    pasivos = AccountType.objects.get(unique_name=AccountsAccountTypeConstants.LIABILITIES)
    activos_corrientes = AccountType.objects.get(
        unique_name=AccountsAccountTypeConstants.ASSETS_CURRENT
    )
    activos_no_corrientes = AccountType.objects.get(
        unique_name=AccountsAccountTypeConstants.ASSETS_NONCURRENT
    )

    # / Activos / Corrientes
    AccountType.objects.bulk_create(
        [
            AccountType(tn_parent=activos_corrientes, name=_("Efectivo/Metálico/Cash")),
            AccountType(tn_parent=activos_corrientes, name=_("Cuenta bancaria")),
            AccountType(tn_parent=activos_corrientes, name=_("Depósito")),
            AccountType(tn_parent=activos_corrientes, name=_("Flujo de caja")),
        ]
    )
    # / Activos / No corrientes
    AccountType.objects.bulk_create(
        [
            AccountType(
                tn_parent=activos_no_corrientes, name=_("Vales descuento, cupones, tarjetas regalo")
            ),
        ]
    )
    # / Pasivos
    pasivo_credito, pasivo_prestamo = AccountType.objects.bulk_create(
        [
            AccountType(tn_parent=pasivos, name=_("Crédito")),
            AccountType(
                tn_parent=pasivos,
                name=_("Préstamo"),
                description=_(
                    "Un préstamo facilita todo el dinero de una sola vez en el mismo momento de la concesión"
                ),
            ),
        ]
    )
    # / Pasivos / Crédito
    AccountType.objects.bulk_create(
        [
            AccountType(tn_parent=pasivo_credito, name=_("Tarjeta de crédito")),
        ]
    )
    # / Pasivos / Préstamo
    AccountType.objects.bulk_create(
        [
            AccountType(tn_parent=pasivo_prestamo, name=_("Préstamo personal")),
            AccountType(tn_parent=pasivo_prestamo, name=_("Hipoteca")),
        ]
    )


class Migration(migrations.Migration):
    dependencies = [
        ("finances_accounts", "0002_accounttype_unique_name_movementtype_unique_name"),
    ]

    operations = [
        migrations.RunPython(populate_required_accounttypes),
        migrations.RunPython(populate_optional_accounttypes),
    ]
