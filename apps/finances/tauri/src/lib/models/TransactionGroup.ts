
export class TransactionGroup {
    readonly pk: number;
    readonly name: string;

    constructor(pk: number, name: string) {
        this.pk = pk;
        this.name = name;
    }

    toString() {
        return this.name;
    }

}
