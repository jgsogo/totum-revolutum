export class Breadcrumb {
    private readonly breadcrumb: string[];

    constructor(breadcrumb: string[]) {
        this.breadcrumb = breadcrumb;
    }

    toString(): string {
        return this.breadcrumb.join(' / ');
    }
}
