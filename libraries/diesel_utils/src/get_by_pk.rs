use crate::error::ObjectDoesNotExist;
use diesel::associations::HasTable;

use diesel::query_dsl::methods::{FindDsl, LimitDsl};
use diesel::{Identifiable, SqliteConnection};

pub trait GetByPk<'a>
where
    &'a Self: Identifiable,
    Self: 'a,
    Self: Sized,
{
    type Error: From<ObjectDoesNotExist>;
    type Output;

    fn get_by_pk(conn: &mut SqliteConnection, pk: <&'a Self as Identifiable>::Id) -> Result<Self::Output, Self::Error>;
}

impl<'a, T> GetByPk<'a> for T
where
    &'a T: Identifiable,
    T: 'a,
    &'a T: HasTable,
    // T::Table: FindDsl<<&'a T as Identifiable>::Id>,
    <&'a T as HasTable>::Table: FindDsl<<&'a T as Identifiable>::Id>,
    // <&'a T as HasTable>::Table: LimitDsl
    <<&'a T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output: LimitDsl, // <<T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output: Table,
                                                                                            // <<<T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output as AsQuery>::Query: Table,
                                                                                            // <<<<T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output as AsQuery>::Query as AsQuery>::Query: Table
                                                                                            // T::Table: Table
                                                                                            // T: FindDsl<i32>
                                                                                            // <<T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output: RunQueryDsl<SqliteConnection> + Table
{
    type Error = ObjectDoesNotExist;
    type Output = <<<&'a T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output as LimitDsl>::Output;

    fn get_by_pk(
        _conn: &mut SqliteConnection,
        pk: <&'a Self as Identifiable>::Id,
    ) -> Result<Self::Output, Self::Error> {
        let table = <&'a T>::table();
        let select_statement = FindDsl::find(table, pk);
        let first = select_statement.limit(1);
        // todo!()
        Ok(first)
    }
}

// pub trait GetByPk<'a>: Sized
//     where
//         Self: 'a,
//         &'a Self: Identifiable,
// {
//     type Error;
//
//     fn get_by_pk(conn: &mut SqliteConnection, pk: <&'a Self as Identifiable>::Id) -> Result<Self, Self::Error>;
//
//     // fn filter_by_pk(
//     //     conn: &mut SqliteConnection,
//     //     pk: &[<&'a Self as Identifiable>::Id],
//     // ) -> Result<Vec<Self>, Self::Error>;
// }
//
// impl<'a, T: HasTable + 'a> GetByPk<'a> for T
// where
//     &'a Self: Identifiable
// {
//     fn get_by_pk(conn: &mut SqliteConnection, pk: <&'a Self as Identifiable>::Id) -> Result<Self, Self::Error> {
//         Ok(Self::table().find(pk).first(conn)?)
//     }
// }
//
// impl GetByPk<'_> for App {
//     fn get_by_pk(conn: &mut SqliteConnection, pk: <&Self as Identifiable>::Id) -> Result<Self> {
//         Ok(App::table().find(pk).first(conn)?)
//     }
//
//     fn filter_by_pk(
//         conn: &mut SqliteConnection,
//         pks: &[<&'_ Self as Identifiable>::Id],
//     ) -> Result<Vec<Self>> {
//         Ok(App::table()
//             .filter(schema::apps_apps::resource_id.eq_any(pks))
//             .load(conn)?)
//     }
// }
