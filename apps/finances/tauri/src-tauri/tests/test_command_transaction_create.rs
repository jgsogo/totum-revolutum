use bigdecimal::BigDecimal;
use serde_json::json;

mod common;
use common::call_command;
use finances_app_models::ProtoWrapper;
use finances_app_models::{
    google_type, FxQuote as FxQuoteProto, FxQuotePair as FxQuotePairProto, MainContext as MainContextProto,
    Movement as MovementProto, MovementAmount as MovementAmountProto, MovementDirection as MovementDirectionProto,
    Transaction as TransactionProto,
};
use std::str::FromStr;

#[test]
fn test_create_transaction() {
    let webview = common::webview();

    let account_non_numerable = 1i64;
    let account_numerable = 8i64;
    let movement_type = {
        // Find a MovementType to use later
        let body = json!({});
        let r = call_command(&webview, "get_main_context", body.into());
        assert!(r.is_ok());
        let main_context: MainContextProto = r.unwrap().try_into_proto().unwrap();
        main_context.find_movement_type_by_name("Tasas").unwrap().clone()
    };

    let date_proto = google_type::Date::new(2024, 12, 10).unwrap();

    {
        let transaction = TransactionProto::new(
            None,
            "New transaction".to_string(),
            Some("Some description".to_string()),
            None,
            vec![
                // non-numerable movement
                MovementProto::new(
                    None,
                    date_proto.clone(),
                    None,
                    movement_type.clone(),
                    MovementDirectionProto::out(),
                    MovementAmountProto::new_non_numerable(
                        google_type::Money::new(
                            BigDecimal::from_str("200.00").unwrap(),
                            google_type::CurrencyCode::USD,
                        )
                        .unwrap(),
                    ),
                    Some(FxQuoteProto::new(
                        FxQuotePairProto::new(google_type::CurrencyCode::EUR, google_type::CurrencyCode::USD).unwrap(),
                        date_proto.clone(),
                        google_type::Decimal::new(BigDecimal::from_str("2").unwrap()),
                    )),
                    account_non_numerable,
                ),
                // dividend movement
                MovementProto::new(
                    None,
                    date_proto.clone(),
                    None,
                    movement_type.clone(),
                    MovementDirectionProto::out(),
                    MovementAmountProto::new_dividend(
                        date_proto.clone(),
                        google_type::Money::new(BigDecimal::from_str("1.00").unwrap(), google_type::CurrencyCode::USD)
                            .unwrap(),
                        google_type::Decimal::new(BigDecimal::from_str("2").unwrap()),
                    ),
                    Some(FxQuoteProto::new(
                        FxQuotePairProto::new(google_type::CurrencyCode::EUR, google_type::CurrencyCode::USD).unwrap(),
                        date_proto.clone(),
                        google_type::Decimal::new(BigDecimal::from_str("0.1").unwrap()),
                    )),
                    account_non_numerable,
                ),
            ],
            vec![
                // numerable movement
                MovementProto::new(
                    None,
                    date_proto.clone(),
                    None,
                    movement_type.clone(),
                    MovementDirectionProto::r#in(),
                    MovementAmountProto::new_numerable(
                        google_type::Money::new(BigDecimal::from_str("10.00").unwrap(), google_type::CurrencyCode::EUR)
                            .unwrap(),
                        google_type::Decimal::new(BigDecimal::from_str("12").unwrap()),
                    ),
                    None,
                    account_numerable,
                ),
            ],
        );

        let r = call_command(&webview, "create_transaction", transaction.encode_to_vec().into());
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().try_into_json::<f32>().unwrap(), 120f32);
    }

    // Test mismatch amounts
    {
        let transaction = TransactionProto::new(
            None,
            "New transaction".to_string(),
            Some("Some description".to_string()),
            None,
            Vec::new(),
            vec![
                // numerable movement
                MovementProto::new(
                    None,
                    date_proto.clone(),
                    None,
                    movement_type.clone(),
                    MovementDirectionProto::out(),
                    MovementAmountProto::new_numerable(
                        google_type::Money::new(BigDecimal::from_str("10.00").unwrap(), google_type::CurrencyCode::EUR)
                            .unwrap(),
                        google_type::Decimal::new(BigDecimal::from_str("12").unwrap()),
                    ),
                    None,
                    account_numerable,
                ),
            ],
        );

        let r = call_command(&webview, "create_transaction", transaction.encode_to_vec().into());
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().to_string(),
            "{\"message\":\"Mismatched amounts, from 0 != to 120\"}"
        );
    }

    // TODO: Wrong movements // invalid checks
}
