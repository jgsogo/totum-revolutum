import { Account } from "$lib/models/Account";
import { MovementType } from "$lib/models/MovementType";

export class NewMovement {
    account?: Account = $state();
    mov_type?: MovementType = $state();
    amount?: number = $state();
    date_value?: Date = $state();
    quantity?: number = $state();
    unit_value?: number = $state();
    fx?: number = $state();

    is_valid(date_required: boolean, base_ccy: string): boolean {
        return (this.account instanceof Account &&
            this.mov_type instanceof MovementType &&
            (!date_required || (date_required && this.date_value instanceof Date)) &&
            ((this.account.is_numerable && this.quantity != undefined && this.unit_value != undefined) || (!this.account.is_numerable && this.amount != undefined)) &&
            (this.account.ccy === base_ccy || this.fx != undefined)
        )
    }

    total(base_ccy: string): number | undefined {
        if (!this.account) return undefined;
        let total = 0;
        if (this.account.is_numerable) {
            if (!this.quantity || !this.unit_value) return undefined;
            total = this.quantity * this.unit_value;
        } else {
            if (!this.amount) return undefined;
            total = this.amount;
        }

        if (this.account.ccy !== base_ccy) {
            if (!this.fx) return undefined;
            total = total / this.fx;
        }
        return total;
    }

    toJSON() {
        return {
            account: this.account,
            movement_type: this.mov_type,
            amount: this.amount,
            date_value: this.date_value,
            quantity: this.quantity,
            unit_value: this.unit_value,
            fx: this.fx
        };
    }
};
