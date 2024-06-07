use crate::common::models::User;
use crate::common::schema;
use diesel::*;
use diesel_utils::All;

mod common;

#[test]
fn all() {
    use common::schema::users::dsl::*;

    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let expected_data_users = vec![
        User {
            id: 1,
            name: "Sean".to_string(),
            hair_color: None,
        },
        User {
            id: 2,
            name: "Tess".to_string(),
            hair_color: None,
        },
    ];
    let expected_data = vec![
        ("Sean".to_string(), None::<String>),
        ("Tess".to_string(), None::<String>),
    ];

    // all (objects)
    let r = User::all(User::as_select(), connection).unwrap();
    assert_eq!(r.collect::<Vec<_>>(), expected_data_users);

    // all (some fields)
    let r = User::all((name, hair_color), connection).unwrap();
    assert_eq!(r.collect::<Vec<(String, Option<String>)>>(), expected_data);

    // select some fields
    let actual_data: Vec<_> = users.select((name, hair_color)).load(connection).unwrap();
    assert_eq!(expected_data, actual_data);

    // select all fields
    let actual_data: Vec<_> = users.select(User::as_select()).load(connection).unwrap();
    assert_eq!(expected_data_users, actual_data);
}
