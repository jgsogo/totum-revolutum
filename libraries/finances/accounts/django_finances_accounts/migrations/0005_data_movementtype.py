from django.conf import settings
from django.db import migrations
from django.utils.translation import gettext_lazy as _
from django_finances_accounts.constants import MovementTypeConstants


def populate_movementtypes(apps, schema_editor):
    MovementType = apps.get_model("finances_accounts", "MovementType")

    gastos = MovementType.objects.get(unique_name=MovementTypeConstants.EXPENSE)
    ingresos = MovementType.objects.get(unique_name=MovementTypeConstants.INCOME)
    vivienda = MovementType.objects.get(tn_parent=gastos, name=_("Vivienda"))
    hijos = MovementType.objects.get(tn_parent=gastos, name=_("Hijos"))
    colegio = MovementType.objects.get(tn_parent=hijos, name=_("Colegio"))
    suministros = MovementType.objects.get(tn_parent=vivienda, name=_("Suministros"))
    salud_higiene = MovementType.objects.get(tn_parent=gastos, name=_("Salud e higiene"))
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

    (subvenciones, _1) = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=ingresos,
                name=_("Subvenciones (y ayudas públicas)"),
                description=_(
                    "La subvención es la entrega de dinero, bienes o servicios que realiza una administración pública a un particular, bien sea una persona física o jurídica. Esta entrega está exenta de la obligación de reembolso o devolución. Aunque la subvención no sea reembolsable, entre el beneficiario y la Administración se crea un vínculo jurídico por el cual, el primero adquiere el derecho a recibir el monto de dinero o bienes incluidos en la prestación, siempre que se cumplan las condiciones legales inherentes a la subvención. Esto obliga al beneficiario a llevar a cabo todas las actividades para las cuales se le entrega la subvención."
                ),
            ),
            MovementType(
                tn_parent=ingresos,
                name=_("Vales descuento/Tarjetas de fidelización"),
            ),
        ]
    )

    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=subvenciones,
                name=_("RDL 6/2022 (gasolina) [Bonificación]"),
                description=_(
                    "La bonificación es una reducción en una obligación fiscal. El impuesto NO se paga"
                ),
            ),
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
