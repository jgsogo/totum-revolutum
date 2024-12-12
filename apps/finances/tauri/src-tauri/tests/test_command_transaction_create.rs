use finances_app_lib::models::{MovementType, NewAmount, NewMovement, NewTransaction};
use serde_json::json;

mod common;
use common::call_it;

#[test]
fn test_create_transaction() {
    let webview = common::webview();

    let account_non_numerable = 1i64;
    let account_numerable = 8i64;
    let movement_type_pk = {
        // Find a MovementType to use later
        let body = json!({});
        let r = call_it::<Vec<MovementType>>(&webview, "get_all_movementtypes".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        let movs = r.unwrap();
        movs.into_iter().find(|m| m.name == "Tasas").unwrap().pk
    };

    let transaction = NewTransaction {
        name: "New transaction".to_string(),
        description: Some("Some description".to_string()),
        transaction_group_pk: None,
        movements_from: vec![NewMovement {
            account_pk: account_non_numerable,
            movement_type_pk: movement_type_pk,
            date_value: "2024-12-10".to_string(),
            amount: NewAmount {
                amount: Some(100f32),
                quantity: None,
                unit_value: None,
            },
            fx: None, // TODO: Add FX
        }],
        movements_to: vec![NewMovement {
            account_pk: account_numerable,
            movement_type_pk: movement_type_pk,
            date_value: "2024-12-10".to_string(),
            amount: NewAmount {
                amount: None,
                quantity: Some(10f32),
                unit_value: Some(10f32),
            },
            fx: None,
        }],
    };

    let body = serde_json::to_value(transaction).unwrap();
    let r = call_it::<usize>(
        &webview,
        "create_transaction".to_string(),
        json!({ "transaction": body }),
    );
    assert!(r.is_ok(), "Error: {}", r.unwrap_err());

    // TODO: Test mismatch amounts
}
