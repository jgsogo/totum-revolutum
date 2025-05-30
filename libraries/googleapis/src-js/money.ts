import { Money as MoneyProto, MoneySchema } from "../protos/google/type/money_pb.js";
import { create } from "@bufbuild/protobuf";

export enum CurrencyCode {
    EUR = "EUR",
    USD = "USD",
}

export function ccy_symbol(ccy: CurrencyCode): string {
    if (ccy === CurrencyCode.EUR) {
        return "€";
    } else if (ccy === CurrencyCode.USD) {
        return "$";
    } else {
        return ccy;
    }
}

function enumFromStringValue<T>(enm: { [s: string]: T }, value: string): T | undefined {
    return (Object.values(enm) as unknown as string[]).includes(value)
        ? value as unknown as T
        : undefined;
}

export function currency_code_from_str(ccy: string): CurrencyCode | undefined {
    return enumFromStringValue(CurrencyCode, ccy.toUpperCase());
}


export class Money {
    private readonly money: MoneyProto;

    static create_from_number(currencyCode: CurrencyCode, amount: number): Money {
        let units = Math.trunc(amount);
        let decimal_part = Math.round((amount - units) * Math.pow(10, 9));
        let proto = create(MoneySchema, { currencyCode: currencyCode.toString(), units: BigInt(units), nanos: decimal_part }) as MoneyProto;
        return new Money(proto);
    }

    constructor(money: MoneyProto) {
        this.money = money;
    }

    amount(): number {
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

    currency_code(): CurrencyCode {
        return currency_code_from_str(this.money.currencyCode)!;
    }

    toString(locale?: string): string {
        locale = locale ?? 'es-ES';

        const formatter = Intl.NumberFormat(locale, {
            style: 'currency',
            currency: this.money.currencyCode,
            notation: 'standard',
            currencySign: "accounting",
            // signDisplay: "always",
            minimumFractionDigits: 2,
            maximumFractionDigits: 2,
            currencyDisplay: "code",
        });

        return formatter.format(this.amount());
    }

    as_proto(): MoneyProto {
        return this.money;
    }

    sum(other: Money): Money {
        if (this.currency_code() != other.currency_code()) {
            throw new Error(`Cannot sum ${this.currency_code()} with ${other.currency_code()}`);
        }
        return Money.create_from_number(this.currency_code(), this.amount() + other.amount());
    }

    substract(other: Money): Money {
        if (this.currency_code() != other.currency_code()) {
            throw new Error(`Cannot sum ${this.currency_code()} with ${other.currency_code()}`);
        }
        return Money.create_from_number(this.currency_code(), this.amount() - other.amount());
    }

    equal(other: Money): boolean {
        return ((this.currency_code() === other.currency_code()) && this.amount() === other.amount());
    }

    equal_with_tolerance(other: Money, tolerance: number): boolean {
        return this.currency_code() === other.currency_code() && Math.abs(this.amount() - other.amount()) < tolerance;
    }
}
