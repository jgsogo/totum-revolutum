
export class MovementType {
    readonly pk: number;
    readonly name: string;
    readonly breadcrumb?: string;

    constructor(pk: number, name: string, breadcrumb?: string) {
        this.pk = pk;
        this.name = name;
        this.breadcrumb = breadcrumb;
    }

    toString() {
        return this.name;
    }

}
