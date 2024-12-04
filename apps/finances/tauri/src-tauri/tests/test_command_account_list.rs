use finances_app_lib::models::Account;
use serde_json::json;

mod common;
use common::call_it;

#[test]
fn test_acount_lists() {
    let webview = common::webview();

    // All accounts
    {
        let body = json!({ "holderPk": 0i64 });
        let r = call_it::<Vec<Account>>(&webview, "get_all_accounts_for_holder".to_string(), body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 7);

        let account_names = r.into_iter().map(|acc| acc.name).collect::<Vec<String>>();
        assert_eq!(
            account_names,
            vec![
                "Gastos compartidos",
                "Depósito 3M",
                "Hipoteca casa NY",
                "IBM",
                "Indexa Capital",
                "Plan de pensiones",
                "IBM2",
            ]
        );
    }

    // Savings accounts
    {
        let body = json!({ "holderPk": 0i64 });
        let r = call_it::<Vec<Account>>(&webview, "get_all_savings_accounts_for_holder".to_string(), body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 2);

        let account_names = r.into_iter().map(|acc| acc.name).collect::<Vec<String>>();
        assert_eq!(account_names, vec!["Gastos compartidos", "Depósito 3M"]);
    }

    // Investments accounts
    {
        let body = json!({ "holderPk": 0i64 });
        let r = call_it::<Vec<Account>>(&webview, "get_all_investment_accounts_for_holder".to_string(), body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 3);

        let account_names = r.into_iter().map(|acc| acc.name).collect::<Vec<String>>();
        assert_eq!(account_names, vec!["IBM", "Indexa Capital", "IBM2"]);
    }

    // Retirement accounts
    {
        let body = json!({ "holderPk": 0i64 });
        let r = call_it::<Vec<Account>>(&webview, "get_all_retirement_accounts_for_holder".to_string(), body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 1);

        let account_names = r.into_iter().map(|acc| acc.name).collect::<Vec<String>>();
        assert_eq!(account_names, vec!["Plan de pensiones"]);
    }
}
