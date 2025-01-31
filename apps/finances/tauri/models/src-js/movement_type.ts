import { MovementType as MovementTypeProto } from "../protos/movement_pb.js";

export class MovementType {
    private readonly movement_type: MovementTypeProto;

    constructor(movement_type: MovementTypeProto) {
        this.movement_type = movement_type;
    }

    toString() : string {
        return this.name()
    }

    pk(): number {
        return Number(this.movement_type.pk);
    }

    name(): string {
        return this.movement_type.name
    }

    breadcrumb(): string[] {
        return this.movement_type.breadcrumb;
    }

    as_proto(): MovementTypeProto {
        return this.movement_type;
    }
}
