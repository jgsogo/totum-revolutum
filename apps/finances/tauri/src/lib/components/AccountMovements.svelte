<script lang="ts">
  import {
    Card,
    ImagePlaceholder,
    Spinner,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import { Snapshot, Movement, Transaction } from "../../../models/src-js";
  import { sort_date_wrapper } from "../../../../../../libraries/googleapis/src-js/date";
  import { get_transaction } from "$lib/commands";
  import { slide } from "svelte/transition";
  import TransactionDetailCard from "./TransactionDetailCard.svelte";

  let { snapshots, movements }: { snapshots: Snapshot[]; movements: Movement[] } = $props();

  const entries = $derived(
    [...snapshots, ...movements].sort((lhs, rhs) => sort_date_wrapper(rhs.date_value(), lhs.date_value()))
  );

  let class_row_snapshot = "bg-gray-300 dark:bg-gray-700";
  let class_row_movement = "";

  let openRow: number | null = $state(null);

  const toggleRow = (i: number) => {
    openRow = openRow === i ? null : i;
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
            <TableBodyRow class={class_row_movement} on:click={() => toggleRow(i)}>
              <TableBodyCell>{entry.date_value()}</TableBodyCell>
              <TableBodyCell>{entry.direction()}</TableBodyCell>
              <TableBodyCell>{entry.type()}</TableBodyCell>
              <TableBodyCell>{entry.amount()}</TableBodyCell>
              <TableBodyCell><Spinner /></TableBodyCell>
            </TableBodyRow>
          {:then transaction: Transaction}
            <TableBodyRow class={class_row_movement} on:click={() => toggleRow(i)}>
              <TableBodyCell>{entry.date_value()}</TableBodyCell>
              <TableBodyCell>{entry.direction()}</TableBodyCell>
              <TableBodyCell>{entry.type()}</TableBodyCell>
              <TableBodyCell>{entry.amount()}</TableBodyCell>
              <TableBodyCell>
                {transaction.name()} ({transaction.movements_from().length} - {transaction.movements_to().length})
              </TableBodyCell>
            </TableBodyRow>
            {#if openRow === i}
              <TableBodyRow>
                <TableBodyCell colspan="5" class="p-0">
                  <div class="px-2 py-3" transition:slide={{ duration: 300, axis: "y" }}>
                    <TransactionDetailCard {transaction}/>
                  </div>
                </TableBodyCell>
              </TableBodyRow>
            {/if}
          {/await}
        {/if}
      {/each}
    </TableBody>
  </Table>
</Card>
