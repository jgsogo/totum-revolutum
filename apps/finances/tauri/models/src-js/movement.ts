import { Movement as MovementProto, MovementDirection } from "../protos/movement_pb.js";
import { MoneyAmount } from "./money_amount.js";
import { DateWrapper } from "../../../../../libraries/googleapis/src-js/date.js";


export class Movement {
    private readonly movement: MovementProto;

    constructor(movement: MovementProto) {
        this.movement = movement;
    }

    pk(): number {
        return this.movement.pk as unknown as number;
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
