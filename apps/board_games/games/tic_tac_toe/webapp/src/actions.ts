import { Action as ActionProto, ActionSchema, ActionPlaceMark as ActionPlaceMarkProto, ActionPlaceMarkSchema } from "../../models/actions_pb.js"
import { OutgoingMessage } from "./message.js";
import { toBinary, create } from "@bufbuild/protobuf";


export class Action extends OutgoingMessage {
    private readonly data: ActionProto;

    constructor(action: ActionPlaceMark) {
        super();

        if (action instanceof ActionPlaceMark) {
            this.data = create(ActionSchema, { place_mark: action }) as ActionProto;
        } else {
            throw new Error("Unknown action type");
        }
    }

    toBinary(): Uint8Array {
        return toBinary(ActionSchema, this.data);
    }
}


export class ActionPlaceMark extends OutgoingMessage {
    private readonly data: ActionPlaceMarkProto;

    constructor(position: number) {
        super();

        this.data = create(ActionPlaceMarkSchema, { position }) as ActionPlaceMarkProto;
    }

    toBinary(): Uint8Array {
        return toBinary(ActionPlaceMarkSchema, this.data);
    }
}
