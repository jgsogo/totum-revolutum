use crate::common::models::User;
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

    // all (objects)
    let r = User::all(User::as_select(), connection);
    assert!(r.is_ok());
    assert_eq!(r.unwrap().collect::<Vec<_>>(), expected_data_users);

    // all (some fields)

    // select some fields
    let expected_data = vec![
        ("Sean".to_string(), None::<String>),
        ("Tess".to_string(), None::<String>),
    ];
    let actual_data: Vec<_> = users.select((name, hair_color)).load(connection).unwrap();
    assert_eq!(expected_data, actual_data);

    // select all fields
    let actual_data: Vec<_> = users.select(User::as_select()).load(connection).unwrap();
    assert_eq!(expected_data_users, actual_data);
}
