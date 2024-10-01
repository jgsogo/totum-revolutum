import { invoke } from "@tauri-apps/api/core";

export class AccountType {
    private _name: string;

    constructor(name: string) {
        this._name = name;
    }

    public get name() {
        return this._name;
    }
}

export class Holder {
    private _name: string;

    constructor(name: string) {
        this._name = name;
    }

    public get name() {
        return this._name;
    }
}

export class Account {
    // private _pk: number;
    private _name: string;
    private _holder: Holder;
    private _type: AccountType;

    static async Create(pk: number): Promise<Account> {
        const instance = new Account();

        /** Return the basic data from the account **/
        const account = await invoke("detail_command", {pk});
        // instance._pk = pk;
        instance._name = account.name;
        instance._holder = new Holder(account.holder.name);
        instance._type = new AccountType(account.type.name);

        return instance;
    }

    public get name() {
        return this._name;
    }

    public get holder() {
        return this._holder;
    }

    public get type() {
        return this._type;
    }
}
