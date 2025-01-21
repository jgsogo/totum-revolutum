import { invoke, } from "@tauri-apps/api/core";
import { AppState, MainContext, HolderContext, AccountContext, OutgoingMessage } from "../../models/src-js/index";


/**
 * Returns (a promise to) the app configuration
 * @returns {AppState} Application configuration
 */
export const get_app_state = async (): Promise<AppState> => {
    const data: ArrayBuffer = await invoke("get_app_state");
    return AppState.create_from(data);
};

/**
 * Returns (a promise to) the main context
 * @returns {MainContext} Main context, the same for all the application
 */
export const get_main_context = async (): Promise<MainContext> => {
    const data: ArrayBuffer = await invoke("get_main_context");
    return MainContext.create_from(data);
};

/**
 * Returns (a promise to) the holder context
 * @returns {HolderContext} Holder context, common things for a given holder
 */
export const get_holder_context = async (holder_pk: number): Promise<HolderContext> => {
    const data: ArrayBuffer = await invoke("get_holder_context", { holderPk: holder_pk });
    return HolderContext.create_from(data);
};

/**
 * Returns (a promise to) the account context
 * @returns {AccountContext} Account context, common things for a given account
 */
export const get_account_context = async (account_pk: number): Promise<AccountContext> => {
    const data: ArrayBuffer = await invoke("get_account_context", { accountPk: account_pk });
    return AccountContext.create_from(data);
};

/**
 * Creates a snapshot for the give account
 * @param {NewSnapshot} snapshot - The new Snapshot to create
 * @returns - A promise that resolves when the snapshot is created, or the error if it was not possible
 */
export const create_snapshot = async (snapshot: OutgoingMessage) => {
    let data = snapshot.toBinary();
    await invoke("create_snapshot", data);
}

/**
 * Creates a transaction
 * @param {NewTransaction} transaction - The new Transaction to create
 * @returns - A promise that resolves when the transaction is created, or the error if it was not possible
 */
export const create_transaction = async (transaction: OutgoingMessage) => {
    let data = transaction.toBinary();
    await invoke("create_transaction", data);
}
