//! Utilities to work with Diesel

use diesel::{Identifiable, SqliteConnection};

pub trait GetByPk<'a>: Sized
where
    Self: 'a,
    &'a Self: Identifiable,
{
    type Error;

    fn get_by_pk(conn: &mut SqliteConnection, pk: <&'a Self as Identifiable>::Id) -> Result<Self, Self::Error>;

    fn filter_by_pk(
        conn: &mut SqliteConnection,
        pk: &[<&'a Self as Identifiable>::Id],
    ) -> Result<Vec<Self>, Self::Error>;
}
