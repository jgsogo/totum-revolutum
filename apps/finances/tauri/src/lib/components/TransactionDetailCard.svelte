<script lang="ts">
  import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";
  import type { MainContext, Transaction } from "../../../models/src-js/index";

  let { transaction, main_context }: { transaction: Transaction; main_context: MainContext } = $props();
</script>

<div class="flex flex-col">
  <div>
    {transaction.name()}
  </div>
  <div>
    {transaction.description()}
  </div>

  <div>
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
            <TableBodyCell>{main_context.find_account(mov.account_pk())?.name() ?? mov.account_pk()}</TableBodyCell>
            <TableBodyCell>{mov.type().breadcrumb()}</TableBodyCell>
            <TableBodyCell>{mov.amount()}</TableBodyCell>
            <TableBodyCell></TableBodyCell>
          </TableBodyRow>
        {/each}
        {#each transaction.movements_to() as mov}
          <TableBodyRow>
            <TableBodyCell>{main_context.find_account(mov.account_pk())?.name() ?? mov.account_pk()}</TableBodyCell>
            <TableBodyCell>{mov.type().breadcrumb()}</TableBodyCell>
            <TableBodyCell></TableBodyCell>
            <TableBodyCell>{mov.amount()}</TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  </div>

  <div>Add "expand" button to see more transactions in the transaction_group</div>
</div>
