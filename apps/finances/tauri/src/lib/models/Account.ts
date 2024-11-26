import {Snapshot} from "$lib/models/Snapshot";
import {Movement} from "$lib/models/Movement";
import {account_snapshot_latest, account_snapshots, account_movements} from "$lib/commands"

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
    private readonly is_company: boolean;

    constructor(name: string, is_company: boolean) {
        this.name = name;
        this.is_company = is_company;
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

    async snapshots(): Promise<Snapshot[]> {
        return account_snapshots(this);
    }

    async movements(): Promise<Movement[]> {
        return account_movements(this);
    }
}
