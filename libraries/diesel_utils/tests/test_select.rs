use diesel::*;

mod common;

#[test]
fn selecting_basic_data() {
    use common::schema::users::dsl::*;

    let connection = &mut common::connection::connection();
    diesel::sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let expected_data = vec![
        ("Sean".to_string(), None::<String>),
        ("Tess".to_string(), None::<String>),
    ];
    let actual_data: Vec<_> = users.select((name, hair_color)).load(connection).unwrap();
    assert_eq!(expected_data, actual_data);
}
