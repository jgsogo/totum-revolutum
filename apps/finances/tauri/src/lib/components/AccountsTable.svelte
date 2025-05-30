<script lang="ts">
  import {
    Toggle,
    Tooltip,
    Toolbar,
    Input,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    Card,
    Table,
    Badge,
  } from 'flowbite-svelte';
  import { FxQuote, FxQuotePair, type Account, type AppState, type Holder } from '../../../models/src-js';
  import { goToAccountDetail } from '$lib/utils';
  import { DateWrapper, Decimal, Money } from '../../../../../../libraries/googleapis/src-js';
  import { get_fx_spot } from '$lib/commands';
  import MoneyString from './MoneyString.svelte';
  import { EnvelopeSolid, LockOutline } from 'flowbite-svelte-icons';
  import LastSnapshotMoneyString from './LastSnapshotMoneyString.svelte';

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

      // Match search term to account name
      matched = matched && (!searchTerm || item.name().toLowerCase().indexOf(searchTerm.toLowerCase()) !== -1);

      // Match show closed
      matched = matched && (show_closed || !item.close()?.lte(today));

      // Match show others
      matched = matched && (show_other_holders || item.holders().findIndex((h: Holder) => h.pk() === holder?.pk()) !== -1);

      return matched;
    }),
  );

  // Notes associated to accounts
  const max_days_old_for_warning = 35;
  const date_warning = DateWrapper.create_from_date(new Date(new Date().setDate(new Date().getDate() - max_days_old_for_warning)));
  const max_days_old_for_error = 50;
  const date_error = DateWrapper.create_from_date(new Date(new Date().setDate(new Date().getDate() - max_days_old_for_error)));
  const colors = new Map([
    [0, 'blue'],
    [1, 'yellow'],
    [2, 'red'],
  ]);
  async function get_account_notes(account: Account): Promise<[string, string[]]> {
    let color_level = 0;
    let notifications: string[] = [];

    const holded_by_me = app_state.holder() && account.holded_by(app_state.holder()!);
    const last_snapshot = account.last_snapshot();

    // Accounts that are holded by me. I can create new snapshots if needed
    if (holded_by_me) {
      // Snpashot?
      if (!last_snapshot) {
        color_level = Math.max(color_level, 2);
        notifications.push('Add a snapshot to this account');
      }

      // Snapshot date
      if (last_snapshot && last_snapshot.date_value().less_than(date_error)) {
        color_level = Math.max(color_level, 2);
        notifications.push(`Last snapshot is ${max_days_old_for_error} days old or more. Please, add one snapshot`);
      } else if (last_snapshot && last_snapshot?.date_value().less_than(date_warning)) {
        color_level = Math.max(color_level, 1);
        notifications.push(`Last snapshot is ${max_days_old_for_warning} days old or more.`);
      }
    }

    // Closed accounts
    if (account.close()) {
      if (!last_snapshot) {
        color_level = Math.max(color_level, 2);
        notifications.push('Account is closed, but there is no snapshot!');
      }
      if (last_snapshot && last_snapshot.date_value().less_than(account.close()!)) {
        color_level = Math.max(color_level, 1);
        notifications.push('Account is closed, but last snapshot is for a previous date');
      }
    }

    const color = colors.get(color_level);
    return [color || 'red', notifications];
  }

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
      <TableHeadCell></TableHeadCell>
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
          <TableBodyCell>
            {#await get_account_notes(account) then [color, notifications]}
              {#if notifications.length}
                <Badge {color} border>
                  <EnvelopeSolid class="me-1.5 h-2.5 w-2.5" />
                  {notifications.length}
                </Badge>
                <Tooltip>
                  {#each notifications as notification}
                    <p>{notification}</p>
                  {/each}
                </Tooltip>
              {/if}
            {/await}
          </TableBodyCell>
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
          <TableBodyCell
            >{account.name()}{#if account.close()}<LockOutline class="m-1 inline" /><Tooltip>Closed account</Tooltip>{/if}</TableBodyCell
          >
          {#if show_category}
            <TableBodyCell>{account.type().category()}</TableBodyCell>
          {/if}
          <TableBodyCell>{account.type().name()}</TableBodyCell>
          <TableBodyCell class="text-right">
            <!-- TODO: Get the movements here, so we can use an updated snapshot -->
            <LastSnapshotMoneyString {app_state} {account} font_mono={true} tooltip={true} />
          </TableBodyCell>
        </TableBodyRow>
      {/each}
    </TableBody>
    <tfoot>
      <tr class="font-semibold text-gray-900 dark:text-white">
        <td></td>
        {#if show_custodian}<td></td>{/if}
        {#if show_holders}<td></td>{/if}
        <td></td>
        {#if show_category}<td></td>{/if}
        <th scope="row" class="px-6 py-3 text-base">Total</th>
        <td class="px-6 py-3 text-right"><MoneyString {app_state} bind:money={total} font_mono={true} tooltip={true} /></td>
      </tr>
    </tfoot>
  </Table>
</Card>
