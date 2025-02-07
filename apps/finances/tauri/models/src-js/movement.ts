import { Movement as MovementProto, MovementAmount as MovementAmountProto, MovementDirection as MovementDirectionProto, MovementAmount_Dividend as MovementAmount_DividendProto, MovementAmount_DividendSchema, MovementAmountSchema, MovementSchema } from "../protos/movement_pb.js";
import { MoneyAmountNonNumerable, MoneyAmountNumerable } from "./money_amount.js";
import { DateWrapper } from "../../../../../libraries/googleapis/src-js/date.js";
import { create } from "@bufbuild/protobuf";
import { Money } from "../../../../../libraries/googleapis/src-js/money.js";
import { Account } from "./account.js";
import { MovementType } from "./movement_type.js";

import { FxQuote } from "./fx_quote.js";


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


    static create_from(ex_dividend_date: DateWrapper, payout: MoneyAmountNumerable): MovementAmountDividend {
        const proto = create(MovementAmount_DividendSchema, { exDividendDate: ex_dividend_date.as_proto(), payout: payout.as_proto() });
        return new MovementAmountDividend(proto)
    }

    as_proto(): MovementAmount_DividendProto {
        return this.proto
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

    static create_from(amount: MoneyAmountNumerable | MoneyAmountNonNumerable | MovementAmountDividend): MovementAmount {
        let proto = create(MovementAmountSchema, {});

        if (amount instanceof MoneyAmountNumerable) {
            proto.amount = {
                case: "numerable",
                value: amount.as_proto(),
            }
        } else if (amount instanceof MoneyAmountNonNumerable) {
            proto.amount = {
                case: "nonNumerable",
                value: amount.as_proto(),
            }
        } else if (amount instanceof MovementAmountDividend) {
            proto.amount = {
                case: "dividend",
                value: amount.as_proto(),
            }
        }
        else {
            throw new Error(`Unexpected amount type: ${typeof amount}`)
        }

        return new MovementAmount(proto)
    }

    as_proto(): MovementAmountProto {
        return this.proto
    }

}


export class Movement {
    private readonly movement: MovementProto;

    constructor(movement: MovementProto) {
        this.movement = movement;
    }

    pk(): number | undefined {
        return this.movement.pk ? Number(this.movement.pk) : undefined;
    }

    date_value(): DateWrapper {
        return new DateWrapper(this.movement.dateValue!);
    }

    transaction_pk(): number | undefined {
        return this.movement.transactionPk ? Number(this.movement.transactionPk) : undefined;
    }

    transaction_name(): string | undefined {
        return this.movement.transactionName;
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

    /// Returns the Money amount in the base currency (use [`Self::movement_amount`] to get the raw information)
    amount(): Money {
        let raw_amount = this.movement_amount().amount();
        let fx_quote = this.fx_quote();
        if (fx_quote) {
            return fx_quote.apply_to(raw_amount);
        } else {
            return raw_amount;
        }
    }

    fx_quote(): FxQuote | undefined {
        return this.movement.fx ? new FxQuote(this.movement.fx!) : undefined;
    }

    account_pk(): number {
        return Number(this.movement.accountPk);
    }


    static create_from(date_value: DateWrapper, transaction_pk: number | undefined, movement_type: MovementType, direction: MovementDirection, movement_amount: MovementAmount, account: Account, fx_quote?: FxQuote): Movement {
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

        const proto = create(MovementSchema, {
            dateValue: date_value.as_proto(),
            transactionPk: transaction_pk ? BigInt(transaction_pk) : undefined,
            type: movement_type.as_proto(),
            direction: mov_direction,
            amount: movement_amount.as_proto(),
            fx: fx_quote ? fx_quote.as_proto() : undefined,
            accountPk: BigInt(account.pk())
        });
        return new Movement(proto)
    }

    as_proto(): MovementProto {
        return this.movement
    }
}
