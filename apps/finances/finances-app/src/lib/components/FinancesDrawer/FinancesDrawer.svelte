<script lang="ts">
	import { getDrawerStore, Drawer } from '@skeletonlabs/skeleton';
	import FinancesSidebar from '../FinancesSidebar/FinancesSidebar.svelte';

	const drawerStore = getDrawerStore();

	$: classesDrawer = $drawerStore.id === 'sidenav' ? 'lg:hidden' : '';
</script>


<Drawer class={classesDrawer}>
	{#if $drawerStore.id === 'sidenav'}
		<!-- Doc Sidebar -->
		<FinancesSidebar embedded={true} />
	{:else}
		<!-- Fallback Error -->
		<div class="w-full h-full flex justify-center items-center">
			<div class="text-center space-y-2">
				<p>Invalid <code class="code">$drawerStore.id</code> provided.</p>
			</div>
		</div>
	{/if}
</Drawer>
