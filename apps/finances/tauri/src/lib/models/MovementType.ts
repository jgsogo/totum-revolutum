import { get_breadcrumbs_for_movementtype } from "$lib/commands";

export class MovementType {
    readonly pk: number;
    readonly name: string;
    private breadcrumb?: string[];

    constructor(pk: number, name: string, breadcrumb?: string[]) {
        this.pk = pk;
        this.name = name;
        this.breadcrumb = breadcrumb;
    }

    toString() {
        return this.name;
    }

    getBreadcrumbs(): string[] | undefined {
        return this.breadcrumb;
    }

    async breadcrumbs(): Promise<string[]> {
        if (this.breadcrumb) {
            return this.breadcrumb;
        }
        this.breadcrumb = await get_breadcrumbs_for_movementtype(this);
        return this.breadcrumb;
    }

}
