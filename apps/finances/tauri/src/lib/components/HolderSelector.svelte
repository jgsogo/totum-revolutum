<script lang="ts">
  import { Dropdown, DropdownItem, Search } from "flowbite-svelte";
  import type { AppState, Holder } from "../../../models/src-js/index";
  import Avatar from "./Avatar.svelte";

  let { all_holders, active_holder, app_state }: { all_holders: Holder[]; active_holder: Holder; app_state: AppState } =
    $props();

  let dropdownOpen = $state(false);
  let searchTerm = $state("");
  const people = all_holders.map((h) => {
    return { name: h.name(), holder: h };
  });
  let filteredItems = $derived(
    people.filter((person) => person.name.toLowerCase().indexOf(searchTerm?.toLowerCase()) !== -1)
  );
</script>

<button class="ms-3 rounded-full ring-gray-400 focus:ring-4 dark:ring-gray-600">
  <Avatar photo_url={app_state.base_media_url() + active_holder.photo()} name={active_holder.name()} />
</button>
<Dropdown bind:open={dropdownOpen} class="overflow-y-auto h-96" placement="bottom-end">
  <div slot="header" class="p-3">
    <Search size="md" bind:value={searchTerm} />
  </div>
  {#each filteredItems as holder (holder)}
    <DropdownItem
      href="/holder/{holder.holder.pk()}"
      on:click={() => {
        dropdownOpen = false;
        searchTerm = "";
      }}
      class=" hover:bg-gray-100 dark:hover:bg-gray-600">{@html holder.name}</DropdownItem
    >
  {/each}
</Dropdown>
