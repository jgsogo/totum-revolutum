<script lang="ts">
  import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";
  import type { MainContext, Transaction, Movement } from "../../../models/src-js/index";
  import { sort_date_wrapper } from "../../../../../../libraries/googleapis/src-js/date";
  import { MovementDirection } from "../../../models/src-js/movement";

  let { transaction, main_context }: { transaction: Transaction; main_context: MainContext } = $props();

  let movements = [...transaction.movements_from(), ...transaction.movements_to()].sort(
    (lhs: Movement, rhs: Movement) => {
      const lhs_date = lhs.date_value()!;
      const rhs_date = rhs.date_value()!;
      let v = sort_date_wrapper(lhs_date, rhs_date);
      if (v === 0) {
        return lhs.type().breadcrumb()!.toString().localeCompare(rhs.type().breadcrumb()!.toString());
      } else {
        return v;
      }
    }
  );
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
        <TableHeadCell>Date</TableHeadCell>
        <TableHeadCell>Account</TableHeadCell>
        <TableHeadCell>Movement Type</TableHeadCell>
        <TableHeadCell>In</TableHeadCell>
        <TableHeadCell>Out</TableHeadCell>
      </TableHead>
      <TableBody tableBodyClass="divide-y">
        {#each movements as mov}
          <TableBodyRow>
            <TableBodyCell>{mov.date_value()}</TableBodyCell>
            <TableBodyCell>{main_context.find_account(mov.account_pk())?.name() ?? mov.account_pk()}</TableBodyCell>
            <TableBodyCell>{mov.type().breadcrumb()}</TableBodyCell>
            {#if mov.direction() == MovementDirection.Out}
              <TableBodyCell>{mov.amount()}</TableBodyCell>
              <TableBodyCell></TableBodyCell>
            {:else}
              <TableBodyCell></TableBodyCell>
              <TableBodyCell>{mov.amount()}</TableBodyCell>
            {/if}
          </TableBodyRow>
        {/each}
        {#each transaction.movements_to() as mov}
          <TableBodyRow>
            <TableBodyCell>{mov.date_value()}</TableBodyCell>
            <TableBodyCell>{main_context.find_account(mov.account_pk())?.name() ?? mov.account_pk()}</TableBodyCell>
            <TableBodyCell>{mov.type().breadcrumb()}</TableBodyCell>
            {#if mov.direction() == MovementDirection.Out}
              <TableBodyCell>{mov.amount()}</TableBodyCell>
              <TableBodyCell></TableBodyCell>
            {:else}
              <TableBodyCell></TableBodyCell>
              <TableBodyCell>{mov.amount()}</TableBodyCell>
            {/if}
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  </div>

  <div>Add "expand" button to see more transactions in the transaction_group</div>
</div>
