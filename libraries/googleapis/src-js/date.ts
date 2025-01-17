import { Date as DateProto, DateSchema } from "../protos/google/type/date_pb.js";
import { create } from "@bufbuild/protobuf";

export class DateWrapper {
    private readonly date: DateProto;

    constructor(date: DateProto) {
        this.date = date;
    }

    static create_from_date(date: Date): DateWrapper {
        let month = date.getMonth() + 1; // It's zero based!
        let day = date.getDate(); // Yes, name is confusing
        let proto = create(DateSchema, { year: date.getFullYear(), month, day }) as DateProto;
        return new DateWrapper(proto);
    }

    as_date(): Date {
        let date = new Date(this.date.year, this.date.month, this.date.day);
        return date;
    }

    as_proto(): DateProto {
        return this.date;
    }
}
