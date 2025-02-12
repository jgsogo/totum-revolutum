<script lang="ts">
  import {
    Card,
    Modal,
    Spinner,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import { Snapshot, Movement, Transaction, MainContext } from "../../../models/src-js";
  import { sort_date_wrapper } from "../../../../../../libraries/googleapis/src-js/date";
  import { get_transaction } from "$lib/commands";
  import TransactionDetailCard from "./TransactionDetailCard.svelte";

  let {
    snapshots,
    movements,
    main_context,
  }: { snapshots: Snapshot[]; movements: Movement[]; main_context: MainContext } = $props();

  const entries = $derived(
    [...snapshots, ...movements].sort((lhs, rhs) => sort_date_wrapper(rhs.date_value(), lhs.date_value()))
  );

  let class_row_snapshot = "bg-gray-300 dark:bg-gray-700";
  let class_row_movement = "";

  let transaction_details_modal: boolean = $state(false);
  let transaction_details: Transaction | null = $state(null);
  const showModal = async (details: Transaction) => {
    transaction_details_modal = true;
    transaction_details = details;
  };
</script>

<Card size="xl" class="shadow-sm max-w-none">
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
            <TableBodyCell>{entry.amount().amount()}</TableBodyCell>
            <TableBodyCell></TableBodyCell>
          </TableBodyRow>
        {:else}
          {#await get_transaction(entry.transaction_pk()!)}
            <TableBodyRow class={class_row_movement}>
              <TableBodyCell>{entry.date_value()}</TableBodyCell>
              <TableBodyCell>{entry.direction()}</TableBodyCell>
              <TableBodyCell>{entry.type()}</TableBodyCell>
              <TableBodyCell>{entry.amount()}</TableBodyCell>
              <TableBodyCell><Spinner /></TableBodyCell>
            </TableBodyRow>
          {:then transaction: Transaction}
            <TableBodyRow class={class_row_movement} on:click={() => showModal(transaction)}>
              <TableBodyCell>{entry.date_value()}</TableBodyCell>
              <TableBodyCell>{entry.direction()}</TableBodyCell>
              <TableBodyCell>{entry.type()}</TableBodyCell>
              <TableBodyCell>{entry.amount()}</TableBodyCell>
              <TableBodyCell>
                {transaction.name()} ({transaction.movements_from().length} - {transaction.movements_to().length})
              </TableBodyCell>
            </TableBodyRow>
          {/await}

          <!-- {#if openRow === i}
            <TableBodyRow>
              <TableBodyCell colspan="5" class="p-0">
                <div class="px-2 py-3" transition:slide={{ duration: 300, axis: "y" }}>
                  <TransactionDetailCard {transaction} {holder_context} />
                </div>
              </TableBodyCell>
            </TableBodyRow>
          {/if} -->
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
