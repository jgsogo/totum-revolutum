<script lang="ts">
	import { Avatar, Dropdown, DropdownDivider, DropdownHeader, DropdownItem } from 'flowbite-svelte';

    import { holders } from '$lib/commands';
</script>

<button class="ms-3 rounded-full ring-gray-400 focus:ring-4 dark:ring-gray-600">
	<Avatar size="sm" src="https://flowbite-admin-dashboard.vercel.app/images/users/bonnie-green.png" tabindex={0} />
</button>
<Dropdown placement="bottom-end">
	<!-- <DropdownHeader>
		<span class="block text-sm">{name}</span>
		<span class="block truncate text-sm font-medium">{email}</span>
	</DropdownHeader> -->
    <DropdownItem>it0</DropdownItem>
    <DropdownItem>it1</DropdownItem>
    {#await holders()}
        <DropdownItem><span class="block text-sm">Loading holders</span></DropdownItem>
    {:then all_holders}
        <DropdownItem>sadfasfa</DropdownItem>
        {#if all_holders.length != 0 }
            {#each all_holders as holder}
                <DropdownItem>{@html holder}</DropdownItem>
            {/each}
        {:else}
            <DropdownItem><span class="block text-sm">No holders found</span></DropdownItem>
        {/if}
    {:catch error}
        <p style="color: red">{error.message}</p>
    {/await}

	<!-- <DropdownItem>Dashboard</DropdownItem>
	<DropdownItem>Settings</DropdownItem>
	<DropdownItem>Earnings</DropdownItem>
	<DropdownDivider />
	<DropdownItem>Sign out</DropdownItem> -->
</Dropdown>
