import { Movement as MovementProto, MovementAmount as MovementAmountProto, MovementDirection as MovementDirectionProto, MovementAmount_Dividend as MovementAmount_DividendProto, MovementAmount_DividendSchema, MovementAmountSchema, MovementSchema } from "../protos/movement_pb.js";
import { MoneyAmountNonNumerable, MoneyAmountNumerable, NewMoneyAmountNonNumerable, NewMoneyAmountNumerable } from "./money_amount.js";
import { DateWrapper } from "../../../../../libraries/googleapis/src-js/date.js";
import { create } from "@bufbuild/protobuf";
import { Money } from "../../../../../libraries/googleapis/src-js/money.js";
import { Account } from "./account.js";
import { MovementType } from "./movement_type.js";

import { FxQuote, NewFxQuote } from "./fx_quote.js";


export enum MovementDirection {
    In = 0,
    Out = 1,
}

export class MovementAmountDividend {
    private readonly proto: MovementAmount_DividendProto;

    constructor(proto: MovementAmount_DividendProto) {
        this.proto = proto;
    }

    ex_dividend_date(): DateWrapper {
        return new DateWrapper(this.proto.exDividendDate!);
    }

    payout(): MoneyAmountNumerable {
        return new MoneyAmountNumerable(this.proto.payout!);
    }
}

export class NewMovementAmountDividend {
    private data: MovementAmount_DividendProto;

    constructor(ex_dividend_date: DateWrapper, payout: NewMoneyAmountNumerable) {
        this.data = create(MovementAmount_DividendSchema, { exDividendDate: ex_dividend_date.as_proto(), payout: payout.as_proto() });
    }

    as_proto(): MovementAmount_DividendProto {
        return this.data
    }
}

export class MovementAmount {
    private readonly proto: MovementAmountProto;

    constructor(proto: MovementAmountProto) {
        this.proto = proto;
    }

    as_numerable(): MoneyAmountNumerable | undefined {
        if (this.proto.amount.case == "numerable") {
            return new MoneyAmountNumerable(this.proto.amount.value);
        } else {
            return undefined;
        }
    }

    as_non_numerable(): MoneyAmountNonNumerable | undefined {
        if (this.proto.amount.case == "nonNumerable") {
            return new MoneyAmountNonNumerable(this.proto.amount.value);
        } else {
            return undefined;
        }
    }

    as_dividend(): MovementAmountDividend | undefined {
        if (this.proto.amount.case == "dividend") {
            return new MovementAmountDividend(this.proto.amount.value);
        } else {
            return undefined;
        }
    }

    amount(): Money {
        switch (this.proto.amount.case) {
            case "nonNumerable":
                return this.as_non_numerable()!.amount();
            case "numerable":
                return this.as_numerable()!.amount();
            case "dividend":
                return this.as_dividend()!.payout().amount();
            default:
                throw new Error(`MoneyAmount alternative not handled: ${this.proto.amount.case}`);
        }
    }
}

export class NewMovementAmount {
    private data: MovementAmountProto;

    constructor(amount: NewMoneyAmountNumerable | NewMoneyAmountNonNumerable | NewMovementAmountDividend) {
        this.data = create(MovementAmountSchema, {});

        if (amount instanceof NewMoneyAmountNumerable) {
            this.data.amount = {
                case: "numerable",
                value: amount.as_proto(),
            }
        } else if (amount instanceof NewMoneyAmountNonNumerable) {
            this.data.amount = {
                case: "nonNumerable",
                value: amount.as_proto(),
            }
        } else if (amount instanceof NewMovementAmountDividend) {
            this.data.amount = {
                case: "dividend",
                value: amount.as_proto(),
            }
        }
        else {
            throw new Error(`Unexpected amount type: ${typeof amount}`)
        }
    }

    as_proto(): MovementAmountProto {
        return this.data;
    }
}

export class Movement {
    private readonly movement: MovementProto;

    constructor(movement: MovementProto) {
        this.movement = movement;
    }

    pk(): number {
        return Number(this.movement.pk);
    }

    date_value(): DateWrapper {
        return new DateWrapper(this.movement.dateValue!);
    }

    transaction_pk(): number | undefined {
        return Number(this.movement.transactionPk);
    }

    type(): MovementType {
        return new MovementType(this.movement.type!);
    }

    direction(): MovementDirection {
        switch (this.movement.direction) {
            case MovementDirectionProto.In:
                return MovementDirection.In;
            case MovementDirectionProto.Out:
                return MovementDirection.Out;
            default:
                throw new Error(`<unknown MovementDirection '${this.movement.direction}'>`);
        }
    }

    movement_amount(): MovementAmount {
        return new MovementAmount(this.movement.amount!);
    }

    amount(): Money {
        return this.movement_amount().amount();
    }

    fx_quote(): FxQuote | undefined {
        return this.movement.fx ? new FxQuote(this.movement.fx!) : undefined;
    }

    account_pk(): number {
        return Number(this.movement.accountPk);
    }

}

export class NewMovement {
    private data: MovementProto;

    constructor(date_value: DateWrapper, transaction_pk: number, movement_type: MovementType, direction: MovementDirection, movement_amount: NewMovementAmount, account: Account, fx_quote?: NewFxQuote) {
        let mov_direction: MovementDirectionProto = MovementDirectionProto.In;
        switch (direction) {
            case MovementDirection.In:
                mov_direction = MovementDirectionProto.In;
                break;
            case MovementDirection.Out:
                mov_direction = MovementDirectionProto.Out;
                break;
            default:
                throw new Error(`<unknown MovementDirection '${mov_direction}'>`);
        }

        this.data = create(MovementSchema, {
            dateValue: date_value.as_proto(),
            transactionPk: BigInt(transaction_pk),
            type: movement_type.as_proto(),
            direction: mov_direction,
            amount: movement_amount.as_proto(),
            fx: fx_quote ? fx_quote.as_proto() : undefined,
            accountPk: BigInt(account.pk())
        });
    }

    as_proto(): MovementProto {
        return this.data
    }
}
