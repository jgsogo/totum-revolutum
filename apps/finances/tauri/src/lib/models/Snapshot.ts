export class Snapshot {
    readonly pk: number;
    readonly amount: number;
    private readonly ccy: string;
    readonly quantity?: number;
    readonly unit_value?: number;
    readonly date_value: Date;

    constructor(pk: number, ccy: string, amount: number, date_value: Date, quantity?: number, unit_value?: number) {
        this.pk = pk;
        this.amount = amount;
        this.ccy = ccy;
        this.date_value = date_value;
        this.quantity = quantity;
        this.unit_value = unit_value
    }

    toString() {
        return `${this.amount} ${this.ccy}`;
    }

}
