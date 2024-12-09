import type {Account} from "$lib/models/Account";
import type { MovementType } from "./MovementType";

export class NewMovement {
    account_pk: number;
    mov_type_pk: number;
    amount?: number;
    date_value?: Date;
    quantity?: number;
    unit_value?: number;
    fx?: number;

    constructor(account: Account, mov_type: MovementType, amount?: number, date_value?: Date, quantity?: number, unit_value?: number, fx?: number) {
        this.account_pk = account.pk;
        this.mov_type_pk = mov_type.pk;
        this.amount = amount;
        this.date_value = date_value;
        this.quantity = quantity;
        this.unit_value = unit_value;
        this.fx = fx;
    }
};


export class Movement {
    readonly amount: number;
    private readonly quantity?: number;
    private readonly unit_value?: number;
    readonly direction: number;
    private readonly date: string;
    readonly date_value: string;
    // private readonly account: Account;
    // private readonly fx?: Fx;
    // private readonly transfer: Transfer;
    // readonly type: MovementType;

    constructor(amount: number, direction: number, date: string, date_value: string, quantity?: number, unit_value?: number) {
        this.amount = amount;
        this.quantity = quantity;
        this.unit_value = unit_value
        this.direction = direction
        this.date = date
        this.date_value = date_value
    }
}
