import { invoke, type InvokeArgs, type InvokeOptions } from "@tauri-apps/api/core";
import { Account, AccountCategories, AccountType } from "$lib/models/Account";
import { Snapshot } from "$lib/models/Snapshot"
import { Movement } from "$lib/models/Movement"
import { Custodian } from "./models/Custodian";
import { Holder } from "./models/Holder";
import { MovementType } from "./models/MovementType";
import type { NewSnapshot } from "./forms/SnapshotForm/NewSnapshot.svelte";
import { TransactionGroup } from "./models/TransactionGroup";
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

/** The data returned by the backend representing an Custodian */
type CustodianData = {
    pk: number, name: string, photo?: string
};


/**
 * Converts an {@link CustodianData} dictionary into an {@link Custodian}
 * @param {CustodianData} data - The data returned by the backend
 * @returns {Custodian} The parsed instance
 */
const create_custodian = function (data: CustodianData): Custodian {
    return new Custodian(data.pk, data.name, data.photo);
}


/** The data returned by the backend representing an Account */
type AccountData = {
    pk: number,
    name: string,
    custodian: CustodianData,
    type: { name: string },
    ccy: string,
    identifier?: string,
    is_numerable: boolean,
    open: string,
};

/**
 * Converts an {@link AccountData} dictionary into an {@link Account}
 * @param {AccountData} data - The data returned by the backend
 * @returns {Account} The parsed instance
 */
const create_account = function (data: AccountData): Account {
    let custodian = create_custodian(data.custodian);
    let account_type = new AccountType(data.type.name);
    let account_category = AccountCategories.default;
    if (data.is_numerable) {
        account_category = AccountCategories.numerable_stock;
    }
    return new Account(account_category, data.pk, data.name, custodian, account_type, data.ccy, data.is_numerable, new Date(data.open), data.identifier);
}

/** The data returned by the backend representing a Snapshot */
type SnapshotData = {
    pk: number,
    account_id: number,
    amount: number,
    date_value: string,
    quantity?: number,
    unit_value?: number
};

/**
 * Converts an {@link SnapshotData} dictionary into an {@link Snapshot}
 * @param {Account} account - The account this snapshot belongs to
 * @param {SnapshotData} data - The data returned by the backend
 * @returns {Snapshot} The parsed instance
 */
const _create_snapshot = function (account: Account, data: SnapshotData): Snapshot {
    if (account.pk !== data.account_id) throw new Error("Snapshot mismatch Account");
    return new Snapshot(data.pk, account.ccy, data.amount, new Date(data.date_value), data.quantity, data.unit_value);
}



/**
 * Returns (a promise to) all the Snapshots for a given Account
 * @param {Account} account - Account instance
 * @returns {Snapshot[]} All the snapshots for the given account
 */
export const get_account_snapshots = async (account: Account): Promise<Snapshot[]> => {
    const data: SnapshotData[] = await invoke("get_account_snapshots", { pk: account.pk });
    return data.map((it) => {
        return _create_snapshot(account, it);
    });
};

/**
 * Returns (a promise to) all the Movements for a given Account
 * @param {Account} account - Account instance
 * @returns {Movement[]} All the movements for the given account
 */
export const get_account_movements = async (account: Account): Promise<Movement[]> => {
    const data: {
        amount: number,
        quantity?: number,
        unit_value?: number,
        direction: number,
        date: string,
        date_value: string,
        // account: ??
        fx?: {
            foreign: string,
            local: string,
            rate: number,
            date_value: string
        },
        transfer: {
            description: string,
        },
        type: {
            name: string,
            level: number,
            // parent: MovementType
        }
    }[] = await invoke("get_account_movements", { pk: account.pk });
    return data.map((it) => {
        return new Movement(it.amount, it.direction, it.date, it.date_value, it.quantity, it.unit_value)
    });
};


/** The data returned by the backend representing a Holder */
type HolderData = {
    pk: number,
    name: string,
    is_company: boolean
    photo?: string,
};

/**
 * Converts an {@link HolderData} dictionary into an {@link Holder}
 * @param {HolderData} data - The data returned by the backend
 * @returns {Holder} The parsed instance
 */
const create_holder = function (data: HolderData): Holder {
    return new Holder(data.pk, data.name, data.is_company, data.photo);
}

/**
 * Returns (a promise to) the Holder with the given primary key value
 * @param {number} pk - The primary key value of the holder we are looking for
 * @returns {Holder} The Holder instance
 */
export const get_holder_details = async (pk: number): Promise<Holder> => {
    const data: HolderData = await invoke("get_holder_details", { pk });
    return create_holder(data);
};

/**
 * Returns (a promise to) the Holder with the given primary key value
 * @param {number} pk - The primary key value of the holder we are looking for
 * @returns {Holder} The Holder instance
 */
export const get_custodian_details = async (pk: number): Promise<Custodian> => {
    const data: CustodianData = await invoke("get_custodian_details", { pk });
    return create_custodian(data);
};



/**
 * Creates a snapshot for the give account
 * @param {NewSnapshot} snapshot - The new Snapshot to create
 * @returns - A promise that resolves when the snapshot is created, or the error if it was not possible
 */
export const create_snapshot = async (snapshot: NewSnapshot) => {
    await invoke("create_snapshot", { snapshot: snapshot.toJSON() });
}

/** The data returned by the backend representing a MovementType */
type MovementTypeData = {
    pk: number,
    name: string,
};

/**
 * Converts an {@link MovementTypeData} dictionary into an {@link MovementType}
 * @param {MovementTypeData} data - The data returned by the backend
 * @returns {MovementType} The parsed instance
 */
const _create_movementtype = function (data: MovementTypeData): MovementType {
    return new MovementType(data.pk, data.name);
}

/**
 * Returns (a promise to) all the MovementTypes in the database
 * @returns {MovementType[]} All the movement types
 */
export const get_all_movementtypes = async (): Promise<MovementType[]> => {
    const data: MovementTypeData[] = await invoke("get_all_movementtypes", {});
    return data.map((it) => {
        return _create_movementtype(it);
    });
};

/**
 * Returns (a promise to) with the breadcrumbs for a MovementType
 * @param {MovementType} movementtype - The movementtype we want to get breadcrumbs for
 * @returns {string[]} An ordered vector with the breadcrumbs
 */
export const get_breadcrumbs_for_movementtype = async (movementtype: MovementType): Promise<string[]> => {
    return await invoke("get_breadcrumbs_for_movementtype", { pk: movementtype.pk });
};


/**
 * Returns (a promise to) all the TransactionGroups in the database
 * @returns {TransactionGroup[]} The TransactionGroup instances
 */
export const get_all_transaction_groups = async (): Promise<TransactionGroup[]> => {
    const data: TransactionGroup[] = await invoke("get_all_transaction_groups", {});
    return data.map((it) => {
        return new TransactionGroup(it.pk, it.name, it.description);
    });
};


/**
 * Creates a transaction
 * @param {NewTransaction} transaction - The new transaction to create
 * @returns - A promise that resolves when the transaction is created, or the error if it was not possible
 */
export const create_transaction = async (transaction: NewTransaction) => {
    console.log(JSON.stringify(transaction));
    await invoke("create_transaction", { transaction: transaction.toJSON() });
}
