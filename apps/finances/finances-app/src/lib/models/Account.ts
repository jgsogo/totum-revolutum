import {invoke} from "@tauri-apps/api/core";
import {Snapshot} from "$lib/models/Snapshot";
import {account_snapshot_latest} from "$lib/commands"

export class AccountType {
    private readonly name: string;

    constructor(name: string) {
        this.name = name;
    }

    toString() {
        return this.name;
    }
}

export class Holder {
    private readonly name: string;

    constructor(name: string) {
        this.name = name;
    }

    toString() {
        return this.name;
    }

}

export class Account {
    readonly pk: number;
    readonly name: string;
    readonly holder: Holder;
    readonly type: AccountType;
    readonly ccy: string;
    readonly identifier?: string;

    constructor(pk: number, name: string, holder: Holder, type: AccountType, ccy: string, identifier?: string) {
        this.pk = pk;
        this.name = name;
        this.holder = holder;
        this.type = type;
        this.ccy = ccy;
        this.identifier = identifier;
    }

    async last_snapshot(): Promise<Snapshot> {
        return account_snapshot_latest(this);
    }
}
