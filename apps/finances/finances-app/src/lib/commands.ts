import {invoke} from "@tauri-apps/api/core";
import {Account, AccountType, Holder} from "$lib/models/Account";
import {Snapshot} from "$lib/models/Snapshot"
import {MenuGroup} from "$lib/models/MenuGroup";
import {Movement} from "$lib/models/Movement"

type AccountData = {
    name: string,
    holder: { name: string },
    type: { name: string },
    ccy: string,
    identifier?: string
};

const create_account = function (pk: number, data: AccountData): Account {
    let holder = new Holder(data.holder.name);
    let account_type = new AccountType(data.type.name);
    return new Account(pk, data.name, holder, account_type, data.ccy, data.identifier);
}

type SnapshotData = {
    amount: number,
    date_value: string,
    quantity?: number,
    unit_value?: number
};

const create_snapshot = function (account: Account, data: SnapshotData): Snapshot {
    return new Snapshot(account.ccy, data.amount, data.date_value, data.quantity, data.unit_value);
}

/**
 * Returns (a promise to) the latest Snapshot for a given Account
 * @param {Account} account - Account instance
 * @returns {Snapshot} Latest snapshot for the given account
 */
export const account_snapshot_latest = async (account: Account): Promise<Snapshot> => {
    const data: {
        amount: number,
        date_value: string,
        quantity?: number,
        unit_value?: number
    } = await invoke("account_snapshot_latest", {pk: account.pk});
    return create_snapshot(account, data);
};

/**
 * Returns (a promise to) the Account with the given primary key value
 * @param {number} pk - The primary key value of the account we are looking for
 * @returns {Account} The Account instance
 */
export const account_detail = async (pk: number): Promise<Account> => {
    const data: AccountData = await invoke("account_detail", {pk});
    return create_account(pk, data);
};

/**
 * Returns (a promise to) the MenuGroup items
 * @returns {MenuGroup[]} The list of menu entries
 */
export const sidebar_menu = async (category: string): Promise<MenuGroup[]> => {
    const data: {
        name: string,
        accounts: [number, AccountData][],
    }[] = await invoke("sidebar_menu", {category});
    return data.map((it) => {
        let accounts = it.accounts.map(([pk, acc]) => create_account(pk, acc))
        return new MenuGroup(it.name, accounts)
    });
};


/**
 * Returns (a promise to) all the Snapshots for a given Account
 * @param {Account} account - Account instance
 * @returns {Snapshot[]} All the snapshots for the given account
 */
export const account_snapshots = async (account: Account): Promise<Snapshot[]> => {
    const data: {
        amount: number,
        date_value: string,
        quantity?: number,
        unit_value?: number
    }[] = await invoke("account_snapshots", {pk: account.pk});
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
