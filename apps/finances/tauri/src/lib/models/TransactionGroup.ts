
export class TransactionGroup {
    readonly pk: number;
    readonly name: string;
    readonly description?: string;

    constructor(pk: number, name: string, description?: string) {
        this.pk = pk;
        this.name = name;
        this.description = description;
    }

    toString() {
        return this.name;
    }

}
