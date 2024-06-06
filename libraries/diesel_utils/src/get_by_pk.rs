use crate::error::{Error, ObjectDoesNotExist};
use diesel::associations::HasTable;

use diesel::query_dsl::methods::{FindDsl, LimitDsl};
use diesel::query_dsl::LoadQuery;
use diesel::{Identifiable, RunQueryDsl};

pub type GetByPkOutput<'a, T> =
    <<<&'a T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output as LimitDsl>::Output;

pub trait GetByPk<'a>
where
    &'a Self: Identifiable,
    Self: 'a,
    Self: Sized,
    <&'a Self as HasTable>::Table: FindDsl<<&'a Self as Identifiable>::Id>,
    <<&'a Self as HasTable>::Table as FindDsl<<&'a Self as Identifiable>::Id>>::Output: LimitDsl,
{
    fn get_by_pk_query(pk: <&'a Self as Identifiable>::Id) -> GetByPkOutput<'a, Self> {
        let table = <&'a Self>::table();
        let select_statement = FindDsl::find(table, pk);
        select_statement.limit(1)
    }

    fn get_by_pk<Conn>(conn: &mut Conn, pk: <&'a Self as Identifiable>::Id) -> Result<Self, Error>
    where
        GetByPkOutput<'a, Self>: RunQueryDsl<Conn>,
        GetByPkOutput<'a, Self>: LoadQuery<'a, Conn, Self>,
    {
        let query = Self::get_by_pk_query(pk);
        match query.get_result(conn) {
            Ok(r) => Ok(r),
            Err(diesel::result::Error::NotFound) => {
                Err(Error::ObjectDoesNotExist(ObjectDoesNotExist { model: "".to_string() }))
            }
            Err(e) => Err(e.into()),
        }
    }
}

impl<'a, T> GetByPk<'a> for T
where
    &'a T: Identifiable,
    T: 'a,
    T: Sized,
    <&'a T as HasTable>::Table: FindDsl<<&'a T as Identifiable>::Id>,
    <<&'a T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output: LimitDsl,
{
}

// impl<'a, T> GetByPk<'a> for T
// where
//     &'a T: Identifiable,
//     T: 'a,
//     // &'a T: HasTable,
//     // T::Table: FindDsl<<&'a T as Identifiable>::Id>,
//     <&'a T as HasTable>::Table: FindDsl<<&'a T as Identifiable>::Id>,
//     // <&'a T as HasTable>::Table: LimitDsl
//     <<&'a T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output: LimitDsl, // <<T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output: Table,
//                                                                                             // <<<T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output as AsQuery>::Query: Table,
//                                                                                             // <<<<T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output as AsQuery>::Query as AsQuery>::Query: Table
//                                                                                             // T::Table: Table
//                                                                                             // T: FindDsl<i32>
//                                                                                             // <<T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output: RunQueryDsl<SqliteConnection> + Table
// {
//     type Error = ObjectDoesNotExist;
//     type Output = <<<&'a T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output as LimitDsl>::Output;
//
//     fn get_by_pk(
//         _conn: &mut SqliteConnection,
//         pk: <&'a Self as Identifiable>::Id,
//     ) -> Result<Self::Output, Self::Error> {
//         let table = <&'a T>::table();
//         let select_statement = FindDsl::find(table, pk);
//         let first = select_statement.limit(1);
//         // todo!()
//         Ok(first)
//     }
// }

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
