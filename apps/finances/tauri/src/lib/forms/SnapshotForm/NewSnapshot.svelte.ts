import { Account, MoneyAmount, MoneyAmountNonNumerable, MoneyAmountNumerable, Snapshot } from "../../../../models/src-js";
import { Snapshot as SnapshotModel } from "../../../../models/src-js";
import { DateWrapper, sort_date_wrapper } from "../../../../../../../libraries/googleapis/src-js/date";
import { Money as MoneyModel } from "../../../../../../../libraries/googleapis/src-js/money";
import { Decimal as DecimalModel } from "../../../../../../../libraries/googleapis/src-js/decimal";

export class NewSnapshot {
    readonly account: Account;

    date_value: Date = new Date();
    amount?: number = undefined;
    quantity?: number = undefined;
    unit_value?: number = undefined;

    constructor(account: Account, amount?: number, quantity?: number, unit_value?: number) {
        this.account = account;
        this.amount = amount;
        this.quantity = quantity;
        this.unit_value = unit_value;
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
