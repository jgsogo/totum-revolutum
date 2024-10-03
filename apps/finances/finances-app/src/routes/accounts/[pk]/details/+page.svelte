<script lang="ts">
	import { error } from '@sveltejs/kit';
	import type { PageData } from './$types';
	import AccountDetail from '$lib/components/Account/AccountDetail.svelte';

	export let data: PageData;
</script>

{#await data}
	Loading account data
{:then account}
	<AccountDetail bind:account={account.account}/>
{/await}
