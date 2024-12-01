<script lang="ts">
  import type { Account } from "$lib/models/Account";
  import type { Snapshot } from "$lib/models/Snapshot";
  import { Button, Input, Label, ButtonGroup, InputAddon, Datepicker, Helper, Alert } from "flowbite-svelte";
  import { InfoCircleSolid } from "flowbite-svelte-icons";

  let {
    account,
    last_snapshot,
    on_snapshot,
  }: {
    account: Account;
    last_snapshot?: Snapshot;
    on_snapshot: (date_value: Date, amount?: number, quantity?: number, unit_value?: number) => Promise<void>;
  } = $props();

  let date_value: Date = $state(new Date());
  let amount: number | undefined = $state(last_snapshot?.amount);
  let quantity: number | undefined = $state(last_snapshot?.quantity);
  let unit_value: number | undefined = $state(last_snapshot?.unit_value);
  let backend_error: string | null = $state(null);
  let date_error: string | undefined = $state();
  let amount_error: string | undefined = $state();
  let quantity_error: string | undefined = $state();
  let unit_value_error: string | undefined = $state();

  let today = new Date();

  const submitSnapshot = async () => {
    // Validation
    backend_error = null;
    date_error = amount_error = quantity_error = unit_value_error = undefined;

    // Date has to be equal or lower than today
    if (date_value > today) {
      date_error = "Cannot take an snapshot of the future.";
    }
    if (date_value < account.open) {
      date_error = "Cannot take an snapshot before the account was opened.";
    }

    if (account.is_numerable) {
      amount = undefined;
      // Quantity needs to be a positive integer
      if (!quantity || !Number.isInteger(quantity) || quantity < 0) {
        quantity_error = "Positive integer required.";
      }

      // Unit value needs to be a positive flaot
      if (!unit_value || unit_value < 0) {
        unit_value_error = "Positive value required.";
      }
    } else {
      quantity = unit_value = undefined;
      // Amount needs to be a positive float
      if (!amount || amount < 0) {
        amount_error = "Positive value required.";
      }
    }

    if (date_error || quantity_error || unit_value_error || amount_error) {
      return;
    }

    try {
        await on_snapshot(date_value, amount, quantity, unit_value);
    }
    catch (error) {
        backend_error = String(error);
    }

  };
</script>

<form class="flex flex-col space-y-6" action="#">
  <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Add snapshot</h3>

  <Label class="space-y-2">
    <span>Date value: {date_value.toLocaleDateString()}</span>
    <Datepicker required inline bind:value={date_value} />
    {#if date_error}
      <Helper class="mt-2" color="red"><span class="font-medium">Error!</span> {date_error}</Helper>
    {/if}
  </Label>
  {#if !account.is_numerable}
    <Label class="space-y-2">
      <span>Amount</span>
      <ButtonGroup class="w-full">
        <InputAddon>{account.ccy}</InputAddon>
        <Input type="number" required placeholder="1234,56" bind:value={amount} />
      </ButtonGroup>
      {#if amount_error}
        <Helper class="mt-2" color="red"><span class="font-medium">Error!</span> {amount_error}</Helper>
      {/if}
    </Label>
  {:else}
    <div class="flex items-start w-full">
      <Label>
        <span>Quantity</span>
        <Input type="number" required placeholder="12" bind:value={quantity} />
        {#if quantity_error}
          <Helper class="mt-2" color="red"><span class="font-medium">Error!</span> {quantity_error}</Helper>
        {/if}
      </Label>
      <Label class="ml-4">
        <span>Unit value</span>
        <ButtonGroup>
          <InputAddon>{account.ccy}</InputAddon>
          <Input type="number" required placeholder="1234,56" bind:value={unit_value} />
        </ButtonGroup>
        {#if unit_value_error}
          <Helper class="mt-2" color="red"><span class="font-medium">Error!</span> {unit_value_error}</Helper>
        {/if}
      </Label>
    </div>
  {/if}

  {#if backend_error}
    <Alert border>
      <InfoCircleSolid slot="icon" class="w-5 h-5" />
      <span class="font-medium">Error!</span>
      {backend_error}
    </Alert>
  {/if}

  <Button type="submit" onclick={submitSnapshot} class="w-full1">Submit</Button>
</form>
