import { Custodian as CustodianProto } from "../protos/custodian_pb.js";

export class Custodian {
    private readonly custodian: CustodianProto;

    constructor(custodian: CustodianProto) {
        this.custodian = custodian;
    }

    pk(): number {
        return Number(this.custodian.pk);
    }

    name(): string {
        return this.custodian.name;
    }

    photo(): string | undefined {
        return this.custodian.photo
    }

    as_proto(): CustodianProto {
        return this.custodian;
    }
}
