import {invoke} from "@tauri-apps/api/core";
import {Snapshot} from "$lib/models/Snapshot";
import {account_snapshot_latest} from "$lib/commands"

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

export class Account {
    private _pk: number;
    private _name: string;
    private _holder: Holder;
    private _type: AccountType;
    private _ccy: string;
    private _identifier?: string;

    constructor(pk: number, name: string, holder: Holder, type: AccountType, ccy: string, identifier?: string) {
        this._pk = pk;
        this._name = name;
        this._holder = holder;
        this._type = type;
        this._ccy = ccy;
        this._identifier = identifier;
    }

    public get pk(): number {
        return this._pk;
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

    public get ccy(): string {
        return this._ccy;
    }

    public get identifier(): string | undefined {
        return this._identifier;
    }

    async last_snapshot(): Promise<Snapshot> {
        return account_snapshot_latest(this);
        // return Snapshot.last_snapshot(this);
    }
}
