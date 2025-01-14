import { Date as DateProto } from "../protos/google/type/date_pb.js";

export class DateWrapper {
    private readonly date: DateProto;

    constructor(date: DateProto) {
        this.date = date;
    }

    as_date(): Date {
        let date = new Date(this.date.year, this.date.month, this.date.day);
        return date;
    }
}
