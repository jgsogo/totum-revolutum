use crate::constants;
use crate::fields::TreeNodeList;
use crate::managers::get_breadcrumbs_for_accounttype;
use crate::models::AccountType;
use crate::sql::filters::acounttype_by_unique_name;
use crate::test_utils::establish_connection;
use diesel::prelude::*;

#[test]
fn test_queries() {
    let pool = establish_connection();
    let mut conn = pool.get().unwrap();

    // All account types
    {
        let all = AccountType::all()
            .select(AccountType::as_select())
            .load::<AccountType>(&mut conn)
            .expect("Error loading account types");

        assert_eq!(all.len(), 15);
    }
}

#[test]
fn test_ancestors() {
    let pool = establish_connection();
    let mut conn = pool.get().unwrap();

    // Check all
    {
        let ancestors = AccountType::all()
            .select((
                crate::schema::finances_accounts_accounttype::tn_ancestors_pks,
                crate::schema::finances_accounts_accounttype::tn_ancestors_count,
            ))
            .load::<(TreeNodeList, i32)>(&mut conn)
            .expect("Error loading account types (ancestors)");

        assert_eq!(ancestors.len(), 15);

        for (treenode_list, count) in ancestors {
            assert_eq!(treenode_list.nodes.len(), count.try_into().unwrap());
        }
    }

    let assets_pk = AccountType::all()
        .filter(acounttype_by_unique_name(constants::accounttype::ASSETS))
        .select(crate::schema::finances_accounts_accounttype::id)
        .get_result::<i64>(&mut conn)
        .unwrap();
    let assets_current_pk = AccountType::all()
        .filter(acounttype_by_unique_name(constants::accounttype::ASSETS_CURRENT))
        .select(crate::schema::finances_accounts_accounttype::id)
        .get_result::<i64>(&mut conn)
        .unwrap();

    // Check know ones: savings
    {
        let bank_account = AccountType::all()
            .filter(acounttype_by_unique_name(
                constants::accounttype::ASSETS_CURRENT_SAVINGS,
            ))
            .select(AccountType::as_select())
            .get_result::<AccountType>(&mut conn)
            .expect("Error fetching ASSETS_CURRENT_SAVINGS");

        assert_eq!(bank_account.tn_ancestors_count, 2);
        assert_eq!(bank_account.tn_ancestors_pks.nodes, &[assets_pk, assets_current_pk]);
    }
}

#[test]
fn test_children() {
    let pool = establish_connection();
    let mut conn = pool.get().unwrap();

    // Check all
    {
        let children = AccountType::all()
            .select((
                crate::schema::finances_accounts_accounttype::tn_children_pks,
                crate::schema::finances_accounts_accounttype::tn_children_count,
            ))
            .load::<(TreeNodeList, i32)>(&mut conn)
            .expect("Error loading account types (children)");

        assert_eq!(children.len(), 15);

        for (treenode_list, count) in children {
            assert_eq!(treenode_list.nodes.len(), count.try_into().unwrap());
        }
    }

    let assets_current_pk = AccountType::all()
        .filter(acounttype_by_unique_name(constants::accounttype::ASSETS_CURRENT))
        .select(crate::schema::finances_accounts_accounttype::id)
        .get_result::<i64>(&mut conn)
        .unwrap();
    let assets_non_current_pk = AccountType::all()
        .filter(acounttype_by_unique_name(constants::accounttype::ASSETS_NON_CURRENT))
        .select(crate::schema::finances_accounts_accounttype::id)
        .get_result::<i64>(&mut conn)
        .unwrap();

    // Check know ones: assets
    {
        let assets = AccountType::all()
            .filter(acounttype_by_unique_name(constants::accounttype::ASSETS))
            .select(AccountType::as_select())
            .get_result::<AccountType>(&mut conn)
            .expect("Error fetching ASSETS");

        assert_eq!(assets.tn_children_count, 2);
        assert_eq!(
            assets.tn_children_pks.nodes,
            &[assets_current_pk, assets_non_current_pk]
        );
    }
}

#[test]
fn test_descendants() {
    let pool = establish_connection();
    let mut conn = pool.get().unwrap();

    // Check all
    {
        let descendants = AccountType::all()
            .select((
                crate::schema::finances_accounts_accounttype::tn_descendants_pks,
                crate::schema::finances_accounts_accounttype::tn_descendants_count,
            ))
            .load::<(TreeNodeList, i32)>(&mut conn)
            .expect("Error loading account types (descendants)");

        assert_eq!(descendants.len(), 15);

        for (treenode_list, count) in descendants {
            assert_eq!(treenode_list.nodes.len(), count.try_into().unwrap());
        }
    }

    let cash_pk = AccountType::all()
        .filter(acounttype_by_unique_name(
            constants::accounttype::ASSETS_CURRENT_SAVINGS,
        ))
        .select(crate::schema::finances_accounts_accounttype::id)
        .get_result::<i64>(&mut conn)
        .unwrap();

    // Check know ones: bank_account
    {
        let assets = AccountType::all()
            .filter(acounttype_by_unique_name(constants::accounttype::ASSETS))
            .select(AccountType::as_select())
            .get_result::<AccountType>(&mut conn)
            .expect("Error fetching ASSETS");

        assert_eq!(assets.tn_descendants_count, 8);
        assert!(assets.tn_descendants_pks.nodes.contains(&cash_pk));
    }
}

#[test]
fn test_breadcrumbs() {
    let pool = establish_connection();
    let mut conn = pool.get().unwrap();

    let assets_current = AccountType::all()
        .filter(acounttype_by_unique_name(constants::accounttype::ASSETS_CURRENT))
        .select(AccountType::as_select())
        .get_result::<AccountType>(&mut conn)
        .unwrap();

    let breadcrumbs = get_breadcrumbs_for_accounttype(&mut conn, &assets_current).unwrap();
    assert_eq!(breadcrumbs.len(), 2);
    assert_eq!(breadcrumbs, ["Activos".to_string(), "Corrientes".to_string()]);
}
