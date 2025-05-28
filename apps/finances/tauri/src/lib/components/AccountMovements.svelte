<script lang="ts">
  import { Card, Modal, Spinner, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from 'flowbite-svelte';
  import { Snapshot, Movement, Transaction, MainContext, AppState } from '../../../models/src-js';
  import { sort_date_wrapper } from '../../../../../../libraries/googleapis/src-js/date';
  import { get_transaction } from '$lib/commands';
  import TransactionDetailCard from './TransactionDetailCard.svelte';
  import MoneyString from './MoneyString.svelte';

  let {
    app_state,
    snapshots,
    movements,
    main_context,
  }: { app_state: AppState; snapshots: Snapshot[]; movements: Movement[]; main_context: MainContext } = $props();

  // Order together movements and snapshots: more recent items go first, snapshots go last (EOD)
  const entries = $derived(
    [...snapshots, ...movements].sort((lhs, rhs) => {
      const r = sort_date_wrapper(rhs.date_value(), lhs.date_value());
      if (r === 0) {
        return rhs instanceof Snapshot ? 1 : lhs instanceof Snapshot ? -1 : 0;
      } else {
        return r;
      }
    }),
  );

  let class_row_snapshot = 'bg-gray-300 dark:bg-gray-700';
  let class_row_movement = '';

  let transaction_details_modal: boolean = $state(false);
  let transaction_details: Transaction | null = $state(null);
  const showModal = async (details: Transaction) => {
    transaction_details_modal = true;
    transaction_details = details;
  };
</script>

<Card size="xl" class="p-4 sm:p-6">
  <Table noborder class="mt-6 min-w-full divide-y divide-gray-200 dark:divide-gray-600">
    <TableHead class="bg-gray-700 text-gray-50 dark:bg-gray-300 dark:text-gray-950">
      <TableHeadCell>Date</TableHeadCell>
      <TableHeadCell>Direction</TableHeadCell>
      <TableHeadCell>Type</TableHeadCell>
      <TableHeadCell>Amount</TableHeadCell>
      <TableHeadCell>Transaction</TableHeadCell>
    </TableHead>
    <TableBody tableBodyClass="divide-y">
      {#each entries as entry, i}
        {#if entry instanceof Snapshot}
          <TableBodyRow class={class_row_snapshot}>
            <TableBodyCell>{entry.date_value()}</TableBodyCell>
            <TableBodyCell></TableBodyCell>
            <TableBodyCell></TableBodyCell>
            <TableBodyCell class="text-right"><MoneyString {app_state} money={entry.amount().amount()} font_mono={true} /></TableBodyCell>
            <TableBodyCell></TableBodyCell>
          </TableBodyRow>
        {:else}
          {#await get_transaction(entry.transaction_pk()!)}
            <TableBodyRow class={class_row_movement}>
              <TableBodyCell>{entry.date_value()}</TableBodyCell>
              <TableBodyCell>{entry.direction()}</TableBodyCell>
              <TableBodyCell>{entry.type()}</TableBodyCell>
              <TableBodyCell class="text-right"><MoneyString {app_state} money={entry.amount()} font_mono={true} /></TableBodyCell>
              <TableBodyCell><Spinner /></TableBodyCell>
            </TableBodyRow>
          {:then transaction: Transaction}
            <TableBodyRow class={class_row_movement} onclick={() => showModal(transaction)}>
              <TableBodyCell>{entry.date_value()}</TableBodyCell>
              <TableBodyCell>{entry.direction()}</TableBodyCell>
              <TableBodyCell>{entry.type()}</TableBodyCell>
              <TableBodyCell class="text-right"><MoneyString {app_state} money={entry.amount()} font_mono={true} /></TableBodyCell>
              <TableBodyCell>{transaction.name()}</TableBodyCell>
            </TableBodyRow>
          {/await}
        {/if}
      {/each}
    </TableBody>
  </Table>
</Card>

<Modal bind:open={transaction_details_modal} size="xl" class="w-full h-full" autoclose outsideclose>
  {#if transaction_details}
    <TransactionDetailCard transaction={transaction_details} {main_context} />
  {:else}
    Error: There is no transaction to show!
  {/if}
</Modal>
