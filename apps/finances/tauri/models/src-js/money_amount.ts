import { MoneyAmount_Numerable as MoneyAmount_NumerableProto, MoneyAmount_NonNumerable as MoneyAmount_NonNumerableProto, MoneyAmount as MoneyAmountProto, MoneyAmountSchema, MoneyAmount_NumerableSchema, MoneyAmount_NonNumerableSchema } from "../protos/money_amount_pb.js";
import { Money } from "../../../../../libraries/googleapis/src-js/index.js";
import { Decimal } from "../../../../../libraries/googleapis/src-js/index.js";
import { create } from "@bufbuild/protobuf";


export class MoneyAmountNumerable {
    private readonly proto: MoneyAmount_NumerableProto;

    constructor(proto: MoneyAmount_NumerableProto) {
        this.proto = proto;
    }

    unit_value(): Money {
        return new Money(this.proto.unitValue!);
    }

    quantity(): Decimal {
        return new Decimal(this.proto.quantity!);
    }

    amount(): Money {
        const amount = this.unit_value().amount() * this.quantity().as_number();
        return Money.create_from_number(this.unit_value().currency_code(), amount);
    }

    static create_from(unit_value: Money, quantity: Decimal): MoneyAmountNumerable {
        const proto = create(MoneyAmount_NumerableSchema, { unitValue: unit_value.as_proto(), quantity: quantity.as_proto() });
        return new MoneyAmountNumerable(proto)
    }

    as_proto(): MoneyAmount_NumerableProto {
        return this.proto
    }
}

export class MoneyAmountNonNumerable {
    private readonly proto: MoneyAmount_NonNumerableProto;

    constructor(proto: MoneyAmount_NonNumerableProto) {
        this.proto = proto;
    }

    amount(): Money {
        return new Money(this.proto.amount!);
    }

    static create_from(amount: Money): MoneyAmountNonNumerable {
        const proto = create(MoneyAmount_NonNumerableSchema, { amount: amount.as_proto() });
        return new MoneyAmountNonNumerable(proto)
    }

    as_proto(): MoneyAmount_NonNumerableProto {
        return this.proto
    }
}

export class MoneyAmount {
    private readonly money_amount: MoneyAmountProto;

    constructor(money_amount: MoneyAmountProto) {
        this.money_amount = money_amount;
    }

    as_numerable(): MoneyAmountNumerable | undefined {
        if (this.money_amount.amount.case == "numerable") {
            return new MoneyAmountNumerable(this.money_amount.amount.value);
        } else {
            return undefined;
        }
    }

    as_non_numerable(): MoneyAmountNonNumerable | undefined {
        if (this.money_amount.amount.case == "nonNumerable") {
            return new MoneyAmountNonNumerable(this.money_amount.amount.value);
        } else {
            return undefined;
        }
    }

    amount(): Money {
        switch (this.money_amount.amount.case) {
            case "nonNumerable":
                return this.as_non_numerable()!.amount();
            case "numerable":
                return this.as_numerable()!.amount();
            default:
                throw new Error(`MoneyAmount alternative not handled: ${this.money_amount.amount.case}`);
        }
    }

    static create_from(amount: MoneyAmountNumerable | MoneyAmountNonNumerable): MoneyAmount {
        const proto = create(MoneyAmountSchema, {});
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
        } else {
            throw new Error(`Unexpected amount type: ${typeof amount}`)
        }

        return new MoneyAmount(proto)
    }

    as_proto(): MoneyAmountProto {
        return this.money_amount
    }
}
