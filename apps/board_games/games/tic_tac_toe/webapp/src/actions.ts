import { Action as ActionProto, ActionSchema, ActionPlaceMarkSchema } from "../../models/actions_pb.js"
import { OutgoingMessage } from "./message.js";
import { toBinary, create } from "@bufbuild/protobuf";


export class Action extends OutgoingMessage {
    private readonly proto: ActionProto;

    constructor(proto: ActionProto) {
        super();
        this.proto = proto;
    }

    static create_place_mark(position: number): Action {
        const proto = create(ActionSchema, {
            action: {
                case: "placeMark",
                value: create(ActionPlaceMarkSchema, {
                position,
                }),
            },
            }) as ActionProto;
        return new Action(proto)
    }

    toBinary(): Uint8Array {
        return toBinary(ActionSchema, this.proto);
    }
}
