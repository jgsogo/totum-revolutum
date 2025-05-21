<script lang="ts">
  import { Toggle, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, Card, Heading, TableSearch } from 'flowbite-svelte';
  import type { Account, Holder } from '../../../models/src-js';
  import { goToAccountDetail } from '$lib/utils';
  import { DateWrapper } from '../../../../../../libraries/googleapis/src-js';

  let {
    accounts,
    holder,
    show_custodian = true,
    show_holders = true,
    show_category = true,
  }: {
    accounts: Account[];
    holder: Holder | undefined;
    show_custodian?: boolean;
    show_holders?: boolean;
    show_category?: boolean;
  } = $props();

  // Filters and search
  let searchTerm = $state('');
  let show_closed = $state(false);
  let show_other_holders = $state(false);

  let today = DateWrapper.create_from_date(new Date());
  let filteredAccounts: Account[] = $derived(
    accounts.filter((item) => {
      let matched = item.open().lte(today);

      // Match search term
      matched = matched && (!searchTerm || item.name().toLowerCase().indexOf(searchTerm.toLowerCase()) !== -1);

      // Match show closed
      matched = matched && (show_closed || !item.close()?.lte(today));

      // Match show others
      matched = matched && (show_other_holders || item.holders().findIndex((h: Holder) => h.pk() === holder?.pk()) !== -1);

      return matched;
    }),
  );

  // Total sum
  let total = $derived(
    // FIXME: Take into account FX (today - spot) when summing different ccys
    filteredAccounts.reduce((sum, item) => sum + (item.last_snapshot() ? item.last_snapshot()!.amount().amount().amount() : 0), 0),
  );
</script>

<Card size="xl" class="block p-4 shadow-sm sm:flex sm:space-x-4 sm:p-6 sm:py-6 xl:block xl:space-x-0 2xl:flex 2xl:space-x-4" horizontal>
  <div class="items-center justify-between lg:flex">
    <Toggle bind:checked={show_closed}>Show closed</Toggle>
    <Toggle bind:checked={show_other_holders}>Show other holders</Toggle>
  </div>
  <TableSearch
    placeholder="Search any column"
    bind:inputValue={searchTerm}
    hoverable={true}
    noborder
    striped
    class="mt-6 min-w-full divide-y divide-gray-200 dark:divide-gray-600"
  >
    <TableHead class="bg-gray-50 dark:bg-gray-700">
      {#if show_custodian}
        <TableHeadCell>Custodian</TableHeadCell>
      {/if}
      {#if show_holders}
        <TableHeadCell>Holders</TableHeadCell>
      {/if}
      <TableHeadCell>Name</TableHeadCell>
      {#if show_category}
        <TableHeadCell>Category</TableHeadCell>
      {/if}
      <TableHeadCell>Type</TableHeadCell>
      <TableHeadCell>Snapshot</TableHeadCell>
    </TableHead>
    <TableBody tableBodyClass="divide-y">
      {#each filteredAccounts as account}
        <TableBodyRow onclick={() => goToAccountDetail(account)}>
          {#if show_custodian}
            <TableBodyCell>{account.custodian().name()}</TableBodyCell>
          {/if}
          {#if show_holders}
            <TableBodyCell>
              {account
                .holders()
                .map((v) => v.name())
                .join(', ')}
            </TableBodyCell>
          {/if}
          <TableBodyCell>{account.name()}</TableBodyCell>
          {#if show_category}
            <TableBodyCell>{account.type().category()}</TableBodyCell>
          {/if}
          <TableBodyCell>{account.type().name()}</TableBodyCell>
          <TableBodyCell>{account.last_snapshot() ? account.last_snapshot()!.amount().amount() : '-'}</TableBodyCell>
        </TableBodyRow>
      {/each}
    </TableBody>
    <tfoot>
      <tr class="font-semibold text-gray-900 dark:text-white">
        {#if show_custodian}<td></td>{/if}
        {#if show_holders}<td></td>{/if}
        <td></td>
        {#if show_category}<td></td>{/if}
        <th scope="row" class="px-6 py-3 text-base">Total</th>
        <td class="px-6 py-3">{total}</td>
      </tr>
    </tfoot>
  </TableSearch>
</Card>
