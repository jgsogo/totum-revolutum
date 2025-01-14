import { invoke, type InvokeArgs, type InvokeOptions } from "@tauri-apps/api/core";
import type { NewSnapshot } from "./forms/SnapshotForm/NewSnapshot.svelte";
import type { NewTransaction } from "./forms/TransactionForm/NewTransaction.svelte";
import { AppStateSchema, type AppState, MainContext, type MainContextProto, MainContextSchema, type HolderContextProto, HolderContext, HolderContextSchema, type AccountContextProto, AccountContextSchema, AccountContext } from "../../models/src-js/index";
import { fromBinary, type DescMessage } from "@bufbuild/protobuf";
import { Buffer } from 'buffer';


/**
 * Calls the given command and returns the protobuf message already parsed
 *
 * @returns {Type} The parsed message
 */
async function invoke_protobuf_command<Desc extends DescMessage, Type>(schema_type: Desc, cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<Type> {
    const data: ArrayBuffer = await invoke(cmd, args, options);
    const context: Type = fromBinary(schema_type, Buffer.from(data, 0, data.byteLength));
    return context;
}

/**
 * Returns (a promise to) the app configuration
 * @returns {AppState} Application configuration
 */
export const get_app_state = async (): Promise<AppState> => {
    return await invoke_protobuf_command(AppStateSchema, "get_app_state");
};

/**
 * Returns (a promise to) the main context
 * @returns {MainContext} Main context, the same for all the application
 */
export const get_main_context = async (): Promise<MainContext> => {
    let context: MainContextProto = await invoke_protobuf_command(MainContextSchema, "get_main_context");
    return new MainContext(context);
};

/**
 * Returns (a promise to) the holder context
 * @returns {HolderContext} Holder context, common things for a given holder
 */
export const get_holder_context = async (holder_pk: number): Promise<HolderContext> => {
    let context: HolderContextProto = await invoke_protobuf_command(HolderContextSchema, "get_holder_context", { holderPk: holder_pk });
    return new HolderContext(context);
};

/**
 * Returns (a promise to) the account context
 * @returns {AccountContext} Account context, common things for a given account
 */
export const get_account_context = async (account_pk: number): Promise<AccountContext> => {
    const context: AccountContextProto = await invoke_protobuf_command(AccountContextSchema, "get_account_context", { accountPk: account_pk });
    return new AccountContext(context);
};




/**
 * Creates a snapshot for the give account
 * @param {NewSnapshot} snapshot - The new Snapshot to create
 * @returns - A promise that resolves when the snapshot is created, or the error if it was not possible
 */
export const create_snapshot = async (snapshot: NewSnapshot) => {
    await invoke("create_snapshot", { snapshot: snapshot.toJSON() });
}


/**
 * Creates a transaction
 * @param {NewTransaction} transaction - The new transaction to create
 * @returns - A promise that resolves when the transaction is created, or the error if it was not possible
 */
export const create_transaction = async (transaction: NewTransaction) => {
    console.log(JSON.stringify(transaction));
    await invoke("create_transaction", { transaction: transaction.toJSON() });
}
