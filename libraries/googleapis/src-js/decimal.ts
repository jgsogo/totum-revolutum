import { Decimal as DecimalProto } from "../protos/google/type/decimal_pb.js";
import { Decimal as DecimalJS } from "decimal.js";

export class Decimal {
    private readonly decimal: DecimalProto;

    constructor(decimal: DecimalProto) {
        this.decimal = decimal;
    }

    as_number(): number {
        return this.as_decimal().toNumber();
    }

    as_decimal(): DecimalJS {
        return new DecimalJS(this.decimal.value);
    }
}
