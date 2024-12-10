import type { Account } from "$lib/models/Account";



export class NewSnapshot {
    readonly account: Account;

    date_value: Date = $state(new Date());
    amount?: number = $state();
    quantity?: number = $state();
    unit_value?: number = $state();

    error_date_value?: string = $state();
    error_amount?: string = $state();
    error_quantity?: string = $state();
    error_unit_value?: string = $state();

    constructor(account: Account) {
        this.account = account;
        const last_snapshot = account.last_snapshot();
        if (last_snapshot) {
            this.amount = last_snapshot.amount;
            this.quantity = last_snapshot.quantity;
            this.unit_value = last_snapshot.unit_value;
        }
    }

    cleanErrors() {
        this.error_date_value = undefined;
        this.error_amount = undefined;
        this.error_quantity = undefined;
        this.error_unit_value = undefined;
    }

    isValid(): boolean {
        this.cleanErrors();

        // Validate date_value
        if (this.date_value < this.account.open) {
            this.error_date_value = "Cannot take an snapshot before the account was opened.";
        }
        if (this.date_value > new Date()) {
            this.error_date_value = "Cannot take an snapshot of the future.";
        }

        // Validate amount, quantity and unit_value
        if (this.account.is_numerable) {
            if (this.quantity === undefined || this.quantity < 0) {
                this.error_quantity = "Positive value required.";
            }
            if (this.unit_value === undefined || this.unit_value < 0) {
                this.error_unit_value = "Positive value required.";
            }

        } else {
            if (this.amount === undefined || this.amount < 0) {
                this.error_amount = "Positive value required.";
            }
        }

        return !(this.error_date_value || this.error_quantity || this.error_unit_value || this.error_amount)
    }

    toJSON() {
        return {
            account: this.account,
            date_value: this.date_value,
            amount: this.amount,
            quantity: this.quantity,
            unit_value: this.unit_value,
        };
    }
};
