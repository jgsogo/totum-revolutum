import { Account, Snapshot } from "../../../../models/src-js";
import { NewSnapshot as NewSnapshotModel } from "../../../../models/src-js/snapshot";
import { DateWrapper, sort_date_wrapper } from "../../../../../../../libraries/googleapis/src-js/date";

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

    constructor(account: Account, last_snapshot?: Snapshot) {
        this.account = account;
        if (last_snapshot) {
            this.amount = last_snapshot.amount().amount().amount();
            if (account.is_numerable()) {
                let numerable = last_snapshot.amount().as_numerable()!;
                this.quantity = numerable.quantity().as_number();
                this.unit_value = numerable.unit_value().amount();
            }
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
        let date_value = DateWrapper.create_from_yyyy_mm_dd(this.date_value.getFullYear(), this.date_value.getMonth() + 1, this.date_value.getDate());
        if (sort_date_wrapper(date_value, this.account.open()) < 0) {
            this.error_date_value = "Cannot take an snapshot before the account was opened.";
        }
        let today = new Date();
        let today_date = DateWrapper.create_from_yyyy_mm_dd(today.getFullYear(), today.getMonth() + 1, today.getDate());
        if (sort_date_wrapper(today_date, date_value) < 0) {
            this.error_date_value = "Cannot take an snapshot of the future.";
        }

        // Validate amount, quantity and unit_value
        if (this.account.is_numerable()) {
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

    toMessage(): NewSnapshotModel {
        let data: NewSnapshotModel = new NewSnapshotModel();
        data.setAccountPk(this.account.pk());

        let date_wrapper = DateWrapper.create_from_yyyy_mm_dd(this.date_value.getFullYear(), this.date_value.getMonth() + 1, this.date_value.getDate());
        data.setDate(date_wrapper);

        if (this.account.is_numerable()) {
            data.setNumerableAmount(this.account.ccy(), this.quantity!, this.unit_value!);
        } else {
            data.setNonNumerableAmount(this.account.ccy(), this.amount!);
        }
        return data;
    }
};
