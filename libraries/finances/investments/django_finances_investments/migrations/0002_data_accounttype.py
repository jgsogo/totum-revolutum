from django.db import migrations
from django.utils.translation import gettext_lazy as _

from ..constants import AccountTypeConstants


def populate_accounttype(apps, schema_editor):
    AccountType = apps.get_model("finances_accounts", "AccountType")

    activos = AccountType.objects.get(unique_name=AccountTypeConstants.ASSETS)
    activos_corrientes = AccountType.objects.get(unique_name=AccountTypeConstants.ASSETS_CURRENT)
    activos_no_corrientes = AccountType.objects.get(
        unique_name=AccountTypeConstants.ASSETS_NONCURRENT
    )

    # / Activos / Corrientes
    AccountType.objects.bulk_create(
        [
            AccountType(tn_parent=activos_corrientes, name=_("Acciones")),
            AccountType(tn_parent=activos_corrientes, name=_("ETF")),
            AccountType(tn_parent=activos_corrientes, name=_("Fondo de inversión")),
        ]
    )
    # / Activos / No corrientes
    plan_pensiones, bienes_inmuebles, _1, _2 = AccountType.objects.bulk_create(
        [
            AccountType(tn_parent=activos_no_corrientes, name=_("Plan de pensiones")),
            AccountType(tn_parent=activos_no_corrientes, name=_("Bienes inmuebles")),
            AccountType(tn_parent=activos_no_corrientes, name=_("Colecciones/Arte")),
            AccountType(tn_parent=activos_no_corrientes, name=_("Vehículo")),
        ]
    )
    # / Activos / No corrientes / Plan de pensiones
    AccountType.objects.bulk_create(
        [
            AccountType(tn_parent=plan_pensiones, name=_("Individual")),
            AccountType(
                tn_parent=plan_pensiones,
                name=_("Empleo"),
                description=_(
                    "Son aquellos en los que el promotor es una empresa y los partícipes son exclusivamente los trabajadores de esa empresa."
                ),
            ),
        ]
    )

    # / Activos / No corrientes / Bienes inmuebles
    AccountType.objects.bulk_create(
        [
            AccountType(tn_parent=bienes_inmuebles, name=_("Terrenos")),
            AccountType(tn_parent=bienes_inmuebles, name=_("Vivienda")),
        ]
    )


class Migration(migrations.Migration):
    dependencies = [
        ("finances_accounts", "0003_data_accounttype"),
        ("finances_investments", "0001_initial"),
    ]

    operations = [
        migrations.RunPython(populate_accounttype),
    ]
