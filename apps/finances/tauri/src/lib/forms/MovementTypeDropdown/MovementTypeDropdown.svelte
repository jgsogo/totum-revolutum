<script lang="ts">
  import type { Account } from "$lib/models/Account";
  import { Button, Select, DropdownDivider } from "flowbite-svelte";
  import { CloseCircleOutline } from "flowbite-svelte-icons";
  import { Dropdown, DropdownItem } from "flowbite-svelte";
  import { ChevronDownOutline } from "flowbite-svelte-icons";
  import type { MovementType } from "$lib/models/MovementType";

  let {
    movementtype = $bindable(),
    all_movementtypes,
  }: {
    movementtype: MovementType | undefined;
    all_movementtypes: MovementType[]; // TODO: Document that all movement types should have breadcrumbs already resolved!
  } = $props();

  const breadcrumbs_group_size = 2;

  function group_movementtype_by_breadcrumb(movtype: MovementType) {
    let breadcrumbs = movtype.breadcrumbs()!;
    if (breadcrumbs.length > breadcrumbs_group_size) {
      return breadcrumbs.slice(0, breadcrumbs_group_size).join(" / ");
    } else {
      return "/";
    }
  }

  // Index all movement types by their first two levels
  // svelte-ignore non_reactive_update
  let all_select_categories: string[] = all_movementtypes.map(group_movementtype_by_breadcrumb);
  all_select_categories = all_select_categories
    .filter((value: string, index: number) => all_select_categories.indexOf(value) === index)
    .sort((one, two) => (one < two ? -1 : 1));

  // Select movement type category
  let selectCategory = $state("All");
  let dropdownOpen = $state(false);
  const handleClickCustodian = (e: MouseEvent) => {
    e.preventDefault();
    selectCategory = e.target?.innerText;
    dropdownOpen = false;
  };

  // All movementtypes sorted by above category
  let all_movementtypes_items = $derived.by(() => {
    if (selectCategory === "All") {
      // Collect all accounts with their custodian
      return all_movementtypes
        .map((value: MovementType) => {
          return { value: value, name: `${value.breadcrumbs()!.join(" / ")}` };
        })
        .sort((lhs, rhs) => (lhs.name < rhs.name ? -1 : lhs.name > rhs.name ? 1 : 0));
    } else {
      // Collect without custodian
      return all_movementtypes
        .filter((value: MovementType) => group_movementtype_by_breadcrumb(value) === selectCategory)
        .map((value) => {
          let breadcrumbs = value.breadcrumbs()!;
          if (breadcrumbs.length > breadcrumbs_group_size) {
            return { value: value, name: `${breadcrumbs.slice(breadcrumbs_group_size).join(" / ")}` };
          } else {
            return { value: value, name: `${breadcrumbs.join(" / ")}` };
          }
        })
        .sort((lhs, rhs) => (lhs.name < rhs.name ? -1 : lhs.name > rhs.name ? 1 : 0));
    }
  });
</script>

<div class="flex relative">
  <Button size="sm" class="rounded-e-none whitespace-nowrap border border-e-0 border-primary-700">
    {selectCategory}
    <ChevronDownOutline class="w-2.5 h-2.5 ms-2.5" />
  </Button>
  <Dropdown bind:open={dropdownOpen} classContainer="w-120">
    {#each all_select_categories as label}
      <DropdownItem onclick={handleClickCustodian}>
        {label}
      </DropdownItem>
    {/each}
    <DropdownDivider />
    <DropdownItem
      class="flex items-center p-3 -mb-1 text-sm font-medium text-red-600 bg-gray-50 hover:bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:text-red-500 hover:underline"
      onclick={handleClickCustodian}><CloseCircleOutline class="w-5 h-5 me-1" />All</DropdownItem
    >
  </Dropdown>

  <Select required size="sm" class="rounded-none py-2.5" items={all_movementtypes_items} bind:value={movementtype} placeholder="Choose movement type ..."/>
</div>
