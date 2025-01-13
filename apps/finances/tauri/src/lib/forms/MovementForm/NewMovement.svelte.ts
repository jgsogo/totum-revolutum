import { toFixedNumber } from "$lib/utils";
import { Account, MovementType, Snapshot } from "../../../../models/src-js";

export enum NewMovementType {
    NonNumerable,
    Numerable,
    Dividend,
}

export class NewMovement {
    account?: Account = $state();
    snapshots?: Snapshot[] = $state();
    mov_type?: MovementType = $state();
    date_value?: Date = $state();
    fx?: number = $state();

    type: NewMovementType = $state(NewMovementType.NonNumerable);

    // non-numerable movement
    amount?: number = $state();

    // numerable movement
    quantity?: number = $state();
    unit_value?: number = $state();

    // dividend movement
    ex_dividend_date?: Date = $state();
    ex_dividend_snapshot?: Snapshot = $state();

    constructor(type: NewMovementType, account?: Account, date_value?: Date, snapshots?: Snapshot[]) {
        this.type = type;
        this.account = account;
        this.snapshots = snapshots;
        this.date_value = date_value;
    }

    is_valid(date_required: boolean, base_ccy: string): boolean {
        let valid = this.account instanceof Account &&
            this.mov_type instanceof MovementType &&
            (!date_required || (date_required && this.date_value instanceof Date)) &&
            (this.account.ccy() === base_ccy || this.fx != undefined);
        switch (this.type) {
            case NewMovementType.NonNumerable:
                return valid && this.amount != undefined;
            case NewMovementType.Numerable:
                return valid && this.quantity != undefined && this.unit_value != undefined;
            case NewMovementType.Dividend:
                return valid && this.ex_dividend_date instanceof Date && this.ex_dividend_snapshot instanceof Snapshot;
        }
    }

    total(base_ccy: string): number | undefined {
        if (!this.account) return undefined;

        let total = 0;
        switch (this.type) {
            case NewMovementType.NonNumerable:
                if (!this.amount) return undefined;
                total = this.amount;
                break;
            case NewMovementType.Numerable:
                if (!this.quantity || !this.unit_value) return undefined;
                total = this.quantity * this.unit_value;
                break;
            case NewMovementType.Dividend:
                if (!this.ex_dividend_snapshot || !this.ex_dividend_snapshot.amount().quantity() || !this.unit_value) return undefined;
                total = this.ex_dividend_snapshot.amount().quantity()!.as_number() * this.unit_value;
                break;
        }

        if (this.account.ccy() !== base_ccy) {
            if (!this.fx) return undefined;
            total = total / this.fx;
        }
        return toFixedNumber(total, 4);
    }

    toJSON() {
        // This serialization is used when sending this structure via a command to the Tauri backend
        return {
            account_pk: this.account?.pk(),
            movement_type_pk: this.mov_type?.pk(),
            date_value:  this.date_value?.toISOString().slice(0, 10),
            fx: this.fx,
            type: NewMovementType[this.type],
            // non-numerable
            amount: this.amount,
            // numerable
            quantity: this.quantity,
            unit_value: this.unit_value,
            // dividend
            ex_dividend_date: this.ex_dividend_date?.toISOString().slice(0, 10),
            ex_dividend_snapshot_pk: this.ex_dividend_snapshot?.pk(),
        };
    }
};
