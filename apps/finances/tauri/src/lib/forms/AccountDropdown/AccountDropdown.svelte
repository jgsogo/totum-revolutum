<script lang="ts">
  import { Button, Select, DropdownDivider } from "flowbite-svelte";
  import { CloseCircleOutline } from "flowbite-svelte-icons";
  import { Dropdown, DropdownItem } from "flowbite-svelte";
  import { ChevronDownOutline } from "flowbite-svelte-icons";
  import { Account } from "../../../../models/src-js";

  let {
    account = $bindable(),
    all_accounts,
  }: {
    account: Account | undefined;
    all_accounts: Account[];
  } = $props();

  // All custodian names
  // svelte-ignore non_reactive_update
  let all_custodians_names = all_accounts.map((value: Account) => value.custodian().name);
  all_custodians_names = all_custodians_names
    .filter((value: string, index: number) => all_custodians_names.indexOf(value) === index)
    .sort((one, two) => (one > two ? -1 : 1));

  // Select custodian
  let selectCategory = $state("All");
  let dropdownOpen = $state(false);
  const handleClickCustodian = (e: MouseEvent) => {
    e.preventDefault();
    selectCategory = e.target?.innerText;
    dropdownOpen = false;
  };

  // All accounts sorted by Custodian
  let all_accounts_items = $derived.by(() => {
    if (selectCategory === "All") {
      // Collect all accounts with their custodian
      return all_accounts
        .map((value) => {
          return { value: value, name: `${value.custodian()} | ${value.name()}` };
        })
        .sort((lhs, rhs) => (lhs.name < rhs.name ? -1 : lhs.name > rhs.name ? 1 : 0));
    } else {
      // Collect without custodian
      return all_accounts
        .filter((acc: Account) => acc.custodian().name === selectCategory)
        .map((value) => {
          return { value: value, name: value.name() };
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
  <Dropdown bind:open={dropdownOpen} classContainer="w-40">
    {#each all_custodians_names as custodian_name}
      <DropdownItem onclick={handleClickCustodian}>
        {custodian_name}
      </DropdownItem>
    {/each}
    <DropdownDivider />
    <DropdownItem
      class="flex items-center p-3 -mb-1 text-sm font-medium text-red-600 bg-gray-50 hover:bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:text-red-500 hover:underline"
      onclick={handleClickCustodian}
    >
      <CloseCircleOutline class="w-5 h-5 me-1" />All
    </DropdownItem>
  </Dropdown>

  <Select
    required
    size="sm"
    class="rounded-none py-2.5"
    items={all_accounts_items}
    bind:value={account}
    placeholder="Choose account ..."
    on:change
  />
</div>
