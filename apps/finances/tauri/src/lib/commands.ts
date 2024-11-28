import {invoke} from "@tauri-apps/api/core";
import {Account, AccountType} from "$lib/models/Account";
import {Snapshot} from "$lib/models/Snapshot"
import {Movement} from "$lib/models/Movement"
import { Custodian } from "./models/Custodian";
import { Holder } from "./models/Holder";

/** The data returned by the backend representing an Account */
type AccountData = {
    pk: number,
    name: string,
    custodian: { pk: number, name: string, photo?: string },
    type: { name: string },
    ccy: string,
    identifier?: string
};

/**
 * Converts an {@link AccountData} dictionary into an {@link Account}
 * @param {AccountData} data - The data returned by the backend
 * @returns {Account} The parsed instance
 */
const create_account = function (data: AccountData): Account {
    let custodian = new Custodian(data.custodian.pk, data.custodian.name, data.custodian.photo);
    let account_type = new AccountType(data.type.name);
    return new Account(data.pk, data.name, custodian, account_type, data.ccy, data.identifier);
}

/** The data returned by the backend representing a Snapshot */
type SnapshotData = {
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
const create_snapshot = function (account: Account, data: SnapshotData): Snapshot {
    if (account.pk !== data.account_id) throw new Error("Snapshot mismatch Account");
    return new Snapshot(account.ccy, data.amount, data.date_value, data.quantity, data.unit_value);
}

/**
 * Returns (a promise to) the latest Snapshot for a given Account
 * @param {Account} account - Account instance
 * @returns {Snapshot} Latest snapshot for the given account
 */
export const account_snapshot_latest = async (account: Account): Promise<Snapshot> => {
    const data: SnapshotData = await invoke("account_snapshot_latest", {pk: account.pk});
    return create_snapshot(account, data);
};

/**
 * Returns (a promise to) the Account with the given primary key value
 * @param {number} pk - The primary key value of the account we are looking for
 * @returns {Account} The Account instance
 */
export const account_detail = async (pk: number): Promise<Account> => {
    const data: AccountData = await invoke("account_detail", {pk});
    return create_account(data);
};

/**
 * Returns (a promise to) all the Accounts for a given Holder
 * @param {Holder} holder - The holder
 * @returns {Account[]} The Account instances
 */
export const all_accounts = async (holder: Holder): Promise<Account[]> => {
    const data: AccountData[] = await invoke("all_accounts", {holderPk: holder.pk});
    return data.map((it) => {
        return create_account(it)
    });
};

/**
 * Returns (a promise to) all the savings Accounts for a given Holder
 * @param {Holder} holder - The holder
 * @returns {Account[]} The Account instances
 */
export const savings_accounts = async (holder: Holder): Promise<Account[]> => {
    const data: AccountData[] = await invoke("savings_accounts", {holderPk: holder.pk});
    return data.map((it) => {
        return create_account(it)
    });
};

/**
 * Returns (a promise to) all the investment Accounts for a given Holder
 * @param {Holder} holder - The holder
 * @returns {Account[]} The Account instances
 */
export const investment_accounts = async (holder: Holder): Promise<Account[]> => {
    const data: AccountData[] = await invoke("investment_accounts", {holderPk: holder.pk});
    return data.map((it) => {
        return create_account(it)
    });
};

/**
 * Returns (a promise to) all the retirement Accounts for a given Holder
 * @param {Holder} holder - The holder
 * @returns {Account[]} The Account instances
 */
export const retirement_accounts = async (holder: Holder): Promise<Account[]> => {
    const data: AccountData[] = await invoke("retirement_accounts", {holderPk: holder.pk});
    return data.map((it) => {
        return create_account(it)
    });
};

/**
 * Returns (a promise to) all the Snapshots for a given Account
 * @param {Account} account - Account instance
 * @returns {Snapshot[]} All the snapshots for the given account
 */
export const account_snapshots = async (account: Account): Promise<Snapshot[]> => {
    const data: SnapshotData[] = await invoke("account_snapshots", {pk: account.pk});
    return data.map((it) => {
        return create_snapshot(account, it);
    });
};

/**
 * Returns (a promise to) all the Movements for a given Account
 * @param {Account} account - Account instance
 * @returns {Movement[]} All the movements for the given account
 */
export const account_movements = async (account: Account): Promise<Movement[]> => {
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
    }[] = await invoke("account_movements", {pk: account.pk});
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
 * Returns (a promise to) all the Holders in the database
 * @returns {Holder[]} All the holders
 */
export const holders = async (): Promise<Holder[]> => {
    const data: HolderData[] = await invoke("holders", {});
    return data.map((it) => {
        return create_holder(it);
    });
};

/**
 * Returns (a promise to) the Holder with the given primary key value
 * @param {number} pk - The primary key value of the holder we are looking for
 * @returns {Holder} The Holder instance
 */
export const holder_details = async (pk: number): Promise<Holder> => {
    const data: HolderData = await invoke("holder_details", {pk});
    return create_holder(data);
};



/**
 * Returns (a promise to) the base_media_url URL
 * @returns {string} The full URL to the base media URL
 */
export const get_base_media_url = async (): Promise<string> => {
    return await invoke("base_media_url", {});
};
