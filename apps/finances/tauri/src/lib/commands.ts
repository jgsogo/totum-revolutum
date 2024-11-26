import {invoke} from "@tauri-apps/api/core";
import {Account, AccountType, Holder} from "$lib/models/Account";
import {Snapshot} from "$lib/models/Snapshot"
import {MenuGroup} from "$lib/models/MenuGroup";
import {Movement} from "$lib/models/Movement"

/** The data returned by the backend representing an Account */
type AccountData = {
    pk: number,
    name: string,
    holder: { name: string },
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
    let holder = new Holder(data.holder.name);
    let account_type = new AccountType(data.type.name);
    return new Account(data.pk, data.name, holder, account_type, data.ccy, data.identifier);
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
 * Returns (a promise to) the MenuGroup items
 * @returns {MenuGroup[]} The list of menu entries
 */
export const sidebar_menu = async (category: string): Promise<MenuGroup[]> => {
    const data: {
        name: string,
        accounts: AccountData[],
    }[] = await invoke("sidebar_menu", {category});
    return data.map((it) => {
        let accounts = it.accounts.map((acc) => create_account(acc))
        return new MenuGroup(it.name, accounts)
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


/**
 * Returns (a promise to) all the Holders in the database
 * @returns {Holder[]} All the holders
 */
export const holders = async (): Promise<Holder[]> => {
    const data: {
        name: string,
        is_company: boolean
    }[] = await invoke("holders", {});
    return data.map((it) => {
        return new Holder(it.name, it.is_company);
    });
};
