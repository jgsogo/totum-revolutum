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
		<!-- Explore -->
		<!-- <div class="relative hidden lg:block">
			<button class="btn hover:variant-soft-primary">
				<span>Explore</span>
			</button>
        </div> -->

        <!-- Settings -->
		<div>
			<!-- trigger -->
			<button class="btn hover:variant-soft-primary" use:popup={{ event: 'click', target: 'settings' }}>
				<i class="fa-solid fa-palette text-lg md:!hidden"></i>
				<span class="hidden md:inline-block">Settings</span>
				<i class="fa-solid fa-caret-down opacity-50"></i>
			</button>
			<!-- popup -->
			<div class="card p-4 w-60 shadow-xl" data-popup="settings">
                <div class="space-y-4">
					<section class="flex justify-between items-center">
						<h6 class="h6">Mode</h6>
						<LightSwitch />
					</section>
					<hr />
                    <nav class="list-nav p-4 -m-4 max-h-64 lg:max-h-[500px] overflow-y-auto">
                        <ul>
                            <li><a href="/settings/database">Database</a></li>
                        </ul>
                    </nav>
                </div>
            </div>
        </div>

        <!-- Menu -->
		<section class="hidden sm:inline-flex space-x-1">
			<a class="btn hover:variant-soft-primary" href="/settings">
				<span>Settings--</span>
			</a>
			<a class="btn hover:variant-soft-primary" href="/about">
				<span>About</span>
			</a>
		</section>

        <!-- Search -->
		<div class="md:inline md:ml-4">
			<button class="btn space-x-4 variant-soft hover:variant-soft-primary" on:click={triggerSearch}>
				<i class="fa-solid fa-magnifying-glass text-sm"></i>
				<small class="hidden md:inline-block">{isOsMac ? '⌘' : 'Ctrl'}+K</small>
			</button>
		</div>
    </svelte:fragment>
</AppBar>
