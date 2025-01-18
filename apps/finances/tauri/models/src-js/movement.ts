import { Movement as MovementProto, MovementDirection, NewMovement as NewMovementProto, NewMovementSchema, NewMovement_DividendAmount, NewMovement_DividendAmountSchema } from "../protos/movement_pb.js";
import { MoneyAmount } from "./money_amount.js";
import { DateWrapper } from "../../../../../libraries/googleapis/src-js/date.js";
import { create } from "@bufbuild/protobuf";
import { MoneyAmount_NonNumerable, MoneyAmount_Numerable, MoneyAmount_NonNumerableSchema, MoneyAmount_NumerableSchema } from "../protos/money_amount_pb.js";
import { Money } from "../../../../../libraries/googleapis/src-js/money.js";
import { Decimal } from "decimal.js";
import { DecimalSchema, Decimal as DecimalProto } from "../../../../../libraries/googleapis/protos/google/type/decimal_pb.js";
import { Account } from "./account.js";
import { MovementType } from "./movement_type.js";
import { Snapshot } from "./snapshot.js";


export class Movement {
    private readonly movement: MovementProto;

    constructor(movement: MovementProto) {
        this.movement = movement;
    }

    pk(): number {
        return Number(this.movement.pk);
    }

    dateValue(): DateWrapper {
        return new DateWrapper(this.movement.dateValue!);
    }

    amount(): MoneyAmount {
        return new MoneyAmount(this.movement.amount!);
    }

    direction(): string {
        switch (this.movement.direction) {
            case MovementDirection.In:
                return "IN";
            case MovementDirection.Out:
                return "OUT";
            default:
                return `<unknown '${this.movement.direction}'>`;
        }
    }
}



export class NewMovement {
    private data: NewMovementProto;
    private readonly ccy: string;

    constructor(account: Account, movement_type: MovementType, date_value: Date) {
        // TODO: Add FX
        let dateValue = DateWrapper.create_from_date(date_value);
        this.data = create(NewMovementSchema, { accountPk: BigInt(account.pk()), movementTypePk: BigInt(movement_type.pk()), dateValue: dateValue.as_proto() });
        this.ccy = account.ccy();
    }

    innerType(): NewMovementProto {
        return this.data;
    }

    setNonNumerableAmount(amount: number) {
        let _amount = Money.create_from_number(this.ccy, amount);
        let non_numerable_amount: MoneyAmount_NonNumerable = create(MoneyAmount_NonNumerableSchema, { amount: _amount.innerType() }) as MoneyAmount_NonNumerable;
        this.data.amount = {
            case: "nonNumerable",
            value: non_numerable_amount
        };
    }

    setNumerableAmount(quantity: number, unit_value: number) {
        let _quantity = create(DecimalSchema, { value: new Decimal(quantity).toString() }) as DecimalProto;
        let _unit_value = Money.create_from_number(this.ccy, unit_value);

        let numerable_amount: MoneyAmount_Numerable = create(MoneyAmount_NumerableSchema, { unitValue: _unit_value.innerType(), quantity: _quantity }) as MoneyAmount_Numerable;
        this.data.amount = {
            case: "numerable",
            value: numerable_amount
        };
    }

    setDividendAmount(payout: number, ex_dividend_date: Date, ex_dividend_snapshot: Snapshot) {
        let _payout = Money.create_from_number(this.ccy, payout);
        // FIXME: Get the quantity from the `ex_dividend_snapshot`
        let _quantity = create(DecimalSchema, { value: new Decimal(0).toString() }) as DecimalProto;
        let exDividendDate = DateWrapper.create_from_date(ex_dividend_date);

        let numerable_amount: MoneyAmount_Numerable = create(MoneyAmount_NumerableSchema, { unitValue: _payout.innerType(), quantity: _quantity }) as MoneyAmount_Numerable;

        let dividend_amount: NewMovement_DividendAmount = create(NewMovement_DividendAmountSchema, {
            exDividendDate: exDividendDate.as_proto(),
            exDividendSnapshotPk: BigInt(ex_dividend_snapshot.pk()),
            payout: numerable_amount
        }) as NewMovement_DividendAmount;
        this.data.amount = {
            case: "dividend",
            value: dividend_amount
        };
    }
}
