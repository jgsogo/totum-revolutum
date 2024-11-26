
export class Holder {
    readonly pk: number;
    readonly name: string;
    readonly is_company: boolean;

    constructor(pk: number, name: string, is_company: boolean) {
        this.pk = pk;
        this.name = name;
        this.is_company = is_company;
    }

    toString() {
        return this.name;
    }

}
