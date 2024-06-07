use diesel::*;

use common::models::{PostTag, Tag, User};
use diesel_utils::error::{Error, ObjectDoesNotExist};
mod common;

#[test]
fn queryset_with_integer_pk() {
    use diesel_utils::querysets::GetByPkQuerySet;

    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    // get_by_pk
    let r: User = User::get_by_pk(1).get_result(connection).unwrap();
    assert_eq!(r.id, 1);
    assert_eq!(r.name, "Sean".to_string());
    assert_eq!(r.hair_color, None);
}

#[test]
fn queryset_with_string_pk() {
    use diesel_utils::querysets::GetByPkQuerySet;

    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO tags (tag) VALUES ('tag1'), ('tag2')")
        .execute(connection)
        .unwrap();

    // get_by_pk
    let r: Tag = Tag::get_by_pk("tag1").get_result(connection).unwrap();
    assert_eq!(r.tag, "tag1".to_string());
    assert_eq!(r.parent, None);
}

#[test]
fn queryset_with_tuple_pk() {
    use diesel_utils::querysets::GetByPkQuerySet;

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
}

#[test]
fn manager_with_integer_pk() {
    use diesel_utils::managers::GetByPkManager;

    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    // get_by_pk
    let r: User = User::get_by_pk(1, connection).unwrap();
    assert_eq!(r.id, 1);
    assert_eq!(r.name, "Sean".to_string());
    assert_eq!(r.hair_color, None);

    // test errors
    let r = User::get_by_pk(10, connection);
    assert!(r.is_err());
    assert!(matches!(
        r.unwrap_err(),
        Error::ObjectDoesNotExist(ObjectDoesNotExist{ref model}) if model.ends_with("common::models::User")
    ))
}
