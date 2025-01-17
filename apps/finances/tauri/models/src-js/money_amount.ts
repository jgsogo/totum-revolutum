import { MoneyAmount as MoneyAmountProto } from "../protos/money_amount_pb.js";
import { Money } from "../../../../../libraries/googleapis/src-js/money.js";
import { Decimal } from "../../../../../libraries/googleapis/src-js/decimal.js";


export class MoneyAmount {
    private readonly money_amount: MoneyAmountProto;

    constructor(money_amount: MoneyAmountProto) {
        this.money_amount = money_amount;
    }

    as_number(): number {
        switch (this.money_amount.amount.case) {
            case "nonNumerable":
                const money = new Money(this.money_amount.amount.value.amount!);
                return money.as_number();
            case "numerable":
                const unit_value = new Money(this.money_amount.amount.value.unitValue!);
                const quantity = new Decimal(this.money_amount.amount.value.quantity!);
                return unit_value.as_number() * quantity.as_number();
            default:
                throw new Error(`MoneyAmount alternative not handled: ${this.money_amount.amount.case}`);
        }
    }

    toString(locale?: string): string {
        locale = locale ?? 'es-ES';
        const value = this.as_number();
        const ccy_code = this.money_amount.amount.case == "nonNumerable" ? this.money_amount.amount.value.amount!.currencyCode : this.money_amount.amount.value!.unitValue!.currencyCode;


        const formatter = Intl.NumberFormat(locale, {
            style: 'currency',
            currency: ccy_code,
            notation: 'standard'
        });

        return formatter.format(value);
    }

    unit_value(): Money | undefined {
        if (this.money_amount.amount.case == "numerable") return new Money(this.money_amount.amount.value!.unitValue!);
        else return undefined;
    }

    quantity(): Decimal | undefined {
        if (this.money_amount.amount.case == "numerable") return new Decimal(this.money_amount.amount.value!.quantity!);
        else return undefined;
    }

    amount(): Money | undefined {
        if (this.money_amount.amount.case == "nonNumerable") return new Money(this.money_amount.amount.value!.amount!);
        else return undefined;
    }


}
