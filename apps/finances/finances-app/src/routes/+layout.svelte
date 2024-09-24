<script lang="ts">
	import '../app.postcss';
	import { AppShell, initializeStores, Toast, Drawer, getDrawerStore, AppBar } from '@skeletonlabs/skeleton';

    // Finances Components
    import FinancesAppBar from '$lib/components/FinancesAppBar/FinancesAppBar.svelte';
    import FinancesSidebar from '$lib/components/FinancesSidebar/FinancesSidebar.svelte';
    import FinancesDrawer from '$lib/components/FinancesDrawer/FinancesDrawer.svelte';

    // Dependency: Floating UI
	import { storePopup } from '@skeletonlabs/skeleton';
	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

    // Global Stylesheets
	import '../app.postcss';
	// Font Awesome
	import '@fortawesome/fontawesome-free/css/fontawesome.css';
	import '@fortawesome/fontawesome-free/css/brands.css';
	import '@fortawesome/fontawesome-free/css/solid.css';

    initializeStores();

    const drawerStore = getDrawerStore();

    function drawerOpen(): void {
        drawerStore.open({});
    }
</script>

<Toast />
<FinancesDrawer />


<!-- App Shell -->
<AppShell slotSidebarLeft="bg-surface-50-900-token lg:w-auto">
    <!-- Header -->
	<svelte:fragment slot="header">
        <FinancesAppBar />
	</svelte:fragment>

	<!-- Sidebar (Left) -->
	<svelte:fragment slot="sidebarLeft">
		<FinancesSidebar div_class="hidden lg:grid w-[360px] overflow-hidden" />
	</svelte:fragment>

	<!-- Page Content -->
	<slot />
</AppShell>
