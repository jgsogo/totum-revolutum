use diesel::associations::HasTable;
use diesel::*;

use common::models::{PostTag, Tag, User};
use diesel_utils::queryset::{FilterByPk, GetByPk};

mod common;

#[test]
fn integer_pk() {
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

#[test]
fn string_pk() {
    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO tags (tag) VALUES ('tag1'), ('tag2')")
        .execute(connection)
        .unwrap();

    // get_by_pk
    let r: Tag = Tag::get_by_pk("tag1").get_result(connection).unwrap();
    assert_eq!(r.tag, "tag1".to_string());
    assert_eq!(r.parent, None);

    // diesel provided tools
    let u: Tag = Tag::table().find("tag1").limit(1).get_result(connection).unwrap();
    assert_eq!(u, r);
}

#[test]
fn tuple_pk() {
    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();
    sql_query("INSERT INTO posts (user_id, title) VALUES (1, 'by Sean'), (1, 'also by Sean')")
        .execute(connection)
        .unwrap();
    sql_query("INSERT INTO tags (tag) VALUES ('tag1'), ('tag2')")
        .execute(connection)
        .unwrap();
    sql_query("INSERT INTO m2m_posts_tags (post_id, tag) VALUES (1, 'tag1'), (1, 'tag2')")
        .execute(connection)
        .unwrap();

    // get_by_pk
    let r: PostTag = PostTag::get_by_pk((1, "tag1")).get_result(connection).unwrap();
    assert_eq!(r.post_id, 1);
    assert_eq!(r.tag, "tag1".to_string());

    // diesel provided tools
    let u: PostTag = PostTag::table()
        .find((1, "tag1"))
        .limit(1)
        .get_result(connection)
        .unwrap();
    assert_eq!(u, r);
}
