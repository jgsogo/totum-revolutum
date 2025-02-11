<script lang="ts">
  import {
    Card,
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

  let { snapshots, movements }: { snapshots: Snapshot[]; movements: Movement[] } = $props();

  const entries = $derived(
    [...snapshots, ...movements].sort((lhs, rhs) => sort_date_wrapper(rhs.date_value(), lhs.date_value()))
  );

  let class_row_snapshot = "bg-gray-300 dark:bg-gray-700";
  let class_row_movement = "";
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
      {#each entries as entry}
        {#if entry instanceof Snapshot}
          <TableBodyRow class={class_row_snapshot}>
            <TableBodyCell>{entry.date_value()}</TableBodyCell>
            <TableBodyCell></TableBodyCell>
            <TableBodyCell></TableBodyCell>
            <TableBodyCell>{entry.amount().amount()}</TableBodyCell>
            <TableBodyCell></TableBodyCell>
          </TableBodyRow>
        {:else}
          <TableBodyRow class={class_row_movement}>
            <TableBodyCell>{entry.date_value()}</TableBodyCell>
            <TableBodyCell>{entry.direction()}</TableBodyCell>
            <TableBodyCell>{entry.type()}</TableBodyCell>
            <TableBodyCell>{entry.amount()}</TableBodyCell>
            {#await get_transaction(entry.transaction_pk()!)}
              <TableBodyCell><Spinner /></TableBodyCell>
            {:then transaction: Transaction}
              <!-- TODO: On click, we can show the information about the Transaction this movement belongs to. There is an example in the official Flowbite documentation about Table component (https://flowbite-svelte.com/docs/components/table#Click_and_double-click_on_row) -->
              <TableBodyCell>
                {transaction.name()} ({transaction.movements_from().length} - {transaction.movements_to().length})
              </TableBodyCell>
            {:catch e}
              <TableBodyCell>{e}</TableBodyCell>
            {/await}
          </TableBodyRow>
        {/if}
      {/each}
    </TableBody>
  </Table>
</Card>
