<script lang="ts">
  import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";
  import { type MainContext, type Transaction, type Movement, MovementDirection } from "../../../models/src-js";
  import { sort_date_wrapper } from "../../../../../../libraries/googleapis/src-js";

  let { transaction, main_context }: { transaction: Transaction; main_context: MainContext } = $props();

  // Order movements by date and type
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
    {transaction.group()}
  </div>

  <div>
    <Table noborder class="mt-6 min-w-full divide-y divide-gray-200 dark:divide-gray-600">
      <TableHead class="bg-gray-700 text-gray-50 dark:bg-gray-300 dark:text-gray-950">
        <TableHeadCell>Date</TableHeadCell>
        <TableHeadCell>Account</TableHeadCell>
        <TableHeadCell>Out</TableHeadCell>
        <TableHeadCell>In</TableHeadCell>
        <TableHeadCell>Movement Type</TableHeadCell>
      </TableHead>
      <TableBody tableBodyClass="divide-y">
        {#each movements as mov}
          <TableBodyRow>
            <TableBodyCell>{mov.date_value()}</TableBodyCell>
            <TableBodyCell>{main_context.find_account(mov.account_pk())?.name() ?? mov.account_pk()}</TableBodyCell>
            {#if mov.direction() === MovementDirection.Out}
            <TableBodyCell>{mov.amount()}</TableBodyCell>
            <TableBodyCell></TableBodyCell>
            {:else}
            <TableBodyCell></TableBodyCell>
            <TableBodyCell>{mov.amount()}</TableBodyCell>
            {/if}
            <TableBodyCell>{mov.type().breadcrumb()}</TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  </div>

  <div>Add "expand" button to see more transactions in the transaction_group</div>
</div>
