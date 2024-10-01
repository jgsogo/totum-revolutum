import { invoke } from "@tauri-apps/api/core";

export class Account {
    // private _pk: number;
    private _name: string;
    private _holder: string;
    private _type: string;

    static async Create(pk: number): Promise<Account> {
        const instance = new Account();

        /** Return the basic data from the account **/
        const account = await invoke("detail_command", {pk});
        // instance._pk = pk;
        instance._name = account.name;
        instance._holder = account.holder;
        instance._type = account.type;

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
