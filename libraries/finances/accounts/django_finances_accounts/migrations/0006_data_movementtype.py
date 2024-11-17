from django.conf import settings
from django.db import migrations
from django.utils.translation import gettext_lazy as _
from django_finances_accounts.constants import MovementTypeConstants


def populate_movementtypes(apps, schema_editor):
    MovementType = apps.get_model("finances_accounts", "MovementType")

    ingresos = MovementType.objects.get(unique_name=MovementTypeConstants.INCOME)
    salario = MovementType.objects.get(tn_parent=ingresos, name=_("Salario"))

    gastos = MovementType.objects.get(unique_name=MovementTypeConstants.EXPENSE)
    impuestos_directos = MovementType.objects.get(unique_name=MovementTypeConstants.DIRECT_TAXES)
    impuestos_directos_espana = MovementType.objects.get(
        tn_parent=impuestos_directos, name=_("España")
    )
    rendimiento_trabajo = MovementType.objects.get(name=_("Rendimientos del trabajo"))

    operations = MovementType.objects.get(unique_name=MovementTypeConstants.OPERATIONS)

    (seguridad_social, deducciones_salario) = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=impuestos_directos_espana,
                name=_("Seguridad Social"),
                description=_("Cuotas a la Seguridad Social"),
            ),
            MovementType(
                tn_parent=operations,
                name=_("Deducciones Salario"),
                description=_(
                    "Son deducidas del salario, pero inmediatamente se transforman en un beneficio: cheque guardería, ESPP, seguro médico"
                ),
            ),
        ]
    )

    # Cotizaciones Seguridad Social
    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=seguridad_social,
                name=_("Contingencias comunes (4,70%)"),
            ),
            MovementType(
                tn_parent=seguridad_social,
                name=_("Formación profesional (0,10%)"),
            ),
            MovementType(
                tn_parent=seguridad_social,
                name=_("Desempleo"),
                description=_(
                    "1,55% si el contrato es indefinido y 1,60 % si el contrato es temporal"
                ),
            ),
            MovementType(
                tn_parent=seguridad_social,
                name=_("Desempleo + Formación profesional"),
                description=_(
                    "Formación profesional (0,10%) + Desempleo (1,55% si el contrato es indefinido y 1,60 % si el contrato es temporal)"
                ),
            ),
            MovementType(
                tn_parent=seguridad_social,
                name=_("Plan Dental (Cotización)"),  # TODO: It probably doesn't belong here
            ),
            MovementType(
                tn_parent=seguridad_social,
                name=_("Cotización MEI (Mecanismo Equidad Intergeneracional)"),
            ),
        ]
    )

    # IRPF / Rendimiento trabajo
    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=rendimiento_trabajo,
                name=_("Retención IRPF Pagos Esp. No Reperc."),
                description=_(
                    "Este concepto también aparece como deducción: Pagos especie (no repercutidos) - Devengo"
                ),
            ),
            MovementType(
                tn_parent=rendimiento_trabajo,
                name=_("Total Especies"),
            ),
        ]
    )

    # Salario - conceptos
    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=salario,
                name=_("Salario base"),
            ),
            MovementType(
                tn_parent=salario,
                name=_("Plus exclusividad"),
            ),
            MovementType(
                tn_parent=salario,
                name=_("Parte proporcional pagas extraordinarias"),
            ),
            MovementType(
                tn_parent=salario,
                name=_("Paga extra"),
            ),
            MovementType(
                tn_parent=salario,
                name=_("Regularización salarial"),
            ),
            MovementType(
                tn_parent=salario,
                name=_("Finiquito"),
            ),
            MovementType(
                tn_parent=salario,
                name=_("Indemnización"),
            ),
            MovementType(
                tn_parent=salario,
                name=_("Liquidación vacaciones"),
            ),
            MovementType(
                tn_parent=salario,
                name=_("Complemento dedicación sede"),
            ),
            MovementType(
                tn_parent=salario,
                name=_("Mejora voluntaria - Complemento personal"),
            ),
            MovementType(
                tn_parent=salario,
                name=_("Plus convenio - Complemento de puesto"),
            ),
            MovementType(
                tn_parent=salario,
                name=_("Pagos especie (no repercutidos) - Devengo"),
                description=_("El mismo concepto aparece como devengo y como deducción!"),
            ),
            MovementType(
                tn_parent=salario,
                name=_("Stock options"),
            ),
        ]
    )

    # Salario / Dietas
    (dietas,) = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=salario,
                name=_("Dietas"),
            ),
        ]
    )
    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=dietas,
                name=_("Dietas sin pernocta"),
            ),
            MovementType(
                tn_parent=dietas,
                name=_("Dietas pernocta extranjero"),
            ),
            MovementType(
                tn_parent=dietas,
                name=_("Dietas pernocta España"),
            ),
        ]
    )

    # Salario / Gastos
    (salario_gastos,) = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=salario,
                name=_("Gastos"),
                description=_(
                    "Gastos abonados por el empleado que son reintegrados por la empresa"
                ),
            ),
        ]
    )
    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=salario_gastos,
                name=_("Gastos locomoción"),
            ),
            MovementType(
                tn_parent=salario_gastos,
                name=_("Gastos estancia"),
            ),
            MovementType(
                tn_parent=salario_gastos,
                name=_("Gastos exentos"),
            ),
        ]
    )

    # Salario / Ayuda
    (salario_ayuda,) = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=salario,
                name=_("Ayuda"),
            ),
        ]
    )
    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=salario_ayuda,
                name=_("Ayuda gimnasio"),
            ),
        ]
    )

    # Salario / Premios/Bonus
    (salario_premio_bonus,) = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=salario,
                name=_("Premios/Bonus"),
            ),
        ]
    )
    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=salario_premio_bonus,
                name=_("Premio nupcialidad"),
            ),
            MovementType(
                tn_parent=salario_premio_bonus,
                name=_("Bonus"),
            ),
            MovementType(
                tn_parent=salario_premio_bonus,
                name=_("Prima"),
            ),
            MovementType(
                tn_parent=salario_premio_bonus,
                name=_("Premio"),
            ),
            MovementType(
                tn_parent=salario_premio_bonus,
                name=_("GDP"),
                description=_("IBM - Growth-driven profit"),
            ),
            MovementType(
                tn_parent=salario_premio_bonus,
                name=_("Blue Points"),
                description=_("IBM - Blue Points"),
            ),
        ]
    )

    # Operaciones / Deducciones salario
    (espp, _1, _2) = MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=deducciones_salario,
                name=_("ESPP"),
            ),
            MovementType(
                tn_parent=deducciones_salario,
                name=_("Guardería (R. Flex)"),
            ),
            MovementType(
                tn_parent=deducciones_salario,
                name=_("Seguro médico (R. Flex)"),
            ),
        ]
    )
    MovementType.objects.bulk_create(
        [
            MovementType(
                tn_parent=espp,
                name=_("Aportaciones empleado"),
            ),
            MovementType(
                tn_parent=espp,
                name=_("Complemento empresa"),
            ),
        ]
    )


class Migration(migrations.Migration):
    dependencies = [
        ("finances_accounts", "0005_data_movementtype"),
    ]

    operations = [
        migrations.RunPython(populate_movementtypes),
    ]
