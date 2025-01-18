import { Decimal as DecimalProto, DecimalSchema } from "../protos/google/type/decimal_pb.js";
import { Decimal as DecimalJS } from "decimal.js";
import { create } from "@bufbuild/protobuf";
export class Decimal {
    private readonly decimal: DecimalProto;

    constructor(decimal: DecimalProto) {
        this.decimal = decimal;
    }

    static create_from_number(value: number): Decimal {
        let dec = new DecimalJS(value)
        let proto = create(DecimalSchema, { value: dec.toString() }) as DecimalProto;
        return new Decimal(proto);
    }

    as_number(): number {
        return this.as_decimal().toNumber();
    }

    as_decimal(): DecimalJS {
        return new DecimalJS(this.decimal.value);
    }

    as_proto(): DecimalProto {
        return this.decimal;
    }
}
