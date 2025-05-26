<script lang="ts">
  import { Sidebar, SidebarWrapper, SidebarItem, SidebarGroup, SidebarDropdownWrapper } from 'flowbite-svelte';
  import { AccountCategory, type Account, type AppState, type Holder } from '../../../models/src-js';
  import { AngleDownOutline, AngleUpOutline, GithubSolid, ColumnSolid, ClipboardSolid, LandmarkSolid } from 'flowbite-svelte-icons';

  let {
    accounts,
    holders,
    drawerHidden = $bindable(),
    app_state,
  }: {
    accounts: Account[];
    holders: Holder[];
    drawerHidden: boolean;
    app_state: AppState;
  } = $props();

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

  let iconClass =
    'flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-300 dark:group-hover:text-white';
  let itemClass =
    'flex items-center p-2 text-base text-gray-900 transition duration-75 rounded-lg hover:bg-gray-100 group dark:text-gray-200 dark:hover:bg-gray-700 w-full';
  let groupClass = 'pt-2 space-y-2 mb-3';
</script>

<!-- <SidebarButton breakpoint="lg" class="fixed top-[22px] z-40 mb-2" /> -->
<Sidebar
  breakpoint="lg"
  backdrop={false}
  isOpen={false}
  params={{ x: -50, duration: 50 }}
  class="top-0 left-0 w-64 h-screen transition-transform bg-gray-50 dark:bg-gray-800 lg:block mt-[69px]"
  divClass="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800"
  activeClass="p-2"
  nonActiveClass="p-2"
>
  <h4 class="sr-only">Main menu</h4>
  <SidebarWrapper
    divClass="overflow-y-auto px-3 pt-20 lg:pt-5 h-full bg-white scrolling-touch max-w-2xs lg:h-[calc(100vh-4rem)] lg:block dark:bg-gray-800 lg:me-0 lg:sticky top-2"
  >
    <SidebarGroup class={groupClass}>
      <SidebarItem label="All" href="/" spanClass="ml-3" class={itemClass} aClass="w-full p-0 py-2">
        {#snippet icon()}
          <ClipboardSolid class={iconClass} />
        {/snippet}
      </SidebarItem>

      <SidebarDropdownWrapper label="By custodian" class="pr-3">
        {#snippet arrowdown()}
          <AngleDownOutline strokeWidth="3.3" size="sm" />
        {/snippet}
        {#snippet arrowup()}
          <AngleUpOutline strokeWidth="3.3" size="sm" />
        {/snippet}
        {#snippet icon()}
          <LandmarkSolid class={iconClass} />
        {/snippet}
        {#each custodians as custodian}
          <SidebarItem
            label={custodian.value.name()}
            href="/custodian/{custodian.id}/accounts/all"
            spanClass="ml-9"
            class={itemClass}
            aClass="w-full"
          />
        {/each}
      </SidebarDropdownWrapper>

      <SidebarDropdownWrapper label="By category" class="pr-3">
        {#snippet arrowdown()}
          <AngleDownOutline strokeWidth="3.3" size="sm" />
        {/snippet}
        {#snippet arrowup()}
          <AngleUpOutline strokeWidth="3.3" size="sm" />
        {/snippet}
        {#snippet icon()}
          <LandmarkSolid class={iconClass} />
        {/snippet}
        {#each Object.values(AccountCategory) as value}
          <SidebarItem
            spanClass="ml-9"
            class={itemClass}
            aClass="w-full"
            label={value}
            href="/category/{value.toString().toLowerCase()}/accounts/all"
          ></SidebarItem>
        {/each}
      </SidebarDropdownWrapper>

      <SidebarDropdownWrapper label="By holder" class="pr-3">
        {#snippet arrowdown()}
          <AngleDownOutline strokeWidth="3.3" size="sm" />
        {/snippet}
        {#snippet arrowup()}
          <AngleUpOutline strokeWidth="3.3" size="sm" />
        {/snippet}
        {#snippet icon()}
          <LandmarkSolid class={iconClass} />
        {/snippet}
        {#each holders as holder}
          <SidebarItem spanClass="ml-9" class={itemClass} aClass="w-full" label={holder.name()} href="/holder/{holder.pk()}/accounts/all"
          ></SidebarItem>
        {/each}
      </SidebarDropdownWrapper>
    </SidebarGroup>

    <SidebarGroup class={groupClass}>
      <SidebarItem
        label="totum-revolutum"
        href="https://github.com/jgsogo/totum-revolutum"
        spanClass="ml-3"
        class={itemClass}
        aClass="w-full p-0 py-2"
      >
        {#snippet icon()}
          <GithubSolid class={iconClass} />
        {/snippet}
      </SidebarItem>
      <SidebarItem
        label="Admin interface"
        href={`${app_state.base_url()}/admin`}
        spanClass="ml-3"
        class={itemClass}
        aClass="w-full p-0 py-2"
      >
        {#snippet icon()}
          <ColumnSolid class={iconClass} />
        {/snippet}
      </SidebarItem>
    </SidebarGroup>
  </SidebarWrapper>
</Sidebar>
