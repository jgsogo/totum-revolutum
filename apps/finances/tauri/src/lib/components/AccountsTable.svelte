<script lang="ts">
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    Card,
    Heading,
  } from "flowbite-svelte";
  import type { Account } from "../../../models/src-js";
  import { goToAccountDetail } from "$lib/utils";

  let {
    accounts,
    show_custodian = true,
    show_holders = true,
    show_category = true,
  }: { accounts: Account[]; show_custodian?: boolean; show_holders?: boolean; show_category?: boolean } = $props();
</script>

<Card size="xl" class="shadow-sm max-w-none">
  <div class="items-center justify-between lg:flex">
    <div class="mb-4 mt-px lg:mb-0">
      <Heading tag="h3" class="-ml-0.25 mb-2 text-xl font-semibold dark:text-white">Accounts</Heading>
      {#if accounts.length == 0}
        <span class="text-base font-normal text-gray-500 dark:text-gray-400">No accounts</span>
      {/if}
    </div>
  </div>
  {#if accounts.length > 0}
    <Table hoverable={true} noborder striped class="mt-6 min-w-full divide-y divide-gray-200 dark:divide-gray-600">
      <TableHead class="bg-gray-50 dark:bg-gray-700">
        {#if show_custodian}
          <TableHeadCell>Custodian</TableHeadCell>
        {/if}
        {#if show_holders}
          <TableHeadCell>Holders</TableHeadCell>
        {/if}
        <TableHeadCell>Name</TableHeadCell>
        {#if show_category}
          <TableHeadCell>Category</TableHeadCell>
        {/if}
        <TableHeadCell>Type</TableHeadCell>
        <TableHeadCell>Snapshot</TableHeadCell>
      </TableHead>
      <TableBody tableBodyClass="divide-y">
        {#each accounts as account}
          <TableBodyRow onclick={() => goToAccountDetail(account)}>
            {#if show_custodian}
              <TableBodyCell>{account.custodian().name()}</TableBodyCell>
            {/if}
            {#if show_holders}
              <TableBodyCell
                >{account
                  .holders()
                  .map((v) => v.name())
                  .join(", ")}</TableBodyCell
              >
            {/if}
            <TableBodyCell>{account.name()}</TableBodyCell>
            {#if show_category}
              <TableBodyCell>{account.type().category()}</TableBodyCell>
            {/if}
            <TableBodyCell>{account.type().name()}</TableBodyCell>
            <TableBodyCell>{account.last_snapshot() ? account.last_snapshot()!.amount().amount() : "-"}</TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  {/if}
</Card>
