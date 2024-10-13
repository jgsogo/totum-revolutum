use crate::{models::MovementType, test_utils::fixtures::database};
use diesel::prelude::*;

#[test]
fn test_constraints() {
    use crate::schema::data_movementtype::dsl::*;
    let mut database = database();

    // Check movementtype level is positive
    {
        let r = diesel::insert_into(data_movementtype)
            .values((name.eq("random"), level.eq(-2)))
            .execute(&mut database.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::CheckViolation));
                assert_eq!(info.message(), "CHECK constraint failed: level_positive");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // Check movementtype parent is set to null if parent is removed
    {
        let movtype = diesel::insert_into(data_movementtype)
            .values((name.eq("random"), level.eq(2), parent_id.eq::<Option<i32>>(Some(0))))
            .returning(MovementType::as_select())
            .get_result(&mut database.conn)
            .expect("Error creating movement type");

        assert_eq!(movtype.id, 9);

        // ... if I remove the parent
        diesel::delete(data_movementtype.filter(id.eq(0)))
            .execute(&mut database.conn)
            .expect("Error removing movement type");

        // ... the movtype no longer has a parent
        let (movtype_name, movtype_parent) = data_movementtype
            .filter(id.eq(movtype.id))
            .select((name, parent_id))
            .first::<(String, Option<i32>)>(&mut database.conn)
            .expect("Error loading movement type");

        assert_eq!(movtype_name, movtype.name);
        assert!(movtype_parent.is_none());
    }
}
