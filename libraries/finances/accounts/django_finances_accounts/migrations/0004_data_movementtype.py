from django.conf import settings
from django.db import migrations
from django.utils.translation import gettext_lazy as _
from django_finances_accounts.constants import MovementTypeConstants


def populate_required_movementtypes(apps, schema_editor):
    MovementType = apps.get_model("finances_accounts", "MovementType")

    # /
    _1, _2, _3, tributos = MovementType.objects.bulk_create(
        [
            MovementType(
                name=_("Gastos"),
                unique_name=MovementTypeConstants.EXPENSE,
                is_abstract=True,
            ),
            MovementType(
                name=_("Ingresos"),
                unique_name=MovementTypeConstants.INCOME,
                is_abstract=True,
            ),
            MovementType(
                name=_("Operaciones"),
                unique_name=MovementTypeConstants.OPERATIONS,
                description=_(
                    "No suponen una salida ni entrada de dinero en las finanzas personales, simplemente un movimiento de una cuenta a otra"
                ),
                is_abstract=True,
            ),
            MovementType(
                name=_("Tributos"),
                unique_name=MovementTypeConstants.TAXES,
                description=_(
                    "Prestaciones dinerarias que los ciudadanos están obligados por ley a pagar"
                ),
                is_abstract=True,
            ),
        ]
    )

    # / Tributos
    _4, impuestos, _5 = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=tributos,
                name=_("Contribuciones especiales"),
                unique_name=MovementTypeConstants.ESPECIAL_TAXES,
                description=_(
                    "Son tributos exigidos como contraprestación directa por la obtención de un beneficio especial individual, como consecuencia de la realización de obras públicas o del establecimiento o ampliación de servicios públicos (una parada de metro o el asfaltado de una calle, por ejemplo)"
                ),
            ),
            MovementType(
                tn_parent=tributos,
                name=_("Impuestos"),
                unique_name=MovementTypeConstants.INCOME_AND_VALUE_TAXES,
                description=_(
                    "Tributos exigidos sin contraprestación directa, cuyo hecho imponible está constituido por negocios, actos o hechos que ponen de manifiesto la capacidad económica del contribuyente como consecuencia de la riqueza que posee (patrimonio), de los ingresos que obtiene (renta) o de lo que consume."
                ),
                is_abstract=True,
            ),
            MovementType(
                tn_parent=tributos,
                name=_("Tasas"),
                unique_name=MovementTypeConstants.FEES_AND_TOLLS,
                description=_(
                    "Se trata de tributos exigidos como contraprestación directa por la prestación de un servicio público, la realización de una actividad pública o la utilización privativa o el aprovechamiento especial del dominio público, cuando el servicio, actividad o aprovechamiento no sean de solicitud o recepción voluntaria para los obligados tributarios o no se presten o realicen por el sector privado. Ejemplos de tasas son las que se pagan por el abastecimiento de agua, la recogida de la basura o un vado permanente."
                ),
            ),
        ]
    )

    # / Tributos / Impuestos
    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=impuestos,
                name=_("Directos"),
                unique_name=MovementTypeConstants.DIRECT_TAXES,
                description=_(
                    "Son impuestos directos los que se aplican sobre una manifestación directa o inmediata de la capacidad económica: la posesión de un patrimonio y la obtención de una renta. Los impuestos directos gravan la riqueza en sí misma."
                ),
                is_abstract=True,
            ),
            MovementType(
                tn_parent=impuestos,
                name=_("Indirectos"),
                unique_name=MovementTypeConstants.INDIRECT_TAXES,
                description=_(
                    "Son impuestos indirectos, por el contrario, los que se aplican sobre una manifestación indirecta o mediata de la capacidad económica: la circulación de la riqueza, bien por actos de consumo o bien por actos de transmisión. Gravan la utilización de esa riqueza."
                ),
                is_abstract=True,
            ),
        ]
    )


def populate_optional_movementtypes(apps, schema_editor):
    if settings.FINANCES_MIGRATE_ONLY_REQUIRED_MOVMENTTYPES:
        return

    MovementType = apps.get_model("finances_accounts", "MovementType")

    gastos = MovementType.objects.get(unique_name=MovementTypeConstants.EXPENSE)
    ingresos = MovementType.objects.get(unique_name=MovementTypeConstants.INCOME)
    operaciones = MovementType.objects.get(unique_name=MovementTypeConstants.OPERATIONS)
    tributos = MovementType.objects.get(unique_name=MovementTypeConstants.TAXES)

    impuestos_directos = MovementType.objects.get(unique_name=MovementTypeConstants.DIRECT_TAXES)
    impuestos_indirectos = MovementType.objects.get(
        unique_name=MovementTypeConstants.INDIRECT_TAXES
    )

    # / Tributos / Impuestos / Directos
    (impuestos_directos_spain,) = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=impuestos_directos,
                name=_("España"),
                is_abstract=True,
            ),
        ]
    )

    # / Tributos / Impuestos / Directos / España
    impuestos_locales, impuestos_comunidad, impuestos_estado = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=impuestos_directos_spain,
                name=_("Locales"),
                description=_("Exigidos por los Ayuntamientos o Diputaciones Provinciales"),
                is_abstract=True,
            ),
            MovementType(
                tn_parent=impuestos_directos_spain,
                name=_("Comunidad Autónoma"),
                description=_("Impuestos cedidos a las comunidades"),
                is_abstract=True,
            ),
            MovementType(
                tn_parent=impuestos_directos_spain,
                name=_("Estado"),
                description=_("Impuestos recaudados por la administración central"),
                is_abstract=True,
            ),
        ]
    )
    # / Tributos / Impuestos / Directos / Locales
    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=impuestos_locales,
                name="IBI",
                description=_("Impuesto sobre Bienes Inmuebles"),
            ),
            MovementType(
                tn_parent=impuestos_locales,
                name="IVTM",
                description=_("Impuesto sobre Vehículos de Tracción Mecánica"),
            ),
            MovementType(
                tn_parent=impuestos_locales,
                name=_("Plusvalía"),
                description=_(
                    "Impuesto sobre el Incremento de Valor de los Terrenos de Naturaleza Urbana"
                ),
            ),
        ]
    )
    # / Tributos / Impuestos / Directos / Comunidad Autónoma
    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=impuestos_comunidad,
                name=_("Sucesiones y donaciones"),
                description=_(
                    "Impuesto sobre Sucesiones y Donaciones. El impuesto sobre Sucesiones y Donaciones se paga por las personas cuando reciben dinero u otros bienes de forma gratuita, es decir, sin que se trate de una contraprestación por un trabajo o servicio que hayan realizado o por un dinero o una cosa que hayan entregado a cambio. Se incluyen aquí tanto los casos en que lo que se recibe es una herencia o legado de una persona fallecida (adquisiciones “mortis causa”) como los casos en que lo que se recibe es una donación efectuada por una persona viva (adquisiciones “inter vivos”). Este impuesto está cedido a las Comunidades autónomas."
                ),
            ),
            MovementType(
                tn_parent=impuestos_comunidad,
                name=_("Patrimonio"),
            ),
        ]
    )
    # / Tributos / Impuestos / Directos / Estado
    (irpf,) = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=impuestos_estado,
                name="IRPF",
                description=_("Impuesto sobre la Renta de las Personas Físicas"),
                is_abstract=True,
            ),
        ]
    )
    # / Tributos / Impuestos / Directos / Estado / IRPF
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=irpf, name=_("Rendimientos del trabajo")),
            MovementType(tn_parent=irpf, name=_("Rendimientos del capital")),
            MovementType(tn_parent=irpf, name=_("Ganancias y pérdidas patrimoniales")),
        ]
    )

    # / Tributos / Impuestos / Indirectos
    (impuestos_indirectos_spain,) = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=impuestos_indirectos,
                name=_("España"),
                is_abstract=True,
            ),
        ]
    )

    # / Tributos / Impuestos / Indirectos / España
    iva, _1 = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=impuestos_indirectos_spain,
                name="IVA",
                description=_("Impuesto sobre el Valor Añadido"),
                is_abstract=True,
            ),
            MovementType(
                tn_parent=impuestos_indirectos_spain,
                name=_("Transmisiones patrimoniales y AJD"),
                description=_(
                    "Impuesto sobre Transmisiones Patrimoniales y Actos Jurídicos Documentados.\n\nEste impuesto tiene un ámbito de aplicación muy amplio y se subdivide en varias modalidades. Muy resumidamente, puede decirse que se aplica a las transmisiones (compraventas) de todo tipo de bienes y derechos, a determinadas operaciones que realizan las empresas y a actos que se tienen que documentar oficialmente (escritura de una casa y otros documentos notariales). La persona que tiene que pagar el impuesto es el adquirente (comprador), no el que transmite el bien o derecho (vendedor). Finalmente, es un impuesto cedido a las Comunidades Autónomas."
                ),
            ),
        ]
    )
    # / Tributos / Impuestos / Indirectos / IVA
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=iva, name=_("21% (tarifa general)")),
            MovementType(tn_parent=iva, name=_("10% (tarifa reducida)")),
            MovementType(tn_parent=iva, name=_("4% (tarifa superreducida)")),
        ]
    )

    # / Operaciones
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=operaciones, name=_("Transferencia")),
        ]
    )

    # / Ingresos
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=ingresos, name=_("Donaciones/Regalos")),
            MovementType(tn_parent=ingresos, name=_("Salario")),
        ]
    )

    # / Gastos
    (
        alimentacion,
        ocio,
        educacion,
        hijos,
        mascotas,
        ropa,
        salud,
        transporte,
        vacaciones,
        vivienda,
        comisiones,
    ) = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=gastos,
                name=_("Alimentación"),
                is_abstract=True,
            ),
            MovementType(tn_parent=gastos, name=_("Diversión/Ocio")),
            MovementType(tn_parent=gastos, name=_("Educación/Formación")),
            MovementType(tn_parent=gastos, name=_("Hijos")),
            MovementType(tn_parent=gastos, name=_("Mascotas")),
            MovementType(tn_parent=gastos, name=_("Ropa y calzado")),
            MovementType(tn_parent=gastos, name=_("Salud e higiene")),
            MovementType(
                tn_parent=gastos,
                name=_("Transporte"),
                is_abstract=True,
            ),
            MovementType(tn_parent=gastos, name=_("Vacaciones")),
            MovementType(tn_parent=gastos, name=_("Vivienda")),
            MovementType(tn_parent=gastos, name=_("Comisiones")),
        ]
    )
    # / Gastos / Alimentación
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=alimentacion, name=_("Supermercado")),
            MovementType(tn_parent=alimentacion, name=_("Restaurante")),
        ]
    )
    # / Gastos / Ocio
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=ocio, name=_("Cine")),
            MovementType(tn_parent=ocio, name=_("Gimnasio/Deporte")),
            MovementType(tn_parent=ocio, name=_("Libros")),
            MovementType(tn_parent=ocio, name=_("Bares")),
            MovementType(tn_parent=ocio, name=_("TV")),
        ]
    )
    # / Gastos / Educación
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=educacion, name=_("Cursos")),
            MovementType(tn_parent=educacion, name=_("Universidad")),
            MovementType(tn_parent=educacion, name=_("Conferencias")),
        ]
    )
    # / Gastos / Hijos
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=hijos, name=_("Colegio")),
            MovementType(tn_parent=hijos, name=_("Extraescolares")),
            MovementType(tn_parent=hijos, name=_("Libros y material escolar")),
            MovementType(tn_parent=hijos, name=_("Regalos/Juguetes")),
            MovementType(tn_parent=hijos, name=_("Ropa y calzado")),
            MovementType(tn_parent=hijos, name=_("Peluquería")),
        ]
    )
    # / Gastos / Mascotas
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=mascotas, name=_("Comida")),
            MovementType(tn_parent=mascotas, name=_("Veterinario")),
        ]
    )
    # / Gastos / Salud e higiene
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=salud, name=_("Seguro médico")),
            MovementType(tn_parent=salud, name=_("Dentista")),
            MovementType(tn_parent=salud, name=_("Peluquería")),
        ]
    )
    # / Gastos / Transporte
    _2, coche = MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=transporte, name=_("Transporte público")),
            MovementType(tn_parent=transporte, name=_("Coche")),
        ]
    )
    # / Gastos / Transporte / Coche
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=coche, name=_("Gasolina")),
            MovementType(tn_parent=coche, name=_("Reparaciones y mantenimiento")),
            MovementType(tn_parent=coche, name=_("Seguro")),
        ]
    )
    # / Gastos / Transporte / Vacaciones
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=vacaciones, name=_("Hotel")),
            MovementType(tn_parent=vacaciones, name=_("Avión")),
            MovementType(tn_parent=vacaciones, name=_("Restaurante")),
        ]
    )
    # / Gastos / Transporte / Vivienda
    _3, suministros, _4, comunidad, _5 = MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=vivienda, name=_("Alquiler")),
            MovementType(
                tn_parent=vivienda,
                name=_("Suministros"),
                is_abstract=True,
            ),
            MovementType(tn_parent=vivienda, name=_("Reparaciones y mantenimiento")),
            MovementType(
                tn_parent=vivienda,
                name=_("Comunidad"),
                is_abstract=True,
            ),
            MovementType(tn_parent=vivienda, name=_("Seguros")),
        ]
    )
    # / Gastos / Transporte / Vivienda / Suministros
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=suministros, name=_("Luz/Electricidad")),
            MovementType(tn_parent=suministros, name=_("Agua")),
            MovementType(tn_parent=suministros, name=_("Gas")),
            MovementType(tn_parent=suministros, name=_("Internet")),
            MovementType(tn_parent=suministros, name=_("Calefacción")),
        ]
    )
    # / Gastos / Transporte / Vivienda / Comunidad
    MovementType.objects.bulk_create(
        [
            MovementType(tn_parent=comunidad, name=_("Cuota de comunidad")),
            MovementType(tn_parent=comunidad, name=_("Derrama")),
        ]
    )


class Migration(migrations.Migration):
    dependencies = [
        ("finances_accounts", "0003_data_accounttype"),
    ]

    operations = [
        migrations.RunPython(populate_required_movementtypes),
        migrations.RunPython(populate_optional_movementtypes),
    ]
