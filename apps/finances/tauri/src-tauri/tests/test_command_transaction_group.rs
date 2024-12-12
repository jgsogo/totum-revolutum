use finances_app_lib::models::TransactionGroup;
use serde_json::json;

mod common;
use common::call_it;

#[test]
fn test_transaction_group() {
    let webview = common::webview();

    // All transaction groups
    {
        let body = json!({});
        let r = call_it::<Vec<TransactionGroup>>(&webview, "get_all_transaction_groups".to_string(), body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 2);

        let names = r.into_iter().map(|tg| tg.name).collect::<Vec<String>>();
        assert_eq!(names, vec!["transaction_group0", "transaction_group1",]);
    }
}
