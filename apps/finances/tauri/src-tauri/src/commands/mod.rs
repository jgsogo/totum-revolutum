//! Contains all the Tauri commands that are available to this application.
//!
//! It's useful to have all of them in the same file, because their names (without scope) need
//! to be unique for each Tauri application. We enforce this guarantee if all the commands are
//! defined in the same module.

pub mod last_transactions;
pub mod snapshot;
pub mod transaction;
use crate::types::ConnectionType;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::{Account, AccountHolder, AccountHolderRole, AccountType, Custodian, Fx};
use finances_accounts::sql::filters::{accountholder_by_pk, fx_by_pk};
use finances_app_models::{
    google_type, Account as AccountProto, AccountContext as AccountContextProto, AppState as AppStateProto,
    FxQuote as FxQuoteProto, FxQuotePair as FxQuotePairProto, Holder as HolderProto,
    HolderContext as HolderContextProto, MainContext as MainContextProto, MoneyAmount as MoneyAmountProto,
    Movement as MovementProto, MovementAmount as MovementAmountProto, MovementDirection as MovementDirectionProto,
    ProtoWrapper, Snapshot as SnapshotProto,
};
use finances_investments::models::Movement;

use finances_investments::sql::queries::all_movements_for_account_id;

use crate::{Error, Result};
use finances_investments::models::SnapshotNumerable;
use finances_investments::sql::queries::all_snapshotnumerable_for_account_id;
use tauri::ipc::Response;
use tauri::State;

#[tauri::command]
pub async fn get_app_state(state: State<'_, AppStateProto>) -> Result<Response> {
    Ok(Response::new(state.encode_to_vec()))
}

#[tauri::command]
pub async fn get_main_context(main_context: State<'_, MainContextProto>) -> Result<Response> {
    Ok(Response::new(main_context.encode_to_vec()))
}

#[tauri::command]
pub async fn get_holder_context(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    main_context: State<'_, MainContextProto>,
    holder_pk: i64,
) -> Result<Response> {
    log::info!("Get Holder {holder_pk} context");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let acc_holder = {
        let v = AccountHolder::from_pk(holder_pk, &mut conn)?;
        HolderProto::new(v.id, v.name, v.is_company, v.photo)
    };

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
            .load::<(Account, AccountHolderRole, Custodian, AccountType)>(&mut conn)?;

        accounts
            .into_iter()
            .map(|(account, account_holder_role, custodian, account_type)| {
                let account_type = main_context
                    .find_account_type(account_type.id)
                    .ok_or(Error::Other(format!(
                        "Account type 'pk={}' not found in main context",
                        account_type.id
                    )))?;

                // FIXME: We only want the last snapshot
                let snapshots = get_snapshots(&account, &mut conn)?;
                let last_snapshot = snapshots.get(0).cloned();

                Ok::<_, Error>(AccountProto::new(
                    account.id,
                    account.name,
                    custodian.into(),
                    account_type.clone(),
                    google_type::CurrencyCode::new(&account.ccy)?,
                    account.identifier,
                    account.description,
                    account.open.into(),
                    account_holder_role.owns_money,
                    account.is_numerable,
                    last_snapshot,
                ))
            })
            .collect::<Result<Vec<_>>>()?
    };

    let context = HolderContextProto::new(acc_holder, accounts);
    Ok(Response::new(context.encode_to_vec()))
}

#[tauri::command]
pub async fn get_account_context(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    main_context: State<'_, MainContextProto>,
    account_pk: i64,
) -> Result<Response> {
    log::info!("Get Account {account_pk} context");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    // TODO: Can we just retrieve it from the MainContext and save one DB call.
    let (account, snapshots) = {
        let (account, account_holder_role, custodian, account_type) = Account::details_for_pk(account_pk, &mut conn)?;
        let account_type = main_context
            .find_account_type(account_type.id)
            .ok_or(Error::Other(format!(
                "Account type 'pk={}' not found in main context",
                account_type.id
            )))?;

        let snapshots = get_snapshots(&account, &mut conn)?;
        let last_snapshot = snapshots.first().cloned();

        let account = AccountProto::new(
            account.id,
            account.name,
            custodian.into(),
            account_type.clone(),
            google_type::CurrencyCode::new(&account.ccy)?,
            account.identifier,
            account.description,
            account.open.into(),
            account_holder_role.owns_money,
            account.is_numerable,
            last_snapshot,
        );

        (account, snapshots)
    };

    // Movements
    let movements: Vec<MovementProto> = {
        all_movements_for_account_id(account_pk, &mut conn)?
            .into_iter()
            .map(|v| {
                let transaction_pk = v.transaction_id();
                movement_into_model_movement(v, &account, &main_context, Some(transaction_pk), &mut conn)
            })
            .collect::<Result<Vec<_>>>()?
    };

    let context = AccountContextProto::new(account, movements, snapshots);
    Ok(Response::new(context.encode_to_vec()))
}

pub fn movement_into_model_movement(
    v: Movement,
    account: &AccountProto,
    main_context: &MainContextProto,
    transaction_pk: Option<i64>,
    conn: &mut PgConnection,
) -> Result<MovementProto> {
    let account_ccy = account.currency_code()?;

    let movement_amount: MovementAmountProto = match &v {
        Movement::NonNumerable(movement) => {
            let amount = google_type::Money::new(movement.amount.clone(), account_ccy)?;
            MovementAmountProto::new_non_numerable(amount)
        }
        Movement::Numerable(movement_numerable) => {
            let unit_value =
                google_type::Money::new(movement_numerable.movement_numerable.unit_value.clone(), account_ccy)?;
            let quantity = google_type::Decimal::new(movement_numerable.movement_numerable.quantity.clone());
            MovementAmountProto::new_numerable(unit_value, quantity)
        }
        Movement::Dividend(movement_dividend) => {
            let ex_dividend_date: google_type::Date = movement_dividend.movement_dividend.ex_dividend_date.into();
            let unit_value =
                google_type::Money::new(movement_dividend.movement_dividend.unit_value.clone(), account_ccy)?;

            // From the date, get the closest snapshot, so I can get the quantity
            let quantity: google_type::Decimal = {
                let snapshot =
                    finances_investments::sql::queries::all_snapshotnumerable_for_account_id(*account.pk(), conn)?
                        .into_iter()
                        .find(|s| s.snapshot.date_value < movement_dividend.movement_dividend.ex_dividend_date)
                        .ok_or(Error::Other("Cannot find snapshot for the given dividend".to_string()))?;
                google_type::Decimal::new(snapshot.snapshot_numerable.quantity)
            };
            MovementAmountProto::new_dividend(ex_dividend_date, unit_value, quantity)
        }
    };

    let movement_type = main_context
        .find_movement_type(v.type_id())
        .cloned()
        .ok_or(Error::Other(format!(
            "Movement type 'pk={}' not found in main context",
            v.type_id()
        )))?;
    let direction: MovementDirectionProto = v.direction().into();
    let fx = v
        .fx_id()
        .map(|v| {
            let fx = Fx::all()
                .filter(fx_by_pk(v))
                .select(Fx::as_select())
                .first::<Fx>(conn)?;

            let pair = FxQuotePairProto::new(
                google_type::CurrencyCode::new(&fx.local)?,
                google_type::CurrencyCode::new(&fx.foreign)?,
            )?;
            Ok::<_, Error>(FxQuoteProto::new(
                pair,
                fx.date_value.into(),
                google_type::Decimal::new(fx.rate),
            ))
        })
        .transpose()?;

    Ok::<_, Error>(MovementProto::new(
        Some(v.id()),
        (*v.date_value()).into(),
        transaction_pk,
        movement_type,
        direction,
        movement_amount,
        fx,
        *account.pk(),
    ))
}

pub(crate) fn get_snapshots(account: &Account, conn: &mut PgConnection) -> Result<Vec<SnapshotProto>> {
    let account_ccy = google_type::CurrencyCode::new(&account.ccy)?;

    // Snapshots
    let snapshots: Vec<SnapshotProto> = if !account.is_numerable {
        Account::snapshots_for_pk(account.id, conn)?
            .into_iter()
            .map(|v| {
                let amount = google_type::Money::new(v.amount, account_ccy)?;
                let money_amount = MoneyAmountProto::new_non_numerable(amount);
                Ok::<_, Error>(SnapshotProto::new(
                    Some(v.id),
                    v.date_value.into(),
                    money_amount,
                    account.id,
                ))
            })
            .collect::<Result<Vec<_>>>()?
    } else {
        all_snapshotnumerable_for_account_id(account.id, conn)?
            .into_iter()
            .map(|v: SnapshotNumerable| {
                let unit_value = google_type::Money::new(v.snapshot_numerable.unit_value, account_ccy)?;
                let quantity = google_type::Decimal::new(v.snapshot_numerable.quantity);
                let money_amount = MoneyAmountProto::new_numerable(unit_value, quantity);
                Ok::<_, Error>(SnapshotProto::new(
                    Some(v.snapshot.id),
                    v.snapshot.date_value.into(),
                    money_amount,
                    account.id,
                ))
            })
            .collect::<Result<Vec<_>>>()?
    };

    Ok(snapshots)
}
