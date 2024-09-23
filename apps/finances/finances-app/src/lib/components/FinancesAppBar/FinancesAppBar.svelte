<script lang="ts">
    import { browser } from '$app/environment';

    // Types
	import type { ModalSettings, DrawerSettings, ToastSettings } from '@skeletonlabs/skeleton';

    // Docs
	import FinancesLogoFull from '$lib/components/FinancesLogos/FinancesLogoFull.svelte';

    // Components & Utilities
	import { AppBar, LightSwitch, popup, getModalStore } from '@skeletonlabs/skeleton';

	// Stores
	import { getDrawerStore } from '@skeletonlabs/skeleton';
    import { getToastStore } from '@skeletonlabs/skeleton';
	const drawerStore = getDrawerStore();
    const toastStore = getToastStore();

    // Local
	let isOsMac = false;
	const modalStore = getModalStore();

	// Set Search Keyboard Shortcut
	if (browser) {
		let os = navigator.userAgent;
		isOsMac = os.search('Mac') !== -1;
	}

	// Drawer Handler
	function drawerOpen(): void {
		const s: DrawerSettings = { id: 'sidenav' };
		drawerStore.open(s);
	}

	// Search
	function triggerSearch(): void {
        // TODO: https://github.com/skeletonlabs/skeleton/blob/dev/sites/skeleton.dev/src/lib/components/DocsAppBar/DocsAppBar.svelte
        const t: ToastSettings = {
            message: 'The method triggerSearch is not implemented (see comment in sources).',
            timeout: 5000
        };
        toastStore.trigger(t);
	}
</script>

<AppBar shadow="shadow-2xl" slotTrail="!space-x-2">

    <svelte:fragment slot="lead">
		<div class="flex items-center space-x-4">
			<!-- Hamburger Menu -->
			<button on:click={drawerOpen} class="btn-icon btn-icon-sm lg:!hidden">
				<i class="fa-solid fa-bars text-xl"></i>
			</button>
			<!-- Logo -->
			<a class="lg:!ml-0 w-[32px] lg:w-auto overflow-hidden" href="/" title="Go to Homepage">
				<FinancesLogoFull />
			</a>
		</div>
	</svelte:fragment>

    <svelte:fragment slot="trail">
		<!-- Search -->
		<div class="md:inline md:ml-4">
			<button class="btn space-x-4 variant-soft hover:variant-soft-primary" on:click={triggerSearch}>
				<i class="fa-solid fa-magnifying-glass text-sm"></i>
				<small class="hidden md:inline-block">{isOsMac ? '⌘' : 'Ctrl'}+K</small>
			</button>
		</div>

		<!-- Settings + About -->
		<section class="hidden sm:inline-flex space-x-1">
			<a class="btn-icon hover:variant-soft-primary" href="/settings">
				<i class="fa-solid fa-gear text-lg"></i>
			</a>
			<div class="btn hover:variant-soft-primary">
                <LightSwitch />
            </div>
		</section>
    </svelte:fragment>
</AppBar>
