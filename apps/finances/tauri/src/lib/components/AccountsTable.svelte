<script lang="ts">
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    Checkbox,
    TableSearch,
    Card,
    Heading,
  } from "flowbite-svelte";

  let { accounts = $bindable(), show_custodian = true } = $props();
</script>

<Card size="xl" class="shadow-sm max-w-none">
  <div class="items-center justify-between lg:flex">
    <div class="mb-4 mt-px lg:mb-0">
      <Heading tag="h3" class="-ml-0.25 mb-2 text-xl font-semibold dark:text-white">Accounts</Heading>
      <span class="text-base font-normal text-gray-500 dark:text-gray-400"> This is a list of accounts </span>
    </div>
  </div>
  <Table hoverable={true} noborder striped class="mt-6 min-w-full divide-y divide-gray-200 dark:divide-gray-600">
    <TableHead class="bg-gray-50 dark:bg-gray-700">
      {#if show_custodian}
        <TableHeadCell>Custodian</TableHeadCell>
      {/if}
      <TableHeadCell>Name</TableHeadCell>
      <TableHeadCell>Type</TableHeadCell>
      <TableHeadCell>Snapshot</TableHeadCell>
    </TableHead>
    <TableBody tableBodyClass="divide-y">
      {#each accounts as account}
        <TableBodyRow>
          {#if show_custodian}
            <TableBodyCell>{account.custodian}</TableBodyCell>
          {/if}
          <TableBodyCell>{account.name}</TableBodyCell>
          <TableBodyCell>{account.type}</TableBodyCell>
          <TableBodyCell>{account.ccy}</TableBodyCell>
        </TableBodyRow>
      {/each}
    </TableBody>
  </Table>
</Card>
