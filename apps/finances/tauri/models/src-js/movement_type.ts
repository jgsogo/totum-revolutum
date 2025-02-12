import { MovementType as MovementTypeProto } from "../protos/movement_pb.js";
import { Breadcrumb } from "./breadcrumb.js";

export class MovementType {
    private readonly data: MovementTypeProto;

    constructor(movement_type: MovementTypeProto) {
        this.data = movement_type;
    }

    toString(): string {
        return this.name()
    }

    pk(): number {
        return Number(this.data.pk);
    }

    name(): string {
        return this.data.name
    }

    breadcrumb(): Breadcrumb | undefined {
        return this.data.breadcrumb ? new Breadcrumb(this.data.breadcrumb) : undefined;
    }

    as_proto(): MovementTypeProto {
        return this.data;
    }
}
