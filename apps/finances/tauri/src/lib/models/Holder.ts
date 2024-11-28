
export class Holder {
    readonly pk: number;
    readonly name: string;
    readonly is_company: boolean;
    readonly photo?: string;

    constructor(pk: number, name: string, is_company: boolean, photo?: string) {
        this.pk = pk;
        this.name = name;
        this.is_company = is_company;
        this.photo = photo;
    }

    toString() {
        return this.name;
    }

}
