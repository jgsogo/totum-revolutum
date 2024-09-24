<script lang="ts">
    import { page } from '$app/stores';

    import { menuNavLinks } from '$lib/links';
    import { AppRail, AppRailAnchor, AppRailTile, getDrawerStore } from '@skeletonlabs/skeleton';

    // Local
	let currentRailCategory: keyof typeof menuNavLinks | undefined = $state(undefined);
	const drawerStore = getDrawerStore();

	function onClickAnchor(): void {
		currentRailCategory = undefined;
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

    // let stateDatatable = $state();

    // const functionReadData = async function () {
    // }

    // Reactive
    const submenu = $derived(menuNavLinks[currentRailCategory ?? '/all'])

    function listboxItemActive(href: string): string {
        return $page.url.pathname?.includes(href) ? 'bg-primary-active-token' : ''
    }

    let {div_class}: {div_class: string} = $props();
	// $: submenu = menuNavLinks[currentRailCategory ?? '/all'];
	// $: listboxItemActive = (href: string) => ($page.url.pathname?.includes(href) ? 'bg-primary-active-token' : '');
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
        {#each submenu as segment, i}
            <!-- Title -->
            <p class="font-bold pl-4 text-2xl">{segment.title}</p>
            <!-- Nav List -->
            <nav class="list-nav">
                <ul>
                    {#each segment.list as { href, label, badge }}
                        <li>
                            <a {href} class={listboxItemActive(href)} data-sveltekit-preload-data="hover" on:keypress on:click={drawerStore.close}>
                                <span class="flex-auto">{@html label}</span>
                                {#if badge}<span class="badge variant-filled-secondary">{badge}</span>{/if}
                            </a>
                        </li>
                    {/each}
                </ul>
            </nav>
            <!-- Divider -->
            {#if i + 1 < submenu.length}<hr class="!my-6 opacity-50" />{/if}
        {/each}
    </section>
</div>
