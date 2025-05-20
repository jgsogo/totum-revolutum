import { Date as DateProto, DateSchema } from "../protos/google/type/date_pb.js";
import { create } from "@bufbuild/protobuf";

/**
 * Wraps a `google::type::Date` protobuf message.
 *
 * Note this class represents a Date, with no time associated, so there is no
 * information associated to timezones.
 */
export class DateWrapper {
    private readonly date: DateProto;

    constructor(date: DateProto) {
        this.date = date;
    }

    static create_from_yyyy_mm_dd(year: number, month: number, day: number): DateWrapper {
        if (month < 1 || month > 12) {
            throw new Error("Month has to be in the [1, 12] range (1-January, 2-February).");
        }
        if (day < 1 || day > 31) {
            throw new Error(`Day ${day} not in the range [1-31]`);
        }
        let proto = create(DateSchema, { year, month, day }) as DateProto;
        return new DateWrapper(proto);
    }

    /**
     * Creates a new DateWrapper, using local time
     * @param {Date} date
     * @returns the DateWrapper object
     */
    static create_from_date(date: Date): DateWrapper {
        return DateWrapper.create_from_yyyy_mm_dd(date.getFullYear(), date.getMonth() + 1, date.getDate());
    }


    as_proto(): DateProto {
        return this.date;
    }

    toString(): string {
        const month_str = String(this.month()).padStart(2, '0');
        const day_str = String(this.day()).padStart(2, '0');
        return `${this.year()}-${month_str}-${day_str}`;
    }

    year(): number {
        return this.date.year;
    }

    month(): number {
        return this.date.month;
    }

    day(): number {
        return this.date.day;
    }

    less_than(other: DateWrapper): boolean {
        if (this.year() === other.year()) {
            if (this.month() === other.month()) {
                return this.day() < other.day();
            }
            else {
                return this.month() < other.month();
            }
        }
        else {
            return this.year() < other.year();
        }
    }

    equal(other: DateWrapper): boolean {
        return (this.year() === other.year()) && (this.month() === other.month()) && (this.day() === other.day())
    }

    lte(other: DateWrapper): boolean {
        return this.equal(other) || this.less_than(other)
    }
}

/**
 * A function to sort `DateWrapper` objects
 *
 * Following the `sort()` function specification, it is expected to return a negative value if
 * the first argument is less than the second argument, zero if they're equal, and a positive
 * value otherwise.
 *
 * @param {DateWrapper} lhs left-hand-side operator
 * @param {DateWrapper} rhs right-hand-side operator
 */
export function sort_date_wrapper(lhs: DateWrapper, rhs: DateWrapper) {
    if (lhs.year() === rhs.year()) {
        if (lhs.month() === rhs.month()) {
            return lhs.day() - rhs.day();
        }
        else {
            return lhs.month() - rhs.month();
        }
    }
    else {
        return lhs.year() - rhs.year();
    }
}
