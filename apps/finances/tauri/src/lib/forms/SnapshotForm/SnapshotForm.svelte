<script lang="ts">
  import { Input, Label, ButtonGroup, InputAddon, Helper, Alert } from "flowbite-svelte";
  import Datepicker from "../Datepicker.svelte";
  import type { NewSnapshot } from "./NewSnapshot.svelte";

  let {
    snapshot = $bindable(),
  }: {
    snapshot: NewSnapshot;
  } = $props();
</script>

<div class="flex flex-col space-y-6">
  <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Add snapshot for {snapshot.account.name()}</h3>

  <Label class="space-y-2">
    <!-- <span>Date value: {snapshot.date_value.toLocaleDateString()}</span> -->
    <Datepicker required inline bind:value={snapshot.date_value} />
    {#if snapshot.error_date_value}
      <Helper class="mt-2" color="red"><span class="font-medium">Error!</span> {snapshot.error_date_value}</Helper>
    {/if}
  </Label>
  {#if !snapshot.account.is_numerable()}
    <Label class="space-y-2">
      <span>Amount</span>
      <ButtonGroup class="w-full">
        <InputAddon>{snapshot.account.ccy()}</InputAddon>
        <Input type="number" required placeholder="amount" bind:value={snapshot.amount} />
      </ButtonGroup>
      {#if snapshot.error_amount}
        <Helper class="mt-2" color="red"><span class="font-medium">Error!</span> {snapshot.error_amount}</Helper>
      {/if}
    </Label>
  {:else}
    <div class="flex items-start w-full">
      <Label>
        <span>Quantity</span>
        <Input type="number" required placeholder="quantity" bind:value={snapshot.quantity} />
        {#if snapshot.error_quantity}
          <Helper class="mt-2" color="red"><span class="font-medium">Error!</span> {snapshot.error_quantity}</Helper>
        {/if}
      </Label>
      <Label class="ml-4">
        <span>Unit value</span>
        <ButtonGroup>
          <InputAddon>{snapshot.account.ccy()}</InputAddon>
          <Input type="number" required placeholder="unit_value" bind:value={snapshot.unit_value} />
        </ButtonGroup>
        {#if snapshot.error_unit_value}
          <Helper class="mt-2" color="red"><span class="font-medium">Error!</span> {snapshot.error_unit_value}</Helper>
        {/if}
      </Label>
    </div>
  {/if}
</div>
