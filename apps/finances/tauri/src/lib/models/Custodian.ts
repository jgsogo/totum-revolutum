
export class Custodian {
    readonly name: string;
    readonly pk: number;

    constructor(pk: number, name: string) {
        this.pk = pk;
        this.name = name;
    }

    toString() {
        return this.name;
    }

}
