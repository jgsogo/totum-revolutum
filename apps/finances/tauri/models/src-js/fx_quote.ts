import { create } from "@bufbuild/protobuf";
import { currency_code_from_str, CurrencyCode, DateWrapper, Decimal, Money } from "../../../../../libraries/googleapis/src-js/index.js";
import { FxQuote as FxQuoteProto, FxQuoteSchema } from "../protos/fx_quote_pb.js";

export class FxQuotePair {
    public readonly base: CurrencyCode;
    public readonly quote: CurrencyCode;

    constructor(base: CurrencyCode, quote: CurrencyCode) {
        this.base = base;
        this.quote = quote;
    }

    toString(): string {
        return `${this.base}/${this.quote}`
    }
}

export class FxQuote {
    private readonly proto: FxQuoteProto;

    constructor(proto: FxQuoteProto) {
        this.proto = proto;
    }

    date_value(): DateWrapper {
        return new DateWrapper(this.proto.dateValue!);
    }

    fx_pair(): FxQuotePair {
        const base_ccy = currency_code_from_str(this.proto.baseCcyCode)!;
        const quote_ccy = currency_code_from_str(this.proto.quoteCcyCode)!;
        return new FxQuotePair(base_ccy, quote_ccy);
    }

    quote(): Decimal {
        return new Decimal(this.proto.quote!);
    }

    /// Applies FX transformation to the given money. It will raise if the CCYs doesn't match
    apply_to(amount: Money): Money {
        let fx_pair = this.fx_pair();
        if (amount.currency_code() == fx_pair.base) {
            let new_amount = amount.amount() * this.quote().as_number();
            return Money.create_from_number(fx_pair.quote, new_amount);
        }
        else if (amount.currency_code() == fx_pair.quote) {
            let new_amount = amount.amount() / this.quote().as_number();
            return Money.create_from_number(fx_pair.base, new_amount);
        }
        else {
            throw new Error(`FxQuote '${fx_pair}' cannot be applied to amount in '${amount.currency_code()}'`)
        }
    }

    static create_from(date_value: DateWrapper, base_ccy: CurrencyCode, quote_ccy: CurrencyCode, rate: Decimal): FxQuote {
    const proto = create(FxQuoteSchema, {
        dateValue: date_value.as_proto(),
        baseCcyCode: base_ccy.toString(),
        quoteCcyCode: quote_ccy.toString(),
        quote: rate.as_proto()
    });
    return new FxQuote(proto)
}

as_proto(): FxQuoteProto {
    return this.proto
}
}
