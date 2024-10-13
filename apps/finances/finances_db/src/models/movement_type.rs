use diesel::prelude::*;

// FIXME: We should have a set of _hardcoded_ MovementTypes into the database, and then the
// FIXME: user can add its own ones. Same for the AccountType data.

#[allow(dead_code)]
pub(crate) const RENDIMIENTOS_BIENES_INMUEBLES: &str = "Rendimientos de bienes inmuebles";
#[allow(dead_code)]
pub(crate) const RENDIMIENTOS_TRABAJO: &str = "Rendimientos del trabajo";
#[allow(dead_code)]
pub(crate) const LOPE_DE_HARO: &str = "Lope de Haro";
#[allow(dead_code)]
pub(crate) const RENDIMIENTOS_CAPITAL: &str = "Rendimientos del capital";
#[allow(dead_code)]
pub(crate) const MOVIMIENTO_EFECTIVO: &str = "Movimiento de efectivo";
#[allow(dead_code)]
pub(crate) const IMPUESTOS: &str = "Impuestos";
#[allow(dead_code)]
pub(crate) const ACCIONES: &str = "Acciones";
#[allow(dead_code)]
pub(crate) const AYUDAS_SUBVENCIONES: &str = "Ayudas, subvenciones";
#[allow(dead_code)]
pub(crate) const CHALET_ATYKA: &str = "Chalet Atyka";

#[derive(Queryable, Selectable, PartialEq, Eq)]
#[diesel(table_name = crate::schema::data_movementtype)]
#[diesel(check_for_backend(crate::types::BackendType))]
pub struct MovementType {
    pub id: i32,
    pub name: String,
    pub level: i16,
    pub parent_id: Option<i32>,
}
