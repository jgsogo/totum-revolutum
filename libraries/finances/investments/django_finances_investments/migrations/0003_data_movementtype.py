from django.conf import settings
from django.db import migrations
from django.utils.translation import gettext_lazy as _

from ..constants import MovementTypeConstants


def populate_required_movementtypes(apps, schema_editor):
    MovementType = apps.get_model("finances_accounts", "MovementType")

    incomes = MovementType.objects.get(unique_name=MovementTypeConstants.INCOME)
    operations = MovementType.objects.get(unique_name=MovementTypeConstants.OPERATIONS)

    # / Ingresos
    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=incomes,
                name=_("Inversiones"),
                unique_name=MovementTypeConstants.INVESTMENTS,
            ),
        ]
    )


def populate_optional_movementtypes(apps, schema_editor):
    if settings.FINANCES_MIGRATE_ONLY_REQUIRED_MOVMENTTYPES:
        return

    MovementType = apps.get_model("finances_accounts", "MovementType")

    inversiones = MovementType.objects.get(unique_name=MovementTypeConstants.INVESTMENTS)
    tributos = MovementType.objects.get(unique_name=MovementTypeConstants.TAXES)
    impuestos_directos = MovementType.objects.get(unique_name=MovementTypeConstants.DIRECT_TAXES)

    # / Ingresos / Inversiones
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=inversiones, name=_("Intereses")),
            MovementType(tn_parent=inversiones, name=_("Dividendos")),
            MovementType(tn_parent=inversiones, name=_("Alquiler")),
        ]
    )

    # / Operaciones
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=operations, name=_("Compra/Venta acciones")),
            MovementType(tn_parent=operations, name=_("Compra/Venta fondos")),
            MovementType(tn_parent=operations, name=_("Compra/Venta patrimonio")),
            MovementType(tn_parent=operations, name=_("Compra/Venta ETFs")),
        ]
    )

    # / Tributos / Impuestos / Directos
    usa_taxes = MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=impuestos_directos, name=_("USA")),
        ]
    )

    # / Tributos / Impuestos / Directos / USA
    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=usa_taxes,
                unique_name=MovementTypeConstants.TAXES_USA_DIVIDEND_15,
                name=_(
                    "Dividend tax (15%)",
                    description=_(
                        "En general, la tasa de retención fiscal de un dividendo en efectivo abonado por una corporación estadounidense es del 30%, pero con el formulario W-8BEN (en virtud del acuerdo de doble imposición con España) la retención es del 15%"
                    ),
                ),
            ),
        ]
    )


class Migration(migrations.Migration):
    dependencies = [
        ("finances_accounts", "0004_data_movementtype"),
        ("finances_investments", "0002_data_accounttype"),
    ]

    operations = [
        migrations.RunPython(populate_required_movementtypes),
        migrations.RunPython(populate_optional_movementtypes),
    ]
