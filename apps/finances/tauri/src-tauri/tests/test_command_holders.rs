use finances_app_lib::models::Holder;
use serde_json::{json, Value};

mod common;
use common::call_it;

#[test]
fn test_holders() -> Result<(), Value> {
    let webview = common::webview();

    {
        let body = json!({});
        let r = call_it::<Vec<Holder>>(&webview, "holders".to_string(), body)?;
        assert_eq!(r.len(), 3);
        {
            let holder = r.get(0).unwrap();
            assert_eq!(holder.name, "holder0");
            assert!(!holder.is_company);
        }
        {
            let holder = r.get(1).unwrap();
            assert_eq!(holder.name, "holder1");
            assert!(!holder.is_company);
        }
        {
            let holder = r.get(2).unwrap();
            assert_eq!(holder.name, "holder2");
            assert!(holder.is_company);
        }
    }

    Ok(())
}
