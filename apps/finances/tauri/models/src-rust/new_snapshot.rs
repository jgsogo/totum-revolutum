use crate::google_type;
use bigdecimal::BigDecimal;
use finances_accounts::models::NewSnapshot as NewSnapshotDb;
use finances_investments::models::NewSnapshotNumerable as NewSnapshotNumerableDb;
use prost::Message;

use diesel::prelude::*;

pub struct NewSnapshot {
    account_id: i64,
    date_value: chrono::NaiveDate,
    amount: NewSnapshotAmount,
}

impl NewSnapshot {
    pub fn insert_into_db(&self, conn: &mut PgConnection) -> Result<i64, diesel::result::Error> {
        let amount = match &self.amount {
            NewSnapshotAmount::NonNumerable(v) => v.clone(),
            NewSnapshotAmount::Numerable(NewNumerableAmount { quantity, unit_value }) => quantity * unit_value,
        };

        let new_snapshot = NewSnapshotDb {
            account_id: &self.account_id,
            amount: &amount,
            date_value: &self.date_value,
        };

        match &self.amount {
            NewSnapshotAmount::NonNumerable(_) => new_snapshot.insert_into_db(conn),
            NewSnapshotAmount::Numerable(NewNumerableAmount { quantity, unit_value }) => {
                let new_snapshot_numerable = NewSnapshotNumerableDb {
                    new_snapshot: &new_snapshot,
                    quantity: &quantity,
                    unit_value: &unit_value,
                };
                new_snapshot_numerable.insert_into_db(conn)
            }
        }
    }
}

struct NewNumerableAmount {
    quantity: bigdecimal::BigDecimal,
    unit_value: bigdecimal::BigDecimal,
}

enum NewSnapshotAmount {
    Numerable(NewNumerableAmount),
    NonNumerable(bigdecimal::BigDecimal),
}

impl TryFrom<crate::protos::money_amount::NonNumerable> for NewSnapshotAmount {
    type Error = String;

    fn try_from(v: crate::protos::finances_app_models::money_amount::NonNumerable) -> Result<Self, Self::Error> {
        let amount = google_type::Money(v.amount.unwrap());
        let (amount, _): (BigDecimal, String) = amount.into();
        Ok(NewSnapshotAmount::NonNumerable(amount))
    }
}

impl TryFrom<crate::protos::money_amount::Numerable> for NewSnapshotAmount {
    type Error = String;

    fn try_from(v: crate::protos::finances_app_models::money_amount::Numerable) -> Result<Self, Self::Error> {
        let unit_value = google_type::Money(v.unit_value.unwrap());
        let (unit_value, _): (BigDecimal, String) = unit_value.into();
        let quantity = google_type::Decimal(v.quantity.unwrap());
        Ok(NewSnapshotAmount::Numerable(NewNumerableAmount {
            quantity: quantity.try_into().unwrap(),
            unit_value,
        }))
    }
}

impl TryFrom<Vec<u8>> for NewSnapshot {
    type Error = prost::DecodeError;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let proto = crate::protos::NewSnapshot::decode(&*v)?;

        let account_id = proto.account_pk;
        let date_value: chrono::NaiveDate = {
            let date = google_type::Date(proto.date_value.expect("date_value is requried"));
            date.into()
        };

        let amount: NewSnapshotAmount = {
            let amount: crate::protos::finances_app_models::money_amount::Amount = proto
                .amount
                .expect("amount (oneof field) is required")
                .amount
                .expect("amount is required");
            match amount {
                crate::protos::money_amount::Amount::NonNumerable(amount) => {
                    amount.try_into().expect("Every required field should be set")
                }
                crate::protos::money_amount::Amount::Numerable(amount) => {
                    amount.try_into().expect("Every required field should be set")
                }
            }
        };

        Ok(Self {
            account_id,
            date_value,
            amount,
        })
    }
}
