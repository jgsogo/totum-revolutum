use finances_app_lib::models::MovementType;
use serde_json::json;
mod common;
use common::call_it;

#[test]
fn test_movement_type() {
    let webview = common::webview();

    // All movement types
    let pk = {
        let body = json!({});
        let r = call_it::<Vec<MovementType>>(&webview, "get_all_movementtypes".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        let movs = r.unwrap();
        assert_eq!(movs.len(), 159);

        // Find a MovementType to use later
        movs.into_iter().find(|m| m.name == "Tasas").unwrap().pk
    };

    // Breadcrumb
    {
        let body = json!({"pk": pk});
        let r = call_it::<Vec<String>>(&webview, "get_breadcrumbs_for_movementtype".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        let breadcrumb = r.unwrap();
        assert_eq!(breadcrumb.len(), 2);
        assert_eq!(breadcrumb, vec!["Tributos".to_string(), "Tasas".to_string()]);
    }
}
