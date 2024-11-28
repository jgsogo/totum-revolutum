export class Snapshot {
    readonly amount: number;
    private readonly ccy: string;
    private readonly quantity?: number;
    private readonly unit_value?: number;
    readonly date_value: string;

    constructor(ccy: string, amount: number, date_value: string, quantity?: number, unit_value?: number) {
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
