import {invoke} from "@tauri-apps/api/core";
import {Account, AccountType, Holder} from "$lib/models/Account";
import {Snapshot} from "$lib/models/Snapshot"

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
    return new Snapshot(account.ccy, data.amount, data.date_value, data.quantity, data.unit_value);
};

export const account_detail = async (pk: number): Promise<Account> => {
    const data: {
        name: string,
        holder: { name: string },
        type: { name: string },
        ccy: string,
        identifier?: string
    } = await invoke("account_detail", {pk});
    let holder = new Holder(data.holder.name);
    let account_type = new AccountType(data.type.name);
    return new Account(pk, data.name, holder, account_type, data.ccy, data.identifier);
};
