use crate::NewMovement;
use bigdecimal::BigDecimal;
use bigdecimal::Zero;
use finances_accounts::fields::MovementDirection;

use finances_accounts::models::NewTransaction as NewTransactionDb;

use prost::Message;

use diesel::prelude::*;

pub struct NewTransaction {
    name: String,
    description: Option<String>,
    transaction_group_pk: Option<i64>,

    movements_from: Vec<NewMovement>,
    movements_to: Vec<NewMovement>,
}

impl NewTransaction {
    /// Inserts the transaction into the database and returns the amount-sum of all the movements (in one direction)
    pub fn insert_into_db(&self, conn: &mut PgConnection, base_ccy: &str) -> Result<BigDecimal, String> {
        let (total_from, total_to) = conn
            .transaction(|conn| {
                // Create the transaction
                let transaction_pk = {
                    let new_transaction = NewTransactionDb {
                        name: &self.name,
                        description: self.description.as_deref(),
                        group_id: self.transaction_group_pk.as_ref(),
                    };
                    new_transaction.insert_into_db(conn)?
                };

                let total_from = self
                    .movements_from
                    .iter()
                    .map(|mov| mov.insert_into_db(conn, transaction_pk, MovementDirection::Out, base_ccy))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .fold(BigDecimal::zero(), |sum, mov_amount| sum + mov_amount);

                let total_to = self
                    .movements_to
                    .iter()
                    .map(|mov| mov.insert_into_db(conn, transaction_pk, MovementDirection::In, base_ccy))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .fold(BigDecimal::zero(), |sum, mov_amount| sum + mov_amount);

                Ok((total_from, total_to))
            })
            .map_err(|e: diesel::result::Error| {
                format!("Error inserting Transaction and movements in the database: {e}")
            })?;

        if !finances_accounts::types::compare_eq(&total_from, &total_to) {
            Err(format!("Mismatched amounts, from {total_from} != to {total_to}"))
        } else {
            Ok(total_from)
        }
    }
}

impl TryFrom<Vec<u8>> for NewTransaction {
    type Error = String;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let proto = crate::protos::finances_app_models::NewTransaction::decode(&*v)
            .map_err(|e| format!("Error decoding the protobuf bytes for NewTransaction: {e}"))?;

        Ok(Self {
            name: proto.name,
            description: proto.description,
            transaction_group_pk: proto.transaction_group_pk,

            movements_from: proto
                .movements_from
                .into_iter()
                .map(|v| v.try_into())
                .collect::<Result<Vec<_>, _>>()?,
            movements_to: proto
                .movements_to
                .into_iter()
                .map(|v| v.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
