use diesel::associations::HasTable;
use diesel::*;

use crate::common::models::{Tag, User};
use crate::common::schema::{tags, users};

mod common;

#[test]
fn queryset_with_integer_pk() {
    use diesel_utils::queryset::FilterByPk;

    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let user_ids = vec![1, 2];

    // filter_by_pk
    let actual_data = User::filter_by_pk(&user_ids).load::<User>(connection).unwrap();
    let actual_data = actual_data.into_iter().map(|u| u.id).collect::<Vec<_>>();
    assert_eq!(user_ids, actual_data);

    // diesel provided tools
    let actual_data = User::table()
        .filter(users::id.eq_any(&user_ids))
        .load::<User>(connection)
        .unwrap();
    let actual_data = actual_data.into_iter().map(|u| u.id).collect::<Vec<_>>();
    assert_eq!(user_ids, actual_data);
}

#[test]
fn queryset_with_string_pk() {
    use diesel_utils::queryset::FilterByPk;

    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO tags (tag) VALUES ('tag1'), ('tag2')")
        .execute(connection)
        .unwrap();

    let tags = vec!["tag1", "tag2"];

    // filter_by_pk
    let actual_data = Tag::filter_by_pk(&tags).load::<Tag>(connection).unwrap();
    let actual_data = actual_data.into_iter().map(|u| u.tag).collect::<Vec<_>>();
    assert_eq!(tags, actual_data);

    // diesel provided tools
    let actual_data = Tag::table()
        .filter(tags::tag.eq_any(&tags))
        .load::<Tag>(connection)
        .unwrap();
    let actual_data = actual_data.into_iter().map(|u| u.tag).collect::<Vec<_>>();
    assert_eq!(tags, actual_data);
}
