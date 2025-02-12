<script lang="ts">
  import { Card, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";
  import type { AppState, HolderContext, Transaction } from "../../../models/src-js/index";

  let { transaction, holder_context }: { transaction: Transaction; holder_context: HolderContext } = $props();
</script>

<!-- <Card size="xl" class="shadow-sm max-w-none"> -->
    {transaction.name()}
    {transaction.description()}

  <Table noborder class="mt-6 min-w-full divide-y divide-gray-200 dark:divide-gray-600">
    <TableHead class="bg-gray-700 text-gray-50 dark:bg-gray-300 dark:text-gray-950">
      <TableHeadCell>Account</TableHeadCell>
      <TableHeadCell>Movement Type</TableHeadCell>
      <TableHeadCell>In</TableHeadCell>
      <TableHeadCell>Out</TableHeadCell>
    </TableHead>
    <TableBody tableBodyClass="divide-y">
      {#each transaction.movements_from() as mov}
        <TableBodyRow>
          <TableBodyCell>{mov.account_pk()}</TableBodyCell>
          <TableBodyCell>{mov.type().breadcrumb().join("/")}</TableBodyCell>
          <TableBodyCell>{mov.amount()}</TableBodyCell>
          <TableBodyCell></TableBodyCell>
        </TableBodyRow>
      {/each}
      {#each transaction.movements_to() as mov}
        <TableBodyRow>
          <TableBodyCell>{holder_context.find_account(mov.account_pk()) ?? mov.account_pk()}</TableBodyCell>
          <TableBodyCell>{mov.type().breadcrumb().join("/")}</TableBodyCell>
          <TableBodyCell></TableBodyCell>
          <TableBodyCell>{mov.amount()}</TableBodyCell>
        </TableBodyRow>
      {/each}
    </TableBody>
  </Table>
<!-- </Card> -->
