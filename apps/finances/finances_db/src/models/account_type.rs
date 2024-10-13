use diesel::prelude::*;

// FIXME: All of these should be a tree of categories with some _hardcoded_ ones and others
// FIXME: populated by the user that are _children_ of the hardcoded ones. But that requires
// FIXME: changing the Python app as well... So probably it will take lot of time to get there.
// FIXME:
// FIXME: Those hardcoded categories will be the menús that we are listing in the GUI

pub(crate) const CUENTA_CORRIENTE: &str = "Cuenta corriente";
pub(crate) const METALICO: &str = "Metálico";
pub(crate) const FONDO_INVERSION: &str = "Fondo de inversión";
pub(crate) const ACCIONES: &str = "Acciones";
pub(crate) const VIVIENDA: &str = "Vivienda";
pub(crate) const DEPOSITO: &str = "Depósito";
pub(crate) const PLAN_PENSIONES: &str = "Plan de pensiones";
pub(crate) const _CUENTA_CREDITO: &str = "Cuenta crédito";
pub(crate) const _ONLINE: &str = "Online";
pub(crate) const _IMPUESTOS: &str = "Impuestos";
pub(crate) const _SUSCRIPCIONES: &str = "Suscripciones";

#[derive(Queryable, Selectable, PartialEq, Eq)]
#[diesel(table_name = crate::schema::data_accounttype)]
#[diesel(check_for_backend(crate::types::BackendType))]
pub struct AccountType {
    pub id: i32,
    pub name: String,
}
