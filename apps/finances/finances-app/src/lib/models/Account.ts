import { invoke } from "@tauri-apps/api/core";

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

    constructor(ccy: string, amount: number) {
        this._amount = amount;
        this._ccy = ccy;
    }

    toString() {
        return `${this._amount} ${this._ccy}`;
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
        const account = await invoke("detail_command", {pk});
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
        return new Promise<Snapshot>((resolve) => {
            resolve(new Snapshot("EUR", 1234.56));
        });
        // const snapshot: Snapshot = await invoke("account_snapshot", {pk: this._pk});
        // return snapshot
    }
}
