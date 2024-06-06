mod common;

use crate::common::models::User;
use crate::common::schema::users;
use diesel::associations::HasTable;
use diesel::*;
use diesel_utils::GetByPk;

#[test]
fn filter_by_pk() {
    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let user_ids = vec![1, 2];

    // filter_by_pk
    // let _r = common::models::User::filter_by_pk(&user_ids);

    // diesel provided tools
    let actual_data = User::table()
        .filter(users::id.eq_any(&user_ids))
        .load::<User>(connection)
        .unwrap();
    let actual_data = actual_data.into_iter().map(|u| u.id).collect::<Vec<_>>();
    assert_eq!(user_ids, actual_data);
}
