import type { NewMovement } from "$lib/forms/MovementForm/NewMovement.svelte";

export class NewTransaction {
    name: string;
    description?: string;
    date_value?: Date;
    transaction_group_pk?: number;

    movements_from: NewMovement[];
    movements_to: NewMovement[];

    constructor(name: string, movements_from: NewMovement[], movements_to: NewMovement[], description?: string, date_value?: Date, transaction_group?: TransactionGroup) {
        this.name = name;
        this.description = description;
        this.date_value = date_value;
        this.transaction_group_pk = transaction_group?.pk;
        this.movements_from = movements_from;
        this.movements_to = movements_to;
    }
};

export class TransactionGroup {
    readonly pk: number;
    readonly name: string;

    constructor(pk: number, name: string) {
        this.pk = pk;
        this.name = name;
    }

    toString() {
        return this.name;
    }

}
