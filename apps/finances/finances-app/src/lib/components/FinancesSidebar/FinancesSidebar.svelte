<script lang="ts">
    import { page } from '$app/stores';

    import { AppRail, AppRailAnchor, AppRailTile, getDrawerStore } from '@skeletonlabs/skeleton';
    import { invoke } from "@tauri-apps/api/core";
    import { Accordion, AccordionItem } from '@skeletonlabs/skeleton';


    import { debug } from '@tauri-apps/plugin-log';

    // Local
	let currentRailCategory: string = $state('/all');
	const drawerStore = getDrawerStore();

	function onClickAnchor(): void {
		currentRailCategory = '/all';
		drawerStore.close();
	}

    // Lifecycle
	page.subscribe((page) => {
		// ex: /basePath/...
		let basePath: string = page.url.pathname.split('/')[1];
		if (!basePath) return;
		// Translate base path to link section
		if (['accounts'].includes(basePath)) currentRailCategory = '/accounts';
		if (['investments'].includes(basePath)) currentRailCategory = '/investments';
		if (['rentals'].includes(basePath)) currentRailCategory = '/rentals';
		if (['taxes'].includes(basePath)) currentRailCategory = '/taxes';
        if (['all'].includes(basePath)) currentRailCategory = '/all';
	});

    // Reactive
    type SidebarMenuItem = {
        name: string;
        href: string;
    };
    type SidebarMenu = {
        group: string;
        entries: SidebarMenuItem[];
    };

    const getSubmenu = async function (rail_category: string): Promise<SidebarMenu[]> {
        debug('Invoke menu command to retrieve SidebarMenu');
        return await invoke("sidebar_menu", {category: currentRailCategory});
    };

    const submenu = $derived(getSubmenu(currentRailCategory))

    function listboxItemActive(href: string): string {
        return $page.url.pathname?.includes(href) ? 'bg-primary-active-token' : ''
    }

    let {div_class}: {div_class: string} = $props();

</script>

<div class="grid grid-cols-[auto_1fr] h-full bg-surface-50-900-token border-r border-surface-500/30 {div_class}">
    <AppRail background="bg-transparent" border="border-r border-surface-500/30">

		<!-- Mobile Only -->
		<!-- prettier-ignore -->
		<AppRailAnchor href="/" class="lg:hidden" on:click={() => { onClickAnchor() }}>
			<svelte:fragment slot="lead"><i class="fa-solid fa-home text-2xl"></i></svelte:fragment>
			<span>Home</span>
		</AppRailAnchor>
		<!-- prettier-ignore -->
		<AppRailAnchor href="/settings" class="lg:hidden" on:click={() => { onClickAnchor() }}>
			<svelte:fragment slot="lead"><i class="fa-solid fa-gear text-2xl"></i></svelte:fragment>
			<span>Settings</span>
		</AppRailAnchor>
		<!-- --- / --- -->

        <AppRailTile bind:group={currentRailCategory} name="all" value={'/all'}>
			<svelte:fragment slot="lead"><i class="fa-solid fa-globe text-2xl"></i></svelte:fragment>
			<span>All</span>
		</AppRailTile>
        <hr class="opacity-30" />
        <AppRailTile bind:group={currentRailCategory} name="accounts" value={'/accounts'}>
			<svelte:fragment slot="lead"><i class="fa-solid fa-sack-dollar text-2xl"></i></svelte:fragment>
			<span>Accounts</span>
		</AppRailTile>
		<AppRailTile bind:group={currentRailCategory} name="investments" value={'/investments'}>
			<svelte:fragment slot="lead"><i class="fa-solid fa-money-bill-trend-up text-2xl"></i></svelte:fragment>
			<span>Investments</span>
		</AppRailTile>
		<AppRailTile bind:group={currentRailCategory} name="retirement" value={'/retirement'}>
			<svelte:fragment slot="lead"><i class="fa-solid fa-person-shelter text-2xl"></i></svelte:fragment>
			<span>Retirement</span>
		</AppRailTile>
		<AppRailTile bind:group={currentRailCategory} name="rentals" value={'/rentals'}>
			<svelte:fragment slot="lead"><i class="fa-solid fa-building text-2xl"></i></svelte:fragment>
			<span>Rentals</span>
		</AppRailTile>
        <hr class="opacity-30" />
        <AppRailTile bind:group={currentRailCategory} name="taxes" value={'/taxes'}>
            <svelte:fragment slot="lead"><i class="fa-solid fa-percent text-2xl"></i></svelte:fragment>
            <span>Taxes</span>
        </AppRailTile>
    </AppRail>

    <!-- Nav Links -->
    <section class="p-4 pb-20 space-y-4 overflow-y-auto">


        {#await submenu}
            <p>...loading accounts</p>
        {:then sidebar_menu_items}

            <Accordion>
                {#each sidebar_menu_items as sidebar_menu, i}
                    <AccordionItem>
                        <svelte:fragment slot="lead">
                            <i class="fa-solid fa-bank text-xl w-6 text-center"></i>
                        </svelte:fragment>
                        <svelte:fragment slot="summary"><p class="font-bold">{sidebar_menu.group}</p></svelte:fragment>
                        <svelte:fragment slot="content">
                            <!-- Nav List -->
                            <nav class="list-nav">
                                <ul>
                                    {#each sidebar_menu.entries as sidebar_menu_entry}
                                        <li>
                                            <a href="{sidebar_menu_entry.href}" class={listboxItemActive(sidebar_menu_entry.href)} data-sveltekit-preload-data="hover" on:keypress on:click={drawerStore.close}>
                                                <span class="flex-auto">{@html sidebar_menu_entry.name}</span>
                                            </a>
                                        </li>
                                    {/each}
                                </ul>
                            </nav>
                        </svelte:fragment>
                    </AccordionItem>
                {/each}
            </Accordion>

            <!-- TODO: Add search in the accordeon, add collapse all -->
        {/await}
    </section>
</div>
