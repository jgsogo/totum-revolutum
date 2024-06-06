mod common;

use diesel::associations::HasTable;
use diesel::*;
use diesel_utils::GetByPk;
#[test]
fn get_by_pk() {
    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let table = common::models::User::table();
    let r = table.find(1);
    let r = r.limit(1);
    let r = r.get_result(connection);
    let _r: common::models::User = r.unwrap();

    let _r = common::models::User::get_by_pk_query(&1);
    let r = common::models::User::get_by_pk(connection, &1);
    let _r: common::models::User = r.unwrap();
    //
    // let expected_data = vec![
    //     ("Sean".to_string(), None::<String>),
    //     ("Tess".to_string(), None::<String>),
    // ];
    // let actual_data: Vec<_> = users.select((name, hair_color)).load(connection).unwrap();
    // assert_eq!(expected_data, actual_data);
}
