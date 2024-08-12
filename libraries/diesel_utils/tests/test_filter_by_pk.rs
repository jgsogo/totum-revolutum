use crate::common::models::{Tag, User};
use diesel::*;
use diesel_utils::error::{Error, ObjectDoesNotExist};
use log::Level;

mod common;

#[test]
fn queryset_with_integer_pk() {
    use diesel_utils::querysets::FilterByPkQuerySet;

    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let user_ids = vec![1, 2];

    // filter_by_pk
    let actual_data = User::filter_by_pk(&user_ids).load::<User>(connection).unwrap();
    let actual_data = actual_data.into_iter().map(|u| u.id).collect::<Vec<_>>();
    assert_eq!(user_ids, actual_data);
}

#[test]
fn queryset_with_string_pk() {
    use diesel_utils::querysets::FilterByPkQuerySet;

    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO tags (tag) VALUES ('tag1'), ('tag2')")
        .execute(connection)
        .unwrap();

    let tags = vec!["tag1", "tag2"];

    // filter_by_pk
    let actual_data = Tag::filter_by_pk(&tags).load::<Tag>(connection).unwrap();
    let actual_data = actual_data.into_iter().map(|u| u.tag).collect::<Vec<_>>();
    assert_eq!(tags, actual_data);
}

#[test]
fn queryset_with_empty_values() {
    // If the input pks is empty, it doesn't fail, it just returns an empty vector.
    use diesel_utils::querysets::FilterByPkQuerySet;

    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let user_ids: Vec<i32> = vec![];
    let actual_data = User::filter_by_pk(&user_ids).load::<User>(connection).unwrap();
    assert!(actual_data.is_empty());
}

#[test]
fn queryset_with_missing_values() {
    // If some input PKs are missing, the queryset will build the query, it's up to the consumer to
    // realize that it was expecting a different number of results.
    use diesel_utils::querysets::FilterByPkQuerySet;

    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let user_ids = vec![1, 50, 60];
    let actual_data = User::filter_by_pk(&user_ids).load::<User>(connection).unwrap();
    assert_eq!(actual_data.len(), 1);
}

#[test]
fn queryset_with_duplicated_values() {
    // If some input PKs are duplicated, the queryset will build the query, it's up to the consumer
    // to realize about it. In this case, there are no missing values, SQL backend just deduplicates
    // them.
    use diesel_utils::querysets::FilterByPkQuerySet;

    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let user_ids = vec![1, 1, 1];
    let actual_data = User::filter_by_pk(&user_ids).load::<User>(connection).unwrap();
    assert_eq!(actual_data.len(), 1);
}

#[test]
fn manager_with_integer_pk() {
    use diesel_utils::managers::FilterByPkManager;

    let connection = &mut common::connection::connection();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let user_ids = vec![1, 2];

    // filter_by_pk
    let actual_data = User::filter_by_pk(&user_ids, connection).unwrap(); // FIXME: Pass `user_ids` by reference
    let actual_data = actual_data.into_iter().map(|u| u.id).collect::<Vec<_>>();
    assert_eq!(user_ids, actual_data);

    // test errors -- if there are fewer results than input PKs, the Manager will raise
    let r = User::filter_by_pk(vec![1, 2, 50, 60], connection);
    assert!(r.is_err());
    assert!(matches!(
        r.unwrap_err(),
        Error::ObjectDoesNotExist(ObjectDoesNotExist{ref model}) if model.ends_with("common::models::User")
    ));

    // test warn -- Duplicated PKs are removed and we get a warning
    testing_logger::setup();
    let r = User::filter_by_pk(vec![1, 1, 1], connection);
    assert!(r.is_ok());
    assert_eq!(r.unwrap().len(), 1);
    testing_logger::validate(|captured_logs| {
        assert_eq!(captured_logs.len(), 1);
        assert_eq!(captured_logs[0].body, "Duplicated pks removed");
        assert_eq!(captured_logs[0].level, Level::Warn);
    });
}
