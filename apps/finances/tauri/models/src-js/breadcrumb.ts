export class Breadcrumb {
    private readonly breadcrumb: string[];

    constructor(breadcrumb: string[]) {
        this.breadcrumb = breadcrumb;
    }

    toString(): string {
        return this.breadcrumb.join(' / ');
    }

    length(): number {
        return this.breadcrumb.length
    }

    slice(start?: number, end?: number): Breadcrumb {
        return new Breadcrumb( this.breadcrumb.slice(start, end))
    }
}
