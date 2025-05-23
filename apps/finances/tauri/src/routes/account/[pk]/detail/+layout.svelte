<script lang="ts">
  import AccountChart from '$lib/components/AccountChart.svelte';
  import AccountDetail from '$lib/components/AccountDetail.svelte';
  import type { Account, AccountContext, AppState, MainContext } from '../../../../../../models/src-js';
  import { Breadcrumb, BreadcrumbItem, Heading } from 'flowbite-svelte';

  /** @type {{ data: import('./$types').PageData }} */
  let { data, children } = $props();

  let app_state: AppState = data.app_state;

  let account_context: AccountContext = data.account_context;
  let account: Account = account_context.account();
</script>

<main class="p-4">
  <div class="grid grid-cols-1 space-y-2 dark:bg-gray-900">
    <div class="col-span-full xl:mb-0">
      <Breadcrumb class="mb-6">
        <BreadcrumbItem home>Home</BreadcrumbItem>
        <BreadcrumbItem class="hover:text-primary-600 inline-flex items-center text-gray-700 dark:text-gray-300 dark:hover:text-white"
          >Account</BreadcrumbItem
        >
        <BreadcrumbItem>{account.name()}</BreadcrumbItem>
      </Breadcrumb>

      <Heading tag="h1" class="text-xl font-semibold text-gray-900 sm:text-2xl dark:text-white">{account.name()}</Heading>
    </div>

    <div class="grid grid-cols-2 gap-4 dark:bg-gray-900">
      <AccountDetail base_media_url={app_state.base_media_url()} {account}></AccountDetail>
      <AccountChart {account} snapshots={account_context.snapshots()}></AccountChart>
    </div>

    <div class="grid grid-cols-1 gap-4">
      {@render children()}
    </div>
  </div>
</main>
