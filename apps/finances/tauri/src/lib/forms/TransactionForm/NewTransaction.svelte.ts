import type { Account } from "$lib/models/Account";
import type { TransactionGroup } from "$lib/models/TransactionGroup";


import { NewMovement, NewMovementType } from "../MovementForm/NewMovement.svelte";


export class NewTransaction {
    name?: string = $state();
    description?: string = $state();
    transaction_group?: TransactionGroup = $state();

    movements_from: NewMovement[] = $state([]);
    movements_to: NewMovement[] = $state([]);

    constructor(date_value?: Date, initial_movements_from?: NewMovement[], initial_movements_to?: NewMovement[]) {
        this.movements_from = initial_movements_from ?? [];
        this.movements_to = initial_movements_to ?? [];
    }

    add_movement_from(account?: Account) {
        let new_mov = new NewMovement(account?.is_numerable ? NewMovementType.Numerable : NewMovementType.NonNumerable);
        new_mov.account = account;
        new_mov.date_value = new Date();
        this.movements_from = this.movements_from.concat(new_mov);
    }

    add_movement_to(account?: Account) {
        let new_mov = new NewMovement(account?.is_numerable ? NewMovementType.Numerable : NewMovementType.NonNumerable);
        new_mov.account = account;
        new_mov.date_value = new Date();
        this.movements_to = this.movements_to.concat(new_mov);
    }

    remove_movement_from(index: number) {
        let pre_list = this.movements_from.slice(0, index);
        this.movements_from = pre_list.concat(
            this.movements_from.slice(index + 1, this.movements_from.length)
        );
    }

    remove_movement_to(index: number) {
        let pre_list = this.movements_to.slice(0, index);
        this.movements_to = pre_list.concat(
            this.movements_to.slice(index + 1, this.movements_to.length)
        );
    }

    is_valid(date_required: boolean, base_ccy: string): boolean {
        return (this.name !== undefined &&
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

    // Assigns this date to all the movements
    set_date(date: Date) {
        this.movements_from.forEach((value) => value.date_value = date);
        this.movements_to.forEach((value) => value.date_value = date);
    }

    toJSON() {
        // This serialization is used when sending this structure via a command to the Tauri backend
        return {
            name: this.name,
            description: this.description,
            transaction_group_pk: this.transaction_group?.pk,
            movements_from: this.movements_from.map((mov) => mov.toJSON()),
            movements_to: this.movements_to.map((mov) => mov.toJSON()),
        };
    }
};
