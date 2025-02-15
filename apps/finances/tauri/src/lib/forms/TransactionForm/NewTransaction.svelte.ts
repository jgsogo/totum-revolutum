import type { MainContext, TransactionGroup } from "../../../../models/src-js";

import { NewMovement, NewMovementType } from "../MovementForm/NewMovement.svelte";
import { Account, Transaction as TransactionModel, Movement as MovementModel } from "../../../../models/src-js";
import { MovementDirection } from "../../../../models/src-js/movement";
import { CurrencyCode } from "../../../../../../../libraries/googleapis/src-js";

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

    reset() {
        this.name = undefined;
        this.description = undefined;
        this.transaction_group = undefined;
        this.movements_from.length = 0;
        this.movements_to.length = 0;
    }

    // Substitutes the existing data with the data from the input transaction
    take(transaction: TransactionModel, main_context: MainContext) {
        this.name = transaction.name()
        this.description = transaction.description()
        this.transaction_group = transaction.group()

        this.movements_from = transaction.movements_from().map((mov: MovementModel) => {
            let account = main_context.find_account(mov.account_pk())!;
            return NewMovement.create_from(mov, account);
        });

        this.movements_to = transaction.movements_to().map((mov: MovementModel) => {
            let account = main_context.find_account(mov.account_pk())!;
            return NewMovement.create_from(mov, account);
        })
    }

    toMessage(base_ccy: CurrencyCode): TransactionModel {
        let data = TransactionModel.create_from(this.name!, this.description, this.transaction_group);
        this.movements_from.forEach((v) => data.pushFromMovement(v.toMessage(MovementDirection.Out, base_ccy)));
        this.movements_to.forEach((v) => data.pushToMovement(v.toMessage(MovementDirection.In, base_ccy)));
        return data;
    }
};
