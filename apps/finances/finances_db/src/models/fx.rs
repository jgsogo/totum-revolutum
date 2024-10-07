use diesel::prelude::*;

#[derive(Queryable, Selectable, PartialEq)]
#[diesel(table_name = crate::schema::data_fx)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Fx {
    pub id: i32,
    pub foreign: String,
    pub local: String,
    pub rate: f64,
    pub date_value: chrono::NaiveDate,
}
