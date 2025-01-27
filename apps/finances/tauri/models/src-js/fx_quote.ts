import { create } from "@bufbuild/protobuf";
import { currency_code_from_str, CurrencyCode, DateWrapper, Decimal } from "../../../../../libraries/googleapis/src-js/index.js";
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
}


export class NewFxQuote {
    private data: FxQuoteProto;

    constructor(date_value: DateWrapper, base_ccy: CurrencyCode, quote_ccy: CurrencyCode, rate: Decimal) {
        this.data = create(FxQuoteSchema, {
            dateValue: date_value.as_proto(),
            baseCcyCode: base_ccy.toString(),
            quoteCcyCode: quote_ccy.toString(),
            quote: rate.as_proto()
        });
    }

    as_proto(): FxQuoteProto {
        return this.data;
    }
}
