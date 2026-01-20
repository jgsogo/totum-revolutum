use bigdecimal::BigDecimal;
use finances_app_models::AccountContext;
use serde_json::json;
mod common;
use common::call_command;
use finances_app_models::ProtoWrapper;
use finances_app_models::{google_type, MoneyAmount as MoneyAmountProto, Snapshot as SnapshotProto};
use std::str::FromStr;

#[test]
fn test_snapshot() {
    let webview = common::webview();

    /****
    Non-numerable account
    ***/
    let account_id = 1i64;
    {
        let body = json!({"accountPk": account_id});
        let r = call_command(&webview, "get_account_context", body.into());
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        let account_context: AccountContext = r.unwrap().try_into_proto().unwrap();
        assert_eq!(account_context.snapshots().len(), 0);
    }

    // Snapshot (non-numerable)
    {
        let date_proto = google_type::Date::new(2024, 11, 30).unwrap();

        let amount = MoneyAmountProto::new_non_numerable(
            google_type::Money::new(BigDecimal::from_str("100.00").unwrap(), google_type::CurrencyCode::USD).unwrap(),
        );

        let new_snapshot_proto = SnapshotProto::new(None, date_proto, amount, account_id);
        let r = call_command(&webview, "create_snapshot", new_snapshot_proto.encode_to_vec().into());
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());

        // Now we have one more snapshot
        let body = json!({"accountPk": account_id});
        let r = call_command(&webview, "get_account_context", body.into());
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        let account_context: AccountContext = r.unwrap().try_into_proto().unwrap();
        assert_eq!(account_context.snapshots().len(), 1);
    }

    /****
    Numerable account
    ***/
    let account_id = 7i64;
    {
        let body = json!({"accountPk": account_id});
        let r = call_command(&webview, "get_account_context", body.into());
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        let account_context: AccountContext = r.unwrap().try_into_proto().unwrap();
        assert_eq!(account_context.snapshots().len(), 0);
    }

    // Snapshot (numerable)
    {
        let date_proto = google_type::Date::new(2024, 11, 30).unwrap();

        let amount = MoneyAmountProto::new_numerable(
            google_type::Money::new(BigDecimal::from_str("100.00").unwrap(), google_type::CurrencyCode::USD).unwrap(),
            google_type::Decimal::new(BigDecimal::from_str("3").unwrap()),
        );

        let new_snapshot_proto = SnapshotProto::new(None, date_proto, amount, account_id);
        let r = call_command(&webview, "create_snapshot", new_snapshot_proto.encode_to_vec().into());
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());

        // Now we have one more snapshot
        let body = json!({"accountPk": account_id});
        let r = call_command(&webview, "get_account_context", body.into());
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        let account_context: AccountContext = r.unwrap().try_into_proto().unwrap();
        assert_eq!(account_context.snapshots().len(), 1);
    }
}
