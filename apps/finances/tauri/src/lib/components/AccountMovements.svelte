<script lang="ts">
  import { Card, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";
  import type { Account } from "$lib/models/Account";
  import type { Snapshot } from "$lib/models/Snapshot";
  import type { Movement } from "$lib/models/Movement";

  let {
    account = $bindable(),
    snapshots = $bindable(),
    movements = $bindable(),
  }: { account: Account; snapshots: Snapshot[]; movements: Movement[] } = $props();

  const entries = [...snapshots, ...movements];
  entries.sort((lhs, rhs) => new Date(rhs.date_value).getTime() - new Date(lhs.date_value).getTime());
</script>

<Card size="xl" class="shadow-sm max-w-none">
  <Table hoverable={true} noborder striped class="mt-6 min-w-full divide-y divide-gray-200 dark:divide-gray-600">
    <TableHead class="bg-gray-50 dark:bg-gray-700">
      <TableHeadCell>Date</TableHeadCell>
      <TableHeadCell>Direction</TableHeadCell>
      <TableHeadCell>Type</TableHeadCell>
      <TableHeadCell>Amount</TableHeadCell>
      <TableHeadCell>Transaction</TableHeadCell>
    </TableHead>
    <TableBody tableBodyClass="divide-y">
      {#each entries as entry}
        <TableBodyRow>
          <TableBodyCell>{entry.date_value}</TableBodyCell>
          <TableBodyCell>dir</TableBodyCell>
          <TableBodyCell>type</TableBodyCell>
          <TableBodyCell>{entry.amount}</TableBodyCell>
          <TableBodyCell>transaction</TableBodyCell>
        </TableBodyRow>
      {/each}
    </TableBody>
  </Table>
</Card>
