
export class Custodian {
    private readonly name: string;
    private readonly pk: number;

    constructor(pk: number, name: string) {
        this.pk = pk;
        this.name = name;
    }

    toString() {
        return this.name;
    }

}
