from django.conf import settings
from django.db import migrations
from django.utils.translation import gettext_lazy as _

from ..constants import MovementTypeConstants


def populate_movementtypes(apps, schema_editor):
    MovementType = apps.get_model("finances_accounts", "MovementType")

    operations = MovementType.objects.get(unique_name=MovementTypeConstants.OPERATIONS)
    comisiones = MovementType.objects.get(name=_("Comisiones"))
    ingresos_inversiones = MovementType.objects.get(name=_("Inversiones"))

    (acciones,) = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=operations,
                name=_("Acciones"),
                description=_("Operations that involve stocks"),
            ),
        ]
    )

    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=acciones,
                name=_("Stock split"),
            ),
            MovementType(
                tn_parent=acciones,
                name=_("Spin-off"),
                description=_("An existing company is divided into two"),
            ),
            MovementType(
                tn_parent=comisiones,
                name=_("ADR pass-through fees"),
            ),
            MovementType(
                tn_parent=ingresos_inversiones,
                name=_("Emisión derechos"),
            ),
        ]
    )


class Migration(migrations.Migration):
    dependencies = [
        ("finances_investments", "0003_data_movementtype"),
    ]

    operations = [
        migrations.RunPython(populate_movementtypes),
    ]
