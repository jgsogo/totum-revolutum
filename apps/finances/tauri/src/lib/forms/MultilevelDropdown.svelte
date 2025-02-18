<script lang="ts">
  import { Button, Dropdown, DropdownDivider, DropdownItem, Select } from "flowbite-svelte";
  import { ChevronDownOutline, CloseCircleOutline } from "flowbite-svelte-icons";

  type T = $$Generic;

  let {
    value = $bindable(),
    items,
    placeholder = "Choose item...",
    labelForItem,
    allGroupsLabel = "All",
    groupBy = undefined,
    labelInGroup = undefined,
    equalItems = (lhs: T, rhs: T) => {
      return lhs === rhs;
    },
  }: {
    value: T | undefined;
    items: T[];
    placeholder?: string;
    labelForItem: (entry: T) => string;
    allGroupsLabel?: string;
    groupBy?: ((entry: T) => string) | undefined;
    labelInGroup?: ((entry: T) => string) | undefined;
    equalItems?: (lhs: T, rhs: T) => boolean;
  } = $props();
  let fLabelInGroup = labelInGroup ?? labelForItem;

  // Ensure the initial value belongs to items
  value = value
    ? items.find((v: T) => {
        return equalItems(v, value!);
      })
    : undefined;

  // Get the items for the Select
  const all_items = items
    .map((it: T) => {
      return { value: it, name: labelForItem(it), group: groupBy ? groupBy(it) : null };
    })
    .sort((lhs, rhs) => lhs.name.localeCompare(rhs.name));
  let listed_items = $state(all_items);

  // Get the groups
  function get_groups(): string[] {
    let groups = all_items.map((it) => {
      return it.group!;
    });
    return groups
      .filter((value: string, index: number) => groups.indexOf(value) === index)
      .sort((lhs, rhs) => (lhs < rhs ? -1 : 1));
  }
  const groups_items = groupBy ? get_groups() : null;
  let groups_selected = $state(allGroupsLabel);
  let groups_opened = $state(false);

  // When the group is changed, the list of objects changes
  const handleClickGroup = (e: MouseEvent) => {
    e.preventDefault();
    groups_selected = e.target?.innerText;
    groups_opened = false;

    listed_items =
      groups_selected === allGroupsLabel
        ? all_items
        : all_items
            .filter((value) => value.group == groups_selected)
            .map((value) => {
              return { value: value.value, group: value.group, name: fLabelInGroup(value.value) };
            });
  };
</script>

<div class="flex relative">
  {#if groups_items}
    <Button size="sm" class="rounded-e-none whitespace-nowrap border border-e-0 border-primary-700">
      {groups_selected}
      <ChevronDownOutline class="w-2.5 h-2.5 ms-2.5" />
    </Button>
    <Dropdown bind:open={groups_opened} classContainer="w-40">
      {#each groups_items as group}
        <DropdownItem onclick={handleClickGroup}>
          {group}
        </DropdownItem>
      {/each}
      <DropdownDivider />
      <DropdownItem
        class="flex items-center p-3 -mb-1 text-sm font-medium text-red-600 bg-gray-50 hover:bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:text-red-500 hover:underline"
        onclick={handleClickGroup}
      >
        <CloseCircleOutline class="w-5 h-5 me-1" />{allGroupsLabel}
      </DropdownItem>
    </Dropdown>
  {/if}

  <Select
    required
    size="sm"
    class="rounded-none py-2.5"
    items={listed_items}
    bind:value
    {placeholder}
    on:apply
    on:clear
    on:select
    on:change
  />
</div>
