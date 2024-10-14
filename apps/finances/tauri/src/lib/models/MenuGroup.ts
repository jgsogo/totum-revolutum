import type {Account} from "$lib/models/Account";

export class MenuGroup {
    readonly name: string;
    readonly accounts: Account[];

    constructor(name: string, accounts: Account[]) {
        this.name = name;
        this.accounts = accounts;
    }


}
