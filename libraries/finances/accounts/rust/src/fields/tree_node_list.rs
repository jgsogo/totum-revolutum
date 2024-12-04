use diesel::backend::Backend;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::sql_types::Text;
use diesel::FromSqlRow;

// #[repr(i32)]
#[derive(Debug, Clone, PartialEq, FromSqlRow)]
pub struct TreeNodeList {
    pub nodes: Vec<i64>,
}

impl<DB> FromSql<Text, DB> for TreeNodeList
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let binding = String::from_sql(bytes)?;
        let nodes_str = binding.trim();
        if nodes_str.is_empty() {
            return Ok(TreeNodeList { nodes: Vec::default() });
        }

        let v = nodes_str
            .split(",")
            .map(|it| it.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        let treenode = TreeNodeList { nodes: v };
        Ok(treenode)
    }
}
