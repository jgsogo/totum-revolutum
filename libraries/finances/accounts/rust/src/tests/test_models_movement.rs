use crate::fields::MovementDirection;
use crate::models::Movement;
use crate::test_utils::establish_connection;
use diesel::prelude::*;

#[test]
fn test_movements() {
    let pool = establish_connection();
    let mut conn = pool.get().unwrap();

    let movs = Movement::all()
        .select(Movement::as_select())
        .load::<Movement>(&mut conn)
        .unwrap();

    assert_eq!(movs.len(), 2);

    let mov0 = movs.get(0).unwrap();
    assert_eq!(mov0.direction, MovementDirection::In);
}
