<script lang="ts">
  import { Card, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";
  import { Account, Snapshot, Movement } from "../../../models/src-js";

  let {
    account,
    snapshots,
    movements,
  }: { account: Account; snapshots: Snapshot[]; movements: Movement[] } = $props();

  const entries = $derived([...snapshots, ...movements].sort((lhs, rhs) => new Date(rhs.date_value).getTime() - new Date(lhs.date_value).getTime()));

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
            <TableBodyCell>{entry.date_value}</TableBodyCell>
            <TableBodyCell></TableBodyCell>
            <TableBodyCell></TableBodyCell>
            <TableBodyCell>{entry.amount}</TableBodyCell>
            <TableBodyCell></TableBodyCell>
          </TableBodyRow>
        {:else}
        <!-- TODO: On click, we can show the information about the Transaction this moement belongs to. There is an example in the official Flowbite documentation about Table component (https://flowbite-svelte.com/docs/components/table#Click_and_double-click_on_row) -->
        <TableBodyRow class={class_row_movement}>
            <TableBodyCell>{entry.date_value}</TableBodyCell>
            <TableBodyCell>{entry.direction}</TableBodyCell>
            <TableBodyCell>TODO: type</TableBodyCell>
            <TableBodyCell>{entry.amount}</TableBodyCell>
            <TableBodyCell>TODO: transaction</TableBodyCell>
          </TableBodyRow>
        {/if}
      {/each}
    </TableBody>
  </Table>
</Card>
