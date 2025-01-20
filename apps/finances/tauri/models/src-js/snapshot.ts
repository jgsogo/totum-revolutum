import { Snapshot as SnapshotProto, NewSnapshot as NewSnapshotProto, NewSnapshotSchema } from "../protos/snapshot_pb.js";
import { MoneyAmount } from "./money_amount.js";
import { DateWrapper } from "../../../../../libraries/googleapis/src-js/date.js";
import { create, toBinary } from "@bufbuild/protobuf";
import { MoneyAmount_NonNumerable, MoneyAmount_Numerable, MoneyAmount_NonNumerableSchema, MoneyAmount_NumerableSchema, MoneyAmountSchema } from "../protos/money_amount_pb.js";
import { MoneyAmount as MoneyAmountProto } from "../protos/money_amount_pb.js"
import { Money } from "../../../../../libraries/googleapis/src-js/money.js";
import { Decimal } from "decimal.js";
import { DecimalSchema, Decimal as DecimalProto } from "../../../../../libraries/googleapis/protos/google/type/decimal_pb.js";
import { OutgoingMessage } from "./message.js";

export class Snapshot {
    private readonly snapshot: SnapshotProto;

    constructor(snapshot: SnapshotProto) {
        this.snapshot = snapshot;
    }

    pk(): number {
        return Number(this.snapshot.pk);
    }

    dateValue(): DateWrapper {
        return new DateWrapper(this.snapshot.dateValue!);
    }

    amount(): MoneyAmount {
        return new MoneyAmount(this.snapshot.amount!);
    }
}


export class NewSnapshot extends OutgoingMessage {
    private data: NewSnapshotProto;

    constructor() {
        super();
        this.data = create(NewSnapshotSchema, {});
    }

    toBinary(): Uint8Array {
        return toBinary(NewSnapshotSchema, this.data);
    }

    setAccountPk(account_pk: number) {
        this.data.accountPk = BigInt(account_pk)
    }

    setDate(date: DateWrapper) {
        this.data.dateValue = date.as_proto();
    }

    setNonNumerableAmount(ccy: string, amount: number) {
        let _amount = Money.create_from_number(ccy, amount);
        let non_numerable_amount: MoneyAmount_NonNumerable = create(MoneyAmount_NonNumerableSchema, { amount: _amount.innerType() }) as MoneyAmount_NonNumerable;
        let money_amount = create(MoneyAmountSchema, {
            amount: {
                case: "nonNumerable",
                value: non_numerable_amount,
            }
        }) as MoneyAmountProto;
        this.data.amount = money_amount;
    }

    setNumerableAmount(ccy: string, quantity: number, unit_value: number) {
        let _quantity = create(DecimalSchema, { value: new Decimal(quantity).toString() }) as DecimalProto;
        let _unit_value = Money.create_from_number(ccy, unit_value);

        let numerable_amount: MoneyAmount_Numerable = create(MoneyAmount_NumerableSchema, { unitValue: _unit_value.innerType(), quantity: _quantity }) as MoneyAmount_Numerable;
        let money_amount = create(MoneyAmountSchema, {
            amount: {
                case: "numerable",
                value: numerable_amount,
            }
        }) as MoneyAmountProto;
        this.data.amount = money_amount;
    }

}
