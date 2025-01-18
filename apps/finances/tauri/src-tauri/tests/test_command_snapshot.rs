use finances_app_models::AccountContext;
use serde_json::json;
mod common;
use common::call_command;
use finances_app_models::protos::finances_app_models::money_amount::{
    Amount as AmountProto, NonNumerable as NonNumerableProto, Numerable as NumerableProto,
};
use finances_app_models::protos::finances_app_models::{
    MoneyAmount as MoneyAmountProto, NewSnapshot as NewSnapshotProto,
};
use finances_app_models::protos::google::r#type::{Date as DateProto, Decimal as DecimalProto, Money as MoneyProto};
use prost::Message;

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
        assert!(r.is_ok());
        let account_context: AccountContext = r.unwrap().try_into_proto().unwrap();
        assert_eq!(account_context.snapshots().len(), 0);
    }

    // Snapshot (non-numerable)
    {
        let date_proto = DateProto {
            day: 30,
            month: 11,
            year: 2024,
        };

        let amount = MoneyAmountProto {
            amount: Some(AmountProto::NonNumerable(NonNumerableProto {
                amount: Some(MoneyProto {
                    currency_code: "USD".to_string(),
                    units: 100i64,
                    nanos: 0i32,
                }),
            })),
        };

        let new_snapshot_proto = NewSnapshotProto {
            account_pk: account_id,
            amount: Some(amount),
            date_value: Some(date_proto),
        };
        let r = call_command(&webview, "create_snapshot", new_snapshot_proto.encode_to_vec().into());
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());

        // Now we have one more snapshot
        let body = json!({"accountPk": account_id});
        let r = call_command(&webview, "get_account_context", body.into());
        assert!(r.is_ok());
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
        assert!(r.is_ok());
        let account_context: AccountContext = r.unwrap().try_into_proto().unwrap();
        assert_eq!(account_context.snapshots().len(), 0);
    }

    // Snapshot (numerable)
    {
        let date_proto = DateProto {
            day: 30,
            month: 11,
            year: 2024,
        };

        let amount = MoneyAmountProto {
            amount: Some(AmountProto::Numerable(NumerableProto {
                unit_value: Some(MoneyProto {
                    currency_code: "USD".to_string(),
                    units: 100i64,
                    nanos: 0i32,
                }),
                quantity: Some(DecimalProto { value: "3".to_string() }),
            })),
        };

        let new_snapshot_proto = NewSnapshotProto {
            account_pk: account_id,
            amount: Some(amount),
            date_value: Some(date_proto),
        };
        let r = call_command(&webview, "create_snapshot", new_snapshot_proto.encode_to_vec().into());
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());

        // Now we have one more snapshot
        let body = json!({"accountPk": account_id});
        let r = call_command(&webview, "get_account_context", body.into());
        assert!(r.is_ok());
        let account_context: AccountContext = r.unwrap().try_into_proto().unwrap();
        assert_eq!(account_context.snapshots().len(), 1);
    }
}
