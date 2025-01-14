import { Money as MoneyProto } from "../protos/google/type/money_pb.js";

export class Money {
    private readonly money: MoneyProto;

    constructor(money: MoneyProto) {
        this.money = money;
    }

    as_number(): number {
        if (this.money.units > Number.MAX_SAFE_INTEGER) {
            throw new Error(`Money amount ${this.money.units} cannot be represented as a (safe) 'Number'`);
        }
        if (this.money.units < Number.MIN_SAFE_INTEGER) {
            throw new Error(`Money amount ${this.money.units} cannot be represented as a (safe) 'Number'`);
        }
        let amount = Number(this.money.units);
        let amount_decimal = this.money.nanos / Math.pow(10, 9);
        return amount + amount_decimal;
    }

    toString(locale?: string): string {
        locale = locale ?? 'es-ES';

        const formatter = Intl.NumberFormat(locale, {
            style: 'currency',
            currency: this.money.currencyCode,
            notation: 'standard'
        });

        return formatter.format(this.as_number());
    }

}
