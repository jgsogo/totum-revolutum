<script lang="ts">
  import {
    Toggle,
    Toolbar,
    Input,
    ToolbarButton,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    Card,
    Heading,
    Table,
  } from 'flowbite-svelte';
  import { FxQuote, FxQuotePair, type Account, type AppState, type Holder } from '../../../models/src-js';
  import { goToAccountDetail } from '$lib/utils';
  import { CurrencyCode, DateWrapper, Decimal, Money } from '../../../../../../libraries/googleapis/src-js';
  import { get_fx_spot } from '$lib/commands';
  import MoneyString from './MoneyString.svelte';

  let {
    app_state,
    accounts,
    show_custodian = true,
    show_holders = true,
    show_category = true,
  }: {
    app_state: AppState;
    accounts: Account[];
    show_custodian?: boolean;
    show_holders?: boolean;
    show_category?: boolean;
  } = $props();

  let holder = app_state.holder();

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
  let total = $state<Money | undefined>();
  $effect(() => {
    const x = async () => {
      let zero_eur = Money.create_from_number(app_state.base_ccy(), 0);
      total = await filteredAccounts.reduce(async (totalP: Promise<Money>, item: Account) => {
        const total: Money = await totalP;
        if (item.last_snapshot()) {
          const amount_snapshot = item.last_snapshot()!.amount().amount();
          if (amount_snapshot.currency_code() === total.currency_code()) {
            return total.sum(amount_snapshot);
          } else {
            const fx_rate = await get_fx_spot(amount_snapshot.currency_code().toString());
            const fx_pair = FxQuotePair.create_from(total.currency_code(), amount_snapshot.currency_code());
            const fx = FxQuote.create_from(DateWrapper.create_from_date(new Date()), fx_pair, Decimal.create_from_number(fx_rate));

            const amount_snapshot_local = fx?.apply_to(amount_snapshot)!;
            return total.sum(amount_snapshot_local);
          }
        }
        return total;
      }, Promise.resolve(zero_eur));
    };
    x();
  });
</script>

<Card size="xl" class="block p-4 shadow-sm sm:flex sm:space-x-4 sm:p-6 sm:py-6 xl:block xl:space-x-0" horizontal>
  <Toolbar embedded class="w-full py-4 text-gray-500 dark:text-gray-300">
    <Input bind:value={searchTerm} placeholder="Search for accounts" class="me-6 w-80 border xl:w-96" />
    <Toggle bind:checked={show_closed}>Show closed</Toggle>
    <Toggle bind:checked={show_other_holders}>Show other holders</Toggle>
  </Toolbar>

  <Table hoverable={true} noborder striped class="mt-6 min-w-full divide-y divide-gray-200 dark:divide-gray-600">
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
          <TableBodyCell class="text-right"><MoneyString {app_state} money={account.last_snapshot()?.amount().amount()} font_mono={true} tooltip={true}/></TableBodyCell>
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
        <td class="px-6 py-3 text-right"><MoneyString money={total} /></td>
      </tr>
    </tfoot>
  </Table>
</Card>
