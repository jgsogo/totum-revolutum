import { Action as ActionProto, ActionSchema } from "../../models/board_pb.js"
import { OutgoingMessage } from "./message.js";
import { toBinary, create } from "@bufbuild/protobuf";

export class Action extends OutgoingMessage {
    private readonly data: ActionProto;

    constructor(position: number) {
        super();

        this.data = create(ActionSchema, { position }) as ActionProto;
    }

    toBinary(): Uint8Array {
        return toBinary(ActionSchema, this.data);
    }
}
