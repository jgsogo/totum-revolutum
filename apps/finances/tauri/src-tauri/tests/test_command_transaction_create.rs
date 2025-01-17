use finances_app_lib::models::{NewAmount, NewMovement, NewMovementType, NewTransaction};
use serde_json::json;

mod common;
use common::call_command;
use finances_app_models::{AccountContext, MainContext};

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

    {
        let transaction = NewTransaction {
            name: "New transaction".to_string(),
            description: Some("Some description".to_string()),
            transaction_group_pk: None,
            movements_from: vec![
                // non-numerable movement
                NewMovement {
                    account_pk: account_non_numerable,
                    movement_type_pk,
                    date_value: "2024-12-10".to_string(),
                    amount: NewAmount {
                        amount: Some(200f32),
                        quantity: None,
                        unit_value: None,
                    },
                    fx: Some(2f32),
                    r#type: NewMovementType::NonNumerable,
                    ex_dividend_date: None,
                    ex_dividend_snapshot_pk: None,
                },
                // dividend movement
                NewMovement {
                    account_pk: account_numerable,
                    movement_type_pk,
                    date_value: "2024-12-10".to_string(),
                    amount: NewAmount {
                        amount: None,
                        quantity: None,
                        unit_value: Some(1f32),
                    },
                    fx: Some(0.1f32),
                    r#type: NewMovementType::Dividend,
                    ex_dividend_date: Some("2024-12-10".to_string()),
                    ex_dividend_snapshot_pk: Some(snapshot_latest_pk),
                },
            ],
            movements_to: vec![
                // numerable movement
                NewMovement {
                    account_pk: account_numerable,
                    movement_type_pk,
                    date_value: "2024-12-10".to_string(),
                    amount: NewAmount {
                        amount: None,
                        quantity: Some(12f32),
                        unit_value: Some(10f32),
                    },
                    fx: None,
                    r#type: NewMovementType::Numerable,
                    ex_dividend_date: None,
                    ex_dividend_snapshot_pk: None,
                },
            ],
        };

        let body = serde_json::to_value(transaction).unwrap();
        let r = call_command(&webview, "create_transaction", json!({ "transaction": body }).into());
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().try_into_json::<f32>().unwrap(), 120f32);
    }

    // Test mismatch amounts
    {
        let transaction = NewTransaction {
            name: "New transaction".to_string(),
            description: Some("Some description".to_string()),
            transaction_group_pk: None,
            movements_from: vec![],
            movements_to: vec![NewMovement {
                account_pk: account_numerable,
                movement_type_pk,
                date_value: "2024-12-10".to_string(),
                amount: NewAmount {
                    amount: None,
                    quantity: Some(10f32),
                    unit_value: Some(10f32),
                },
                fx: None,
                r#type: NewMovementType::Numerable,
                ex_dividend_date: None,
                ex_dividend_snapshot_pk: None,
            }],
        };

        let body = serde_json::to_value(transaction).unwrap();
        let r = call_command(&webview, "create_transaction", json!({ "transaction": body }).into());
        assert!(r.is_err());
        assert_eq!(r.unwrap_err(), "Mismatched amounts, from 0 != to 100");
    }

    // TODO: Wrong movements // invalid checks
}
