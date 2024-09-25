use std::collections::HashMap;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_db::models::{Account, AccountHolder, AccountType};
use finances_db::schema::*;
use itertools::Itertools;
use std::collections::hash_map::Entry;
use tauri::State;

#[derive(serde::Serialize, Debug, PartialEq, Eq)]
pub struct SidebarMenuItem {
    pub name: String,
    pub href: String,
}

impl PartialOrd for SidebarMenuItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SidebarMenuItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

#[derive(serde::Serialize, Debug, PartialEq, Eq)]
pub struct SidebarMenu {
    pub group: String,
    pub entries: Vec<SidebarMenuItem>,
}

impl PartialOrd for SidebarMenu {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SidebarMenu {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.group.cmp(&other.group)
    }
}

fn all_accounts(pool: State<'_, Pool<ConnectionManager<PgConnection>>>) -> Vec<(Account, AccountHolder, AccountType)> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all_with_holder_and_type()
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        // TODO: .filter(data_account::is_closed.eq(false))
        .filter(data_accountholder::owner.eq(0))
        .load::<(Account, AccountHolder, AccountType)>(&mut conn)
        .expect("Error loading accounts")
}

fn checking_accounts(
    pool: State<'_, Pool<ConnectionManager<PgConnection>>>,
) -> Vec<(Account, AccountHolder, AccountType)> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all_with_holder_and_type()
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        // TODO: .filter(data_account::is_closed.eq(false))
        .filter(data_accountholder::owner.eq(0))
        .filter(
            data_accounttype::name
                .eq("Cuenta corriente")
                .or(data_accounttype::name.eq("Metálico")),
        )
        .load::<(Account, AccountHolder, AccountType)>(&mut conn)
        .expect("Error loading accounts")
}

fn investment_accounts(
    pool: State<'_, Pool<ConnectionManager<PgConnection>>>,
) -> Vec<(Account, AccountHolder, AccountType)> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all_with_holder_and_type()
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        // TODO: .filter(data_account::is_closed.eq(false))
        .filter(data_accountholder::owner.eq(0))
        .filter(
            data_accounttype::name
                .eq("Plan de pensiones")
                .or(data_accounttype::name.eq("Fondo de inversión"))
                .or(data_accounttype::name.eq("Acciones"))
                .or(data_accounttype::name.eq("Vivienda")),
        )
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
            entries.sort();
            SidebarMenu { group, entries }
        })
        .collect();
    r.sort();
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
