use crate::constants;
use crate::managers::get_breadcrumbs_for_movementtype;
use crate::models::MovementType;
use crate::sql::filters::movementtype_by_unique_name;
use crate::test_utils::establish_connection;
use diesel::prelude::*;

#[test]
fn test_breadcrumbs() {
    let pool = establish_connection();
    let mut conn = pool.get().unwrap();

    let assets_current = MovementType::all()
        .filter(movementtype_by_unique_name(
            constants::movementtype::TAXES_INCOME_AND_VALUE_DIRECT,
        ))
        .select(MovementType::as_select())
        .get_result::<MovementType>(&mut conn)
        .unwrap();

    let breadcrumbs = get_breadcrumbs_for_movementtype(&mut conn, &assets_current).unwrap();
    assert_eq!(breadcrumbs.len(), 3);
    assert_eq!(
        breadcrumbs,
        ["Tributos".to_string(), "Impuestos".to_string(), "Directos".to_string()]
    );
}
