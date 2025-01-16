//! Contains all the Tauri commands that are available to this application.
//!
//! It's useful to have all of them in the same file, because their names (without scope) need
//! to be unique for each Tauri application. We enforce this guarantee if all the commands are
//! defined in the same module.

pub mod snapshot;
pub mod transaction;
use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::fields::MovementDirection;
use finances_accounts::models::{Account, AccountHolder, AccountHolderRole, AccountType, Custodian, Fx, Movement};
use finances_accounts::sql::filters::{accountholder_by_pk, fx_by_pk, movement_filter_account_by_pk};
use finances_app_models::{
    google_type, Account as AppModelAccount, AccountContext, AppState, Fx as AppModelFx, HolderContext, MainContext,
    MoneyAmount, Movement as AppModelMovement, MovementDirection as ModelMovementDirection, OutgoingModel,
    Snapshot as AppModelSnapshot,
};
use finances_investments::models::SnapshotNumerable;
use finances_investments::sql::queries::all_snapshotnumerable_for_account_id;
use prost::Message;
use tauri::ipc::Response;
use tauri::State;

#[tauri::command]
pub async fn get_app_state(state: State<'_, AppState>) -> Result<Response, String> {
    Ok(Response::new(state.as_message().encode_to_vec()))
}

#[tauri::command]
pub async fn get_main_context(main_context: State<'_, MainContext>) -> Result<Response, String> {
    Ok(Response::new(main_context.as_message().encode_to_vec()))
}

#[tauri::command]
pub async fn get_holder_context(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    main_context: State<'_, MainContext>,
    holder_pk: i64,
) -> Result<Response, String> {
    log::info!("Get Holder {holder_pk} context");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let acc_holder = AccountHolder::from_pk(holder_pk, &mut conn).map_err(|e| format!("Error loading holder: {e}"))?;

    let accounts = {
        // Get all accounts for a given holder
        let accounts = Account::all()
            .inner_join(
                finances_accounts::schema::finances_accounts_accountholderrole::table
                    .inner_join(finances_accounts::schema::finances_accounts_accountholder::table),
            )
            .inner_join(Custodian::all())
            .inner_join(AccountType::all())
            .filter(accountholder_by_pk(holder_pk))
            .select((
                Account::as_select(),
                AccountHolderRole::as_select(),
                Custodian::as_select(),
                AccountType::as_select(),
            ))
            .load::<(Account, AccountHolderRole, Custodian, AccountType)>(&mut conn)
            .map_err(|e| format!("Error loading accounts: {}", e))?;

        accounts
            .into_iter()
            .map(|(account, account_holder_role, custodian, account_type)| {
                let account_type = main_context.find_account_type(account_type.id).ok_or(format!(
                    "Account type 'pk={}' not found in main context",
                    account_type.id
                ))?;
                Ok::<_, String>(AppModelAccount::new(
                    account,
                    account_holder_role,
                    custodian.into(),
                    account_type,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    let context = HolderContext::new(acc_holder, accounts);
    Ok(Response::new(context.as_message().encode_to_vec()))
}

#[tauri::command]
pub async fn get_account_context(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    main_context: State<'_, MainContext>,
    account_pk: i64,
) -> Result<Response, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let account = {
        let (account, account_holder_role, custodian, account_type) =
            Account::details_for_pk(account_pk, &mut conn).map_err(|e| format!("Error loading account: {e}"))?;
        let account_type = main_context.find_account_type(account_type.id).ok_or(format!(
            "Account type 'pk={}' not found in main context",
            account_type.id
        ))?;
        AppModelAccount::new(account, account_holder_role, custodian.into(), account_type)
    };

    let account_ccy = account.ccy().to_string();

    // Snapshots
    let snapshots: Vec<AppModelSnapshot> = if !account.is_numerable() {
        Account::snapshots_for_pk(account_pk, &mut conn)
            .map_err(|e| format!("Error retrieving snapshots: {e}"))?
            .into_iter()
            .map(|v| {
                let money: google_type::Money = (v.amount, account_ccy.clone()).into();
                let money_amount = MoneyAmount::new_non_numerable(money);
                AppModelSnapshot::new(v.id, v.date_value.into(), money_amount)
            })
            .collect()
    } else {
        all_snapshotnumerable_for_account_id()
            .bind::<diesel::sql_types::Int8, _>(account_pk)
            .load(&mut conn)
            .map_err(|e| format!("Error loading snapshots numerable: {e}"))?
            .into_iter()
            .map(|v: SnapshotNumerable| {
                let unit_value: google_type::Money = (v.snapshot_numerable.unit_value, account_ccy.clone()).into();
                let quantity: google_type::Decimal = v.snapshot_numerable.quantity.into();
                let money_amount = MoneyAmount::new_numerable(unit_value, quantity);
                AppModelSnapshot::new(v.snapshot.id, v.snapshot.date_value.into(), money_amount)
            })
            .collect()
    };

    // Movements
    let movements: Vec<AppModelMovement> = {
        Movement::all()
            .filter(movement_filter_account_by_pk(account_pk))
            .select(Movement::as_select())
            .load::<Movement>(&mut conn)
            .map_err(|e| format!("Error retrieving movements: {e}"))?
            .into_iter()
            .map(|v| {
                let money: google_type::Money = (v.amount, account_ccy.clone()).into();
                let money_amount = MoneyAmount::new_non_numerable(money);
                let movement_type = main_context
                    .find_movement_type(v.type_id)
                    .cloned()
                    .ok_or(format!("Movement type 'pk={}' not found in main context", v.type_id))?;
                let direction = match v.direction {
                    MovementDirection::In => ModelMovementDirection::In,
                    MovementDirection::Out => ModelMovementDirection::Out,
                };
                let fx = v
                    .fx_id
                    .map(|v| {
                        let fx = Fx::all()
                            .filter(fx_by_pk(v))
                            .select(Fx::as_select())
                            .first::<Fx>(&mut conn)
                            .map_err(|e| format!("Error retrieving movements: {e}"))?;
                        Ok::<_, String>(AppModelFx::new(
                            fx.foreign,
                            fx.local,
                            fx.date_value.into(),
                            fx.rate.into(),
                        ))
                    })
                    .transpose()?;

                Ok::<_, String>(AppModelMovement::new(
                    v.id,
                    v.date_value.into(),
                    v.transaction_id,
                    movement_type,
                    direction,
                    money_amount,
                    fx,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    let context = AccountContext::new(account, movements, snapshots);
    Ok(Response::new(context.as_message().encode_to_vec()))
}
