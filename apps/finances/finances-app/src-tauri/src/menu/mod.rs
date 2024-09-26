use std::collections::HashMap;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_db::models::{Account, AccountHolder, AccountType};
use itertools::Itertools;
use std::collections::hash_map::Entry;
use tauri::State;

#[derive(serde::Serialize, Debug, PartialEq, Eq)]
pub struct SidebarMenuItem {
    pub name: String,
    pub href: String,
}

#[derive(serde::Serialize, Debug, PartialEq, Eq)]
pub struct SidebarMenu {
    pub group: String,
    pub entries: Vec<SidebarMenuItem>,
}

fn all_accounts(pool: State<'_, Pool<ConnectionManager<PgConnection>>>) -> Vec<(Account, AccountHolder, AccountType)> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all_with_holder_and_type()
        .filter(Account::opened())
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        .filter(Account::mine())
        .load::<(Account, AccountHolder, AccountType)>(&mut conn)
        .expect("Error loading accounts")
}

fn checking_accounts(
    pool: State<'_, Pool<ConnectionManager<PgConnection>>>,
) -> Vec<(Account, AccountHolder, AccountType)> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all_with_holder_and_type()
        .filter(Account::opened())
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        .filter(Account::mine())
        .filter(Account::checking_account())
        .load::<(Account, AccountHolder, AccountType)>(&mut conn)
        .expect("Error loading accounts")
}

fn investment_accounts(
    pool: State<'_, Pool<ConnectionManager<PgConnection>>>,
) -> Vec<(Account, AccountHolder, AccountType)> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all_with_holder_and_type()
        .filter(Account::opened())
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        .filter(Account::mine())
        .filter(Account::investment())
        .load::<(Account, AccountHolder, AccountType)>(&mut conn)
        .expect("Error loading accounts")
}

fn group_by_account_holder(accounts: Vec<(Account, AccountHolder, AccountType)>) -> Vec<SidebarMenu> {
    let mut r: HashMap<String, Vec<SidebarMenuItem>> = HashMap::new();
    for (key, chunk) in &accounts.into_iter().chunk_by(|(_, holder, _)| holder.name.clone()) {
        let mut entries: Vec<SidebarMenuItem> = chunk
            .into_iter()
            .map(|(account, _, _)| SidebarMenuItem {
                name: account.name,
                href: "href".into(),
            })
            .collect();
        match r.entry(key) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().append(&mut entries);
            }
            Entry::Vacant(entry) => {
                entry.insert(entries);
            }
        }
    }

    let mut r: Vec<SidebarMenu> = r
        .into_iter()
        .map(|(group, mut entries)| {
            entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            SidebarMenu { group, entries }
        })
        .collect();
    r.sort_by(|a, b| a.group.to_lowercase().cmp(&b.group.to_lowercase()));
    r
}

#[tauri::command]
pub async fn sidebar_menu(
    pool: State<'_, Pool<ConnectionManager<PgConnection>>>,
    category: &str,
) -> Result<Vec<SidebarMenu>, String> {
    log::info!("Get Accounts for category {category}");

    if category == "/all" {
        let all_accounts = all_accounts(pool);
        let r = group_by_account_holder(all_accounts);
        Ok(r)
    } else if category == "/accounts" {
        let checking_accounts = checking_accounts(pool);
        let r = group_by_account_holder(checking_accounts);
        Ok(r)
    } else if category == "/investments" {
        let investment_accounts = investment_accounts(pool);
        let r = group_by_account_holder(investment_accounts);
        Ok(r)
    } else if category == "/rentals" {
        // TODO: Return links to views about rented properties
        Ok(vec![])
    } else if category == "/taxes" {
        // TODO: Return links to views about taxes: IRPF, 720,...
        Ok(vec![])
    } else {
        log::error!("Unexpected sidebar_menu category '{category}'");
        // FIXME: Return error?
        Ok(vec![])
    }
}
