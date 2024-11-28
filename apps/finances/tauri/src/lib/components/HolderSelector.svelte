<script lang="ts">
  import { Avatar, Dropdown, DropdownItem, Search } from "flowbite-svelte";
  import type { Holder } from "$lib/models/Holder";

  let {
    holders,
    active_holder = $bindable(),
    base_media_url,
  }: { holders: Holder[]; active_holder: Holder; base_media_url: string } = $props();

  const initials = (holder: Holder): string => {
    let words = holder.name.split(/\s/);
    if (words.length == 1) {
      return holder.name.substring(0, 3);
    } else {
      let acronym = words
        .map((word) => word.replace("(", ""))
        .reduce((response, word) => (response += word.slice(0, 1)), "");
      return acronym.substring(0, 3);
    }
  };

  let dropdownOpen = $state(false);
  let searchTerm = $state("");
  const people = holders.map((h) => {
    return { name: h.name, holder: h };
  });
  let filteredItems = $derived(
    people.filter((person) => person.name.toLowerCase().indexOf(searchTerm?.toLowerCase()) !== -1)
  );
</script>

<button class="ms-3 rounded-full ring-gray-400 focus:ring-4 dark:ring-gray-600">
  {#if active_holder.photo}
    <Avatar title={active_holder.name} src="{base_media_url}{active_holder.photo}">{active_holder.name}</Avatar>
  {:else}
    <Avatar title={active_holder.name}>{initials(active_holder).toUpperCase()}</Avatar>
  {/if}
</button>
<Dropdown bind:open={dropdownOpen} class="overflow-y-auto h-96" placement="bottom-end">
  <div slot="header" class="p-3">
    <Search size="md" bind:value={searchTerm} />
  </div>
  {#each filteredItems as holder (holder)}
    <DropdownItem
      href="/holder/{holder.holder.pk}"
      on:click={() => {
        dropdownOpen = false;
        searchTerm = "";
      }}
      class=" hover:bg-gray-100 dark:hover:bg-gray-600">{@html holder.name}</DropdownItem
    >
  {/each}
</Dropdown>
