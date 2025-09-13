<script lang="ts">
  import { Breadcrumb, BreadcrumbItem, Heading } from 'flowbite-svelte';
  import AccountsTable from '$lib/components/AccountsTable.svelte';
  import { info } from '@tauri-apps/plugin-log';

  info('/page.svelte');

  /** @type {{ data: import('./$types').PageData }} */
  let { main_context, app_state } = $props();
</script>

<main class="p-4">
  <div class="grid grid-cols-1 space-y-2 dark:bg-gray-900">
    <div class="col-span-full xl:mb-0">
      <Breadcrumb class="mb-6">
        <BreadcrumbItem home>Home</BreadcrumbItem>
        <BreadcrumbItem class="hover:text-primary-600 inline-flex items-center text-gray-700 dark:text-gray-300 dark:hover:text-white"
          >Accounts</BreadcrumbItem
        >
        <BreadcrumbItem>All</BreadcrumbItem>
      </Breadcrumb>

      <Heading tag="h1" class="text-xl font-semibold text-gray-900 sm:text-2xl dark:text-white">All accounts</Heading>
    </div>
    <div class="col-span-full space-y-4 xl:col-auto">
      <AccountsTable accounts={main_context.accounts()} {app_state} />
    </div>
  </div>
</main>
