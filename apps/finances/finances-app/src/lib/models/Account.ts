import {invoke} from "@tauri-apps/api/core";

export class AccountType {
    private _name: string;

    constructor(name: string) {
        this._name = name;
    }

    toString() {
        return this._name;
    }
}

export class Holder {
    private _name: string;

    constructor(name: string) {
        this._name = name;
    }

    toString() {
        return this._name;
    }

}

export class Snapshot {
    private _amount: number;
    private _ccy: string;
    private _quantity?: number;
    private _unit_value?: number;
    private _date_value: string;

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

export class Account {
    private _pk: number;
    private _name: string;
    private _holder: Holder;
    private _type: AccountType;
    private _ccy: string;
    private _identifier?: string;

    static async Create(pk: number): Promise<Account> {
        const instance = new Account();

        /** Return the basic data from the account **/
        const account = await invoke("account_detail", {pk});
        instance._pk = pk;
        instance._name = account.name;
        instance._holder = new Holder(account.holder.name);
        instance._type = new AccountType(account.type.name);
        instance._ccy = account.ccy;
        instance._identifier = account.identifier;

        return instance;
    }

    public get name(): string {
        return this._name;
    }

    public get holder(): Holder {
        return this._holder;
    }

    public get type(): AccountType {
        return this._type;
    }

    public get identifier(): string | undefined {
        return this._identifier;
    }

    async snapshot(): Promise<Snapshot> {
        const snapshot = await invoke("account_snapshot_latest", {pk: this._pk});
        const instance = new Snapshot(this._ccy, snapshot.amount, snapshot.date_value, snapshot.quantity, snapshot.unit_value);
        return instance;
    }
}
