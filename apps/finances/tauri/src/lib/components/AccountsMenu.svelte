<script lang="ts">
  import { DateWrapper } from "../../../../../../libraries/googleapis/src-js";
  import { Sidebar, SidebarWrapper, SidebarItem, SidebarGroup, SidebarDropdownWrapper } from "flowbite-svelte";
  import { AccountCategory, type Account, type AppState, type Holder } from "../../../models/src-js";
  import {
    AngleDownOutline,
    AngleUpOutline,
    GithubSolid,
    ColumnSolid,
    ClipboardSolid,
    LandmarkSolid,
    CashSolid,
    ChartMixedDollarSolid,
    LockSolid,
  } from "flowbite-svelte-icons";

  let {
    accounts,
    holders,
    drawerHidden = $bindable(),
    app_state,
    // selectedAccounts = $bindable(),
  }: {
    accounts: Account[];
    holders: Holder[];
    drawerHidden: boolean;
    app_state: AppState;
    // selectedAccounts: Account[];
  } = $props();

  //
  let hide_closed_accounts = $state(true); // Start with opened accounts
  let shown_holders = $state(holders); // Show all holders
  let now = DateWrapper.create_from_date(new Date());

  // Get all custodians
  const custodians_ = accounts.map((acc: Account) => {
    return {
      id: acc.custodian().pk(),
      value: acc.custodian(),
    };
  });
  const custodians = custodians_
    .filter((item, index) => custodians_.findIndex((value) => item.id === value.id) === index)
    .sort((lhs, rhs) => lhs.value.name().localeCompare(rhs.value.name()));

  // // All the accounts that satisfy the filters
  // const filtered_accounts = () => {
  //   return accounts.filter((acc: Account) => {
  //     // Consider if closed
  //     if (hide_closed_accounts && acc.close()?.less_than(now)) {
  //       return false;
  //     }

  //     // Consider holders
  //     return shown_holders.find((holder: Holder) => {
  //       return acc.holders().find((h: Holder) => h.pk() === holder.pk()) !== undefined;
  //     });
  //   });
  // };

  // const all_accounts = () => {
  //   selectedAccounts = filtered_accounts();
  // };

  // // Accounts (filtered) for a given custodian
  // const accounts_for_custodian = (custodian_pk: number) => {
  //   selectedAccounts = filtered_accounts().filter((acc: Account) => acc.custodian().pk() === custodian_pk);
  // };

  // // Accounts (filtered) for a given category
  // const accounts_for_category = (category: AccountCategory) => {
  //   selectedAccounts = filtered_accounts().filter((acc: Account) => acc.type().category() === category);
  // };

  // UI stuff
  const closeDrawer = () => {
    drawerHidden = true;
  };

  const groupClass = "pt-2 space-y-2";

  const iconClass =
    "flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white";

  const itemClass =
    "flex items-center p-2 text-base text-gray-900 transition duration-75 rounded-lg hover:bg-gray-100 group dark:text-gray-200 dark:hover:bg-gray-700";
</script>

<Sidebar
  class={drawerHidden ? "hidden" : ""}
  activeClass="bg-gray-100 dark:bg-gray-700"
  asideClass="fixed inset-0 z-30 flex-none h-full w-64 lg:h-auto border-e border-gray-200 dark:border-gray-600 lg:overflow-y-visible lg:pt-16 lg:block"
>
  <h4 class="sr-only">Main menu</h4>
  <SidebarWrapper
    divClass="overflow-y-auto px-3 pt-20 lg:pt-5 h-full bg-white scrolling-touch max-w-2xs lg:h-[calc(100vh-4rem)] lg:block dark:bg-gray-800 lg:me-0 lg:sticky top-2"
  >
    <nav class="divide-y divide-gray-200 dark:divide-gray-700">
      <SidebarGroup ulClass={groupClass} class="mb-3">
        <SidebarItem label="All" href="/">
          <svelte:fragment slot="icon">
            <ClipboardSolid class={iconClass} />
          </svelte:fragment>
        </SidebarItem>

        <SidebarDropdownWrapper isOpen={false} label="By custodian" class="pr-3">
          <svelte:fragment slot="icon">
            <LandmarkSolid class={iconClass} />
          </svelte:fragment>
          <svelte:fragment slot="arrowup">
            <AngleDownOutline class="w-6 h-6" />
          </svelte:fragment>
          <svelte:fragment slot="arrowdown">
            <AngleUpOutline class="w-6 h-6" />
          </svelte:fragment>
          {#each custodians as custodian}
            <SidebarItem
              label={custodian.value.name()}
              href="/custodian/{custodian.id}/accounts/all"
              spanClass="ml-9"
              class={itemClass}
            />
          {/each}
        </SidebarDropdownWrapper>

        {#each Object.values(AccountCategory) as value}
          <SidebarItem
            ulClass={groupClass}
            spanClass="ml-3"
            label={value}
            href="/category/{value.toString().toLowerCase()}/accounts/all"
          >
            <svelte:fragment slot="icon">
              {#if value === AccountCategory.Investment}
                <ChartMixedDollarSolid class={iconClass} />
              {:else if value === AccountCategory.Savings}
                <CashSolid class={iconClass} />
              {:else if value === AccountCategory.Retirement}
                <LockSolid class={iconClass} />
              {:else}
                <ColumnSolid class={iconClass} />
              {/if}
            </svelte:fragment>
          </SidebarItem>
        {/each}

        <SidebarItem ulClass={groupClass} spanClass="ml-3" label="Savings" href="/category/savings/accounts/all">
          <svelte:fragment slot="icon">
            <CashSolid class={iconClass} />
          </svelte:fragment>
        </SidebarItem>

        <SidebarItem ulClass={groupClass} spanClass="ml-3" label="Investment" href="/category/investment/accounts/all">
          <svelte:fragment slot="icon">
            <ChartMixedDollarSolid class={iconClass} />
          </svelte:fragment>
        </SidebarItem>

        <SidebarItem ulClass={groupClass} spanClass="ml-3" label="Retirement" href="/category/retirement/accounts/all">
          <svelte:fragment slot="icon">
            <LockSolid class={iconClass} />
          </svelte:fragment>
        </SidebarItem>
      </SidebarGroup>

      <SidebarGroup ulClass={groupClass}>
        <SidebarItem
          label="totum-revolutum"
          href="https://github.com/jgsogo/totum-revolutum"
          ulClass={groupClass}
          spanClass="ml-3"
          target="_blank"
        >
          <svelte:fragment slot="icon">
            <GithubSolid class={iconClass} />
          </svelte:fragment>
        </SidebarItem>

        <SidebarItem
          label="Admin interface"
          href={`${app_state.base_url()}/admin`}
          ulClass={groupClass}
          spanClass="ml-3"
          target="_blank"
        >
          <svelte:fragment slot="icon">
            <ColumnSolid class={iconClass} />
          </svelte:fragment>
        </SidebarItem>
      </SidebarGroup>
    </nav>
  </SidebarWrapper>
</Sidebar>

<div
  hidden={drawerHidden}
  class="fixed inset-0 z-20 bg-gray-900/50 dark:bg-gray-900/60"
  onclick={closeDrawer}
  onkeydown={closeDrawer}
  role="presentation"
></div>
