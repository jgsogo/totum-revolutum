import { get_breadcrumbs_for_movementtype } from "$lib/commands";

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

    async getBreadcrumbs(): Promise<string[]> {
        if (this._breadcrumbs) {
            return this._breadcrumbs;
        }
        this._breadcrumbs = await get_breadcrumbs_for_movementtype(this);
        return this._breadcrumbs;
    }

}
