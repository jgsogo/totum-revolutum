<script lang="ts">
  import { Avatar, Dropdown, DropdownDivider, DropdownHeader, DropdownItem, Spinner } from "flowbite-svelte";
  import type { Holder } from "$lib/models/Holder";

  let { holders, active_holder = $bindable(), base_media_url }: { holders: Holder[], active_holder: Holder, base_media_url: string } = $props();
  const personas = holders.filter((it) => !it.is_company);
  const companies = holders.filter((it) => it.is_company);

  const initials = (holder: Holder): string => {
    let words = holder.name.split(/\s/);
    if (words.length == 1) {
      return holder.name.substring(0,3);
    }
    else {
      let acronym = words.map((word) => word.replace('(', '')).reduce((response,word)=> response+=word.slice(0,1),'');
      return acronym.substring(0,3);
    }
  }

</script>

<button class="ms-3 rounded-full ring-gray-400 focus:ring-4 dark:ring-gray-600">
  {#if active_holder.photo}
    <Avatar src="{base_media_url}{active_holder.photo}">{active_holder.name}</Avatar>
  {:else}
    <Avatar>{initials(active_holder).toUpperCase()}</Avatar>
  {/if}
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
