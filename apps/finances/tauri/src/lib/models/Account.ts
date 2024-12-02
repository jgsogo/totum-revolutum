import {Snapshot} from "$lib/models/Snapshot";
import {Movement} from "$lib/models/Movement";
import {account_snapshot_latest, account_snapshots, account_movements} from "$lib/commands"
import { Custodian } from "./Custodian";

export enum AccountCategories {
    default = 'default',
    numerable_stock = 'numerable_stock',
  };

export class AccountType {
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
    readonly custodian: Custodian;
    readonly type: AccountType;
    readonly ccy: string;
    readonly identifier?: string;
    readonly is_numerable: boolean;
    readonly open: Date;
    readonly category: AccountCategories

    constructor(category: AccountCategories, pk: number, name: string, custodian: Custodian, type: AccountType, ccy: string, is_numerable: boolean, open: Date, identifier?: string) {
        this.pk = pk;
        this.name = name;
        this.custodian = custodian;
        this.type = type;
        this.ccy = ccy;
        this.identifier = identifier;
        this.is_numerable = is_numerable;
        this.open = open;
        this.category = category;
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
