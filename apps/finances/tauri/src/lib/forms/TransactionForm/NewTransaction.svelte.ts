import type { TransactionGroup } from "$lib/models/TransactionGroup";


import type { NewMovement } from "../MovementForm/NewMovement.svelte";


export class NewTransaction {
    name?: string = $state();
    description?: string = $state();
    date_value?: Date = $state();
    transaction_group?: number = $state();

    movements_from: NewMovement[] = $state([]);
    movements_to: NewMovement[] = $state([]);

    is_valid(date_required: boolean, base_ccy: string): boolean {
        return (this.name !== undefined &&
            (!date_required || (date_required && this.date_value instanceof Date)) &&
            this.movements_from.length > 0 &&
            this.movements_from.every((v) => v.is_valid(!date_required, base_ccy)) &&
            this.movements_to.length > 0 &&
            this.movements_to.every((v) => v.is_valid(!date_required, base_ccy))
        ) && (this.total_from(base_ccy) === this.total_to(base_ccy))
    }

    total_from(base_ccy: string): number | undefined {
        let initial = 0;
        let total = this.movements_from.reduce((prev: number | undefined, curr: NewMovement) => {
            if (prev === undefined) return undefined;
            let total_mov = curr.total(base_ccy);
            if (total_mov === undefined) return undefined;
            return prev + total_mov;
        }, initial);
        return total;
    }

    total_to(base_ccy: string): number | undefined {
        let initial = 0;
        let total = this.movements_to.reduce((prev: number | undefined, curr: NewMovement) => {
            if (prev === undefined) return undefined;
            let total_mov = curr.total(base_ccy);
            if (total_mov === undefined) return undefined;
            return prev + total_mov;
        }, initial);
        return total;
    }

    toJSON() {
        return {
            name: this.name,
            description: this.description,
            date_value: this.date_value,
            transaction_group: this.transaction_group,
            movements_from: this.movements_from,
            movements_to: this.movements_to,
        };
    }
};
