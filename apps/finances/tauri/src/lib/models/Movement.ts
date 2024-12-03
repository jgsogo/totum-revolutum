import type {Account} from "$lib/models/Account";

export class NewMovement {
    amount?: number;
    date_value?: Date;
    quantity?: number;
    unit_value?: number;
    account?: Account;
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
