use crate::common::schema::*;
use diesel::*;

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, QueryableByName, Selectable,
)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
}

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, QueryableByName, Selectable,
)]
#[diesel(table_name = tags)]
#[diesel(primary_key(tag))]
pub struct Tag {
    pub tag: String,
    pub parent: Option<String>,
}

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, QueryableByName, Selectable)]
#[diesel(table_name = m2m_posts_tags)]
#[diesel(primary_key(post_id, tag))]
pub struct PostTag {
    pub post_id: i32,
    pub tag: String,
}
