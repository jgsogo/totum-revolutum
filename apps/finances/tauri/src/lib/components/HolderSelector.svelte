<script lang="ts">
  import { Avatar, Dropdown, DropdownDivider, DropdownHeader, DropdownItem, Spinner } from "flowbite-svelte";
  import type { Holder } from "$lib/models/Holder";

  let { holders, active_holder = $bindable() }: { holders: Holder[], active_holder: Holder } = $props();
  const personas = holders.filter((it) => !it.is_company);
  const companies = holders.filter((it) => it.is_company);
</script>

<button class="ms-3 rounded-full ring-gray-400 focus:ring-4 dark:ring-gray-600">
  <Avatar>{active_holder.name}</Avatar>
  <!-- <Avatar size="sm" src="https://flowbite-admin-dashboard.vercel.app/images/users/bonnie-green.png" tabindex={0} /> -->
</button>
<Dropdown placement="bottom-end">
  <DropdownHeader>
		<span class="block text-sm">{active_holder.name}</span>
	</DropdownHeader>
  {#each personas as persona}
    <DropdownItem href="/holder/{persona.pk}">{@html persona}</DropdownItem>
  {/each}
  <DropdownDivider />
  {#each companies as company}
    <DropdownItem href="/holder/{company.pk}">{@html company}</DropdownItem>
  {/each}
</Dropdown>
