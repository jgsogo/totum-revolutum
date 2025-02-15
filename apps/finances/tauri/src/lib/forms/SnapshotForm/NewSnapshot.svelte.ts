import { Account, MoneyAmount, MoneyAmountNonNumerable, MoneyAmountNumerable, Snapshot } from "../../../../models/src-js";
import { Snapshot as SnapshotModel } from "../../../../models/src-js";
import { DateWrapper, sort_date_wrapper } from "../../../../../../../libraries/googleapis/src-js/date";
import { Money as MoneyModel } from "../../../../../../../libraries/googleapis/src-js/money";
import { Decimal as DecimalModel } from "../../../../../../../libraries/googleapis/src-js/decimal";

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
        let date_value = DateWrapper.create_from_date(this.date_value);
        if (sort_date_wrapper(date_value, this.account.open()) < 0) {
            this.error_date_value = "Cannot take an snapshot before the account was opened.";
        }
        let today = new Date();
        let today_date = DateWrapper.create_from_date(today);
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

    toMessage(): SnapshotModel {
        let date_wrapper = DateWrapper.create_from_date(this.date_value);
        let money_amount = MoneyAmount.create_from(
            this.account.is_numerable() ?
                MoneyAmountNumerable.create_from(MoneyModel.create_from_number(this.account.ccy(), this.unit_value!), DecimalModel.create_from_number(this.quantity!)) :
                MoneyAmountNonNumerable.create_from(MoneyModel.create_from_number(this.account.ccy(), this.amount!)));
        let data = SnapshotModel.create_from(this.account.pk(), date_wrapper, money_amount)
        return data;
    }
};
