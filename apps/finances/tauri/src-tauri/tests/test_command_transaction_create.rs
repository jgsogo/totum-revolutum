use serde_json::json;

mod common;
use common::call_command;
use finances_app_models::protos::finances_app_models::money_amount::{
    NonNumerable as NonNumerableProto, Numerable as NumerableProto,
};
use finances_app_models::protos::finances_app_models::{
    new_movement as new_movement_proto, Fx as FxProto, NewMovement as NewMovementProto,
    NewTransaction as NewTransactionProto,
};
use finances_app_models::protos::google::r#type::{Date as DateProto, Decimal as DecimalProto, Money as MoneyProto};
use finances_app_models::{AccountContext, MainContext};
use prost::Message;

#[test]
fn test_create_transaction() {
    let webview = common::webview();

    let account_non_numerable = 1i64;
    let account_numerable = 8i64;
    let movement_type_pk = {
        // Find a MovementType to use later
        let body = json!({});
        let r = call_command(&webview, "get_main_context", body.into());
        assert!(r.is_ok());
        let main_context: MainContext = r.unwrap().try_into_proto().unwrap();
        main_context.find_movement_type_by_name("Tasas").unwrap().pk()
    };
    let snapshot_latest_pk = {
        let body = json!({"accountPk": account_numerable});
        let r = call_command(&webview, "get_account_context", body.into());
        assert!(r.is_ok());
        let account_context: AccountContext = r.unwrap().try_into_proto().unwrap();
        account_context.snapshots().into_iter().nth(0).unwrap().pk()
    };

    let date_value = DateProto {
        day: 10,
        month: 12,
        year: 2024,
    };

    {
        let ccy_eur = "EUR";
        let ccy_usd = "USD";

        let transaction = NewTransactionProto {
            name: "New transaction".to_string(),
            description: Some("Some description".to_string()),
            transaction_group_pk: None,
            movements_from: vec![
                // non-numerable movement
                NewMovementProto {
                    account_pk: account_non_numerable,
                    movement_type_pk,
                    date_value: Some(date_value),
                    fx: Some(FxProto {
                        foreign_code: ccy_usd.to_string(),
                        local_code: ccy_eur.to_string(),
                        fx: Some(DecimalProto {
                            value: "2.0".to_string(),
                        }),
                        date_value: Some(date_value),
                    }),
                    amount: Some(new_movement_proto::Amount::NonNumerable(NonNumerableProto {
                        amount: Some(MoneyProto {
                            currency_code: ccy_usd.to_string(),
                            units: 200i64,
                            nanos: 0i32,
                        }),
                    })),
                },
                // dividend movement
                NewMovementProto {
                    account_pk: account_non_numerable,
                    movement_type_pk,
                    date_value: Some(date_value),
                    fx: Some(FxProto {
                        foreign_code: ccy_usd.to_string(),
                        local_code: ccy_eur.to_string(),
                        fx: Some(DecimalProto {
                            value: "0.1".to_string(),
                        }),
                        date_value: Some(date_value),
                    }),
                    amount: Some(new_movement_proto::Amount::Dividend(
                        new_movement_proto::DividendAmount {
                            ex_dividend_date: Some(date_value),
                            ex_dividend_snapshot_pk: snapshot_latest_pk,
                            payout: Some(NumerableProto {
                                unit_value: Some(MoneyProto {
                                    currency_code: ccy_usd.to_string(),
                                    units: 1i64,
                                    nanos: 0i32,
                                }),
                                quantity: Some(DecimalProto { value: "2".to_string() }),
                            }),
                        },
                    )),
                },
            ],
            movements_to: vec![
                // numerable movement
                NewMovementProto {
                    account_pk: account_numerable,
                    movement_type_pk,
                    date_value: Some(date_value),
                    fx: None,
                    amount: Some(new_movement_proto::Amount::Numerable(NumerableProto {
                        unit_value: Some(MoneyProto {
                            currency_code: ccy_eur.to_string(),
                            units: 10i64,
                            nanos: 0i32,
                        }),
                        quantity: Some(DecimalProto {
                            value: "12".to_string(),
                        }),
                    })),
                },
            ],
        };

        let r = call_command(&webview, "create_transaction", transaction.encode_to_vec().into());
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().try_into_json::<f32>().unwrap(), 120f32);
    }

    // Test mismatch amounts
    {
        let transaction = NewTransactionProto {
            name: "New transaction".to_string(),
            description: Some("Some description".to_string()),
            transaction_group_pk: None,
            movements_from: Vec::default(),
            movements_to: vec![NewMovementProto {
                account_pk: account_numerable,
                movement_type_pk,
                date_value: Some(date_value),
                fx: None,
                amount: Some(new_movement_proto::Amount::Numerable(NumerableProto {
                    unit_value: Some(MoneyProto {
                        currency_code: "EUR".to_string(),
                        units: 10i64,
                        nanos: 0i32,
                    }),
                    quantity: Some(DecimalProto {
                        value: "12".to_string(),
                    }),
                })),
            }],
        };

        let r = call_command(&webview, "create_transaction", transaction.encode_to_vec().into());
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err(),
            "Error saving transaction to db: Mismatched amounts, from 0 != to 120"
        );
    }

    // TODO: Wrong movements // invalid checks
}
