
export class Custodian {
    readonly name: string;
    readonly pk: number;
    readonly photo?: string;

    constructor(pk: number, name: string, photo?: string) {
        this.pk = pk;
        this.name = name;
        this.photo = photo;
    }

    toString() {
        return this.name;
    }

}
