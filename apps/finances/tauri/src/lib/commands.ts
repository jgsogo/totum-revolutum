import { invoke, } from "@tauri-apps/api/core";
import { AppState, MainContext, HolderContext, AccountContext, LastTransactionsResponse, LastTransactionsRequest, Snapshot, Transaction } from "../../models/src-js";

/**
 * Returns (a promise to) the app configuration
 * @returns {AppState} Application configuration
 */
export const get_app_state = async (): Promise<AppState> => {
    const data: ArrayBuffer = await invoke("get_app_state");
    return AppState.create_from_array(data);
};

/**
 * Returns (a promise to) the main context
 * @returns {MainContext} Main context, the same for all the application
 */
export const get_main_context = async (): Promise<MainContext> => {
    const data: ArrayBuffer = await invoke("get_main_context");
    return MainContext.create_from_array(data);
};

/**
 * Returns (a promise to) the holder context
 * @returns {HolderContext} Holder context, common things for a given holder
 */
export const get_holder_context = async (holder_pk: number): Promise<HolderContext> => {
    const data: ArrayBuffer = await invoke("get_holder_context", { holderPk: holder_pk });
    return HolderContext.create_from_array(data);
};

/**
 * Returns (a promise to) the account context
 * @returns {AccountContext} Account context, common things for a given account
 */
export const get_account_context = async (account_pk: number): Promise<AccountContext> => {
    const data: ArrayBuffer = await invoke("get_account_context", { accountPk: account_pk });
    return AccountContext.create_from_array(data);
};

/**
 * Returns (a promise to) past transactions for a given account
 * @returns {LastTransactionsResponse}
 */
export const get_past_transactions = async (last_transactions_request: LastTransactionsRequest): Promise<LastTransactionsResponse> => {
    const data: ArrayBuffer = await invoke("past_transactions", last_transactions_request.toBinary());
    return LastTransactionsResponse.create_from_array(data);
}

/**
 * Returns (a promise to) a transaction, given its pk
 * @returns {Transaction}
 */
export const get_transaction = async (transaction_pk: number): Promise<Transaction> => {
    const data: ArrayBuffer = await invoke("get_transaction", { transactionPk: transaction_pk });
    return Transaction.create_from_array(data);
}

/**
 * Creates a snapshot for the give account
 * @param {Snapshot} snapshot - The new Snapshot to create
 * @returns - A promise that resolves when the snapshot is created, or the error if it was not possible
 */
export const create_snapshot = async (snapshot: Snapshot) => {
    let data = snapshot.toBinary();
    await invoke("create_snapshot", data);
}

/**
 * Creates a transaction
 * @param {Transaction} transaction - The new Transaction to create
 * @returns - A promise that resolves when the transaction is created, or the error if it was not possible
 */
export const create_transaction = async (transaction: Transaction) => {
    let data = transaction.toBinary();
    await invoke("create_transaction", data);
}
