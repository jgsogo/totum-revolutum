
export class SidebarEntry {
    readonly label: string;
    readonly icon: string;
    readonly href?: string;
    readonly children: { label: string, href: string}[] = [];

    constructor(label: string, icon: string, href?: string) {
        this.label = label;
        this.icon = icon;
        this.href = href;
    }

    addChildren(label: string, href: string) {
        this.children.push({label, href})
    }
}
