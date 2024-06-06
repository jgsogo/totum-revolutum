use diesel::associations::HasTable;
use diesel::*;

use common::models::User;
use diesel_utils::GetByPk;

mod common;

#[test]
fn get_by_pk() {
    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    // get_by_pk
    let r: User = User::get_by_pk(1).get_result(connection).unwrap();
    assert_eq!(r.id, 1);
    assert_eq!(r.name, "Sean".to_string());
    assert_eq!(r.hair_color, None);

    // diesel provided tools
    let u: User = User::table().find(1).limit(1).get_result(connection).unwrap();
    assert_eq!(u, r);
}
