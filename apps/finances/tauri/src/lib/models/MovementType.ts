
export class MovementType {
    readonly pk: number;
    readonly name: string;
    private _breadcrumbs?: string[];

    constructor(pk: number, name: string) {
        this.pk = pk;
        this.name = name;
    }

    toString() {
        return this.name;
    }

    breadcrumbs(): string[] | undefined {
        return this._breadcrumbs;
    }


}
