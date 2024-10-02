import {invoke} from "@tauri-apps/api/core";
import {Account} from "$lib/models/Account";

export class Snapshot {
    private _amount: number;
    private _ccy: string;
    private _quantity?: number;
    private _unit_value?: number;
    private _date_value: string;

    // public static async last_snapshot(account: Account): Promise<Snapshot> {
    //     const data: {
    //         amount: number,
    //         date_value: string,
    //         quantity?: number,
    //         unit_value?: number
    //     } = await invoke("account_snapshot_latest", {pk: account.pk});
    //     return new Snapshot(account.ccy, data.amount, data.date_value, data.quantity, data.unit_value);
    // };

    constructor(ccy: string, amount: number, date_value: string, quantity?: number, unit_value?: number) {
        this._amount = amount;
        this._ccy = ccy;
        this._date_value = date_value;
        this._quantity = quantity;
        this._unit_value = unit_value
    }

    toString() {
        return `${this._amount} ${this._ccy}`;
    }

    public get date_value(): string {
        return this._date_value;
    }

    public get quantity(): number | undefined {
        return this._quantity;
    }

    public get unit_value(): number | undefined {
        return this._unit_value;
    }
}
