import { Money as MoneyProto, MoneySchema } from "../protos/google/type/money_pb.js";
import { create, toBinary } from "@bufbuild/protobuf";

export class Money {
    private readonly money: MoneyProto;

    static create_from_number(currencyCode: string, amount: number): Money {
        let units = Math.trunc(amount);
        let decimal_part = (amount - units) * Math.pow(10, 9);
        let proto = create(MoneySchema, { currencyCode, units: BigInt(units), nanos: decimal_part }) as MoneyProto;
        return new Money(proto);
    }

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

    innerType(): MoneyProto {
        return this.money;
    }

    toBinary(): Uint8Array {
        return toBinary(MoneySchema, this.money);
    }

}
