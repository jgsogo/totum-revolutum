<script lang="ts">
  import { Button, Dropdown, DropdownDivider, DropdownItem, Search, ButtonGroup, DropdownGroup, Input } from 'flowbite-svelte';
  import { ChevronDownOutline, CloseCircleOutline, SearchOutline } from 'flowbite-svelte-icons';
  import { onMount } from 'svelte';
  import type { Component } from 'svelte';

  type T = $$Generic;

  let {
    value = $bindable(),
    items = $bindable(),

    labelForItem,
    searchMatches = (item: T, searchTerm: string) => {
      return labelForItem(item).toLowerCase().indexOf(searchTerm?.toLowerCase()) !== -1;
    },

    searchEl = undefined,
    sorted = true,
    // placeholder = 'Choose item...',
    // labelForItem,
    // allGroupsLabel = 'All',
    // groupBy = undefined,
    // labelInGroup = undefined,
    // equalItems = (lhs: T, rhs: T) => {
    //   return lhs === rhs;
    // },
  }: {
    value: T | undefined;
    items: T[];
    labelForItem: (item: T | undefined) => string;
    searchMatches?: (item: T, searchTerm: string) => boolean;

    searchEl?: string;
    sorted: boolean;
    // placeholder?: string;
    // allGroupsLabel?: string;
    // groupBy?: ((entry: T) => string) | undefined;
    // labelInGroup?: ((entry: T) => string) | undefined;
    // equalItems?: (lhs: T, rhs: T) => boolean;
  } = $props();

  // Search
  let searchTerm = $state<string>('');
  let filteredItems: T[] = $derived.by(() => {
    let all_items = items.filter((item: T) => searchMatches(item, searchTerm));
    if (sorted) {
      all_items.sort((lhs: T, rhs: T) => labelForItem(lhs).localeCompare(labelForItem(rhs)));
    }
    return all_items;
  });
  let value_label = $derived(labelForItem(value));

  let isOpen = $state(false);
  let containerEl: Component<{ offsetWidth: number }>;
  let dropdownWidth = $state('auto');

  onMount(() => {
    if (containerEl) {
      dropdownWidth = `${containerEl.offsetWidth}px`;
    }
  });

  // Optional: re-compute on open in case size changes
  $effect(() => {
    if (isOpen && containerEl) {
      dropdownWidth = `${containerEl.offsetWidth}px`;
    }
  });
</script>

<ButtonGroup bind:this={containerEl} class="w-full">
  <!-- Trigger Button -->
  <Button
    color={undefined}
    class="shrink-0 border border-gray-300 bg-gray-100 text-gray-900 hover:bg-gray-200 focus:ring-gray-300 dark:border-gray-700 dark:bg-gray-600 dark:text-white dark:hover:bg-gray-700 dark:focus:ring-gray-800"
  >
    {#if searchEl}
      <span>{searchEl}</span>
    {:else}
      <SearchOutline class="h-6 w-6" />
    {/if}
    <ChevronDownOutline class="ms-2 h-6 w-6" />
  </Button>

  <!-- Value preview -->
  <Input disabled bind:value={value_label} onclick={() => (isOpen = !isOpen)} />
</ButtonGroup>

<!-- Dropdown (absolutely positioned) -->
<Dropdown
  bind:isOpen
  simple
  placement="bottom-start"
  class="p-3 text-sm bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded shadow-lg"
  style={`width: ${dropdownWidth};`}
>
  <div class="p-3">
    <Search size="md" bind:value={searchTerm} />
  </div>
  <DropdownGroup class="h-48 overflow-y-auto">
    {#each filteredItems as item}
      <DropdownItem
        class="rounded-sm p-2 hover:bg-gray-100 dark:hover:bg-gray-600"
        onclick={() => {
          value = item;
          isOpen = false;
        }}>{labelForItem(item)}</DropdownItem
      >
    {/each}
  </DropdownGroup>
</Dropdown>
