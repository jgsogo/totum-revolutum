import { Holder as HolderProto } from "../protos/holder_pb.js";

export class Holder {
    private readonly proto: HolderProto;

    constructor(proto: HolderProto) {
        this.proto = proto;
    }

    pk(): number {
        return Number(this.proto.pk);
    }

    name(): string {
        return this.proto.name;
    }

    is_company(): boolean {
        return this.proto.isCompany
    }

    photo(): string | undefined {
        return this.proto.photo
    }
}
