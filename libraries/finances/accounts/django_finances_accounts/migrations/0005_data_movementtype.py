from django.conf import settings
from django.db import migrations
from django.utils.translation import gettext_lazy as _
from django_finances_accounts.constants import MovementTypeConstants


def populate_movementtypes(apps, schema_editor):
    MovementType = apps.get_model("finances_accounts", "MovementType")

    gastos = MovementType.objects.get(unique_name=MovementTypeConstants.EXPENSE)
    vivienda = MovementType.objects.get(tn_parent=gastos, name=_("Vivienda"))
    hijos = MovementType.objects.get(tn_parent=gastos, name=_("Hijos"))
    colegio = MovementType.objects.get(tn_parent=hijos, name=_("Colegio"))
    suministros = MovementType.objects.get(tn_parent=vivienda, name=_("Suministros"))
    salud_higiene = MovementType.objects.get(tn_parent=vivienda, name=_("Salud e higiene"))
    impuestos_indirectos = MovementType.objects.get(
        unique_name=MovementTypeConstants.INDIRECT_TAXES
    )
    impuestos_indirectos_spain = MovementType.objects.get(
        tn_parent=impuestos_indirectos, name=_("España")
    )
    iva = MovementType.objects.get(tn_parent=impuestos_indirectos_spain, name=_("IVA"))

    (colecciones,) = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=gastos,
                name=_("Coleccionables"),
                description=_("In theory they could be considered as investements"),
            ),
        ]
    )

    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=colegio,
                name=_("Cuotas/Aportaciones"),
            ),
            MovementType(
                tn_parent=colegio,
                name=_("AMPA"),
            ),
            MovementType(
                tn_parent=colegio,
                name=_("Uniforme"),
            ),
            MovementType(
                tn_parent=colegio,
                name=_("Comedor"),
            ),
            MovementType(
                tn_parent=gastos,
                name=_("Regalos/Propinas"),
            ),
            MovementType(
                tn_parent=gastos,
                name=_("Donaciones"),
            ),
            MovementType(
                tn_parent=vivienda,
                name=_("Muebles"),
            ),
            MovementType(
                tn_parent=vivienda,
                name=_("Limpieza"),
            ),
            MovementType(
                tn_parent=suministros,
                name=_("Teléfono"),
            ),
            MovementType(
                tn_parent=hijos,
                name=_("Guardería"),
            ),
            MovementType(
                tn_parent=hijos,
                name=_("Niñera/Au-pair/Canguro"),
            ),
            MovementType(
                tn_parent=gastos,
                name=_("Coleccionables"),
                description=_("In theory they could be considered as investements"),
            ),
            MovementType(
                tn_parent=colecciones,
                name=_("LEGO"),
            ),
            MovementType(
                tn_parent=salud_higiene,
                name=_("Farmacia"),
            ),
            MovementType(
                tn_parent=iva,
                name=_("5% (RDL 20/2022 y RDL 4/2024)"),
            ),
        ]
    )


class Migration(migrations.Migration):
    dependencies = [
        ("finances_accounts", "0004_data_movementtype"),
    ]

    operations = [
        migrations.RunPython(populate_movementtypes),
    ]
