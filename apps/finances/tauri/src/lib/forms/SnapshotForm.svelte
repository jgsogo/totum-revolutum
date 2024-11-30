<script lang="ts">
  import type { Account } from "$lib/models/Account";
  import type { Snapshot } from "$lib/models/Snapshot";
  import { Button, Input, Label, ButtonGroup, InputAddon, Datepicker } from "flowbite-svelte";

  let {
    account,
    last_snapshot,
    on_snapshot,
  }: {
    account: Account,
    last_snapshot?: Snapshot,
    on_snapshot: (date_value: Date, amount?: number, quantity?: number, unit_value?: number) => void | string;
  } = $props();

  let date_value: Date = $state(new Date());
  let amount: number | undefined = $state(last_snapshot?.amount);
  let quantity: number | undefined = $state(last_snapshot?.quantity);
  let unit_value: number | undefined = $state(last_snapshot?.unit_value);

  const submitSnapshot = () => {
    // TODO: Validation:
    //  * Date lower or equal than today
    //  * Amount makes sense
    if (amount) {
      let err = on_snapshot(date_value, amount, quantity, unit_value);
      // TODO: Manage error
    }
  };
</script>

<form class="flex flex-col space-y-6" action="#">
  <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Add snapshot</h3>
  <Label class="space-y-2">
    <span>Date value: {date_value.toLocaleDateString()}</span>
    <Datepicker required inline bind:value={date_value} />
  </Label>
  {#if !account.is_numerable}
    <Label class="space-y-2">
      <span>Amount</span>
      <ButtonGroup class="w-full">
        <InputAddon>{account.ccy}</InputAddon>
        <Input required placeholder="1234,56" bind:value={amount} />
      </ButtonGroup>
    </Label>
  {:else}
    <div class="flex items-start w-full">
      <Label>
        <span>Quantity</span>
        <Input required placeholder="12" bind:value={quantity} />
      </Label>
      <Label class="ml-4">
        <span>Unit value</span>
        <ButtonGroup >
          <InputAddon>{account.ccy}</InputAddon>
          <Input required placeholder="1234,56" bind:value={unit_value} />
        </ButtonGroup>
      </Label>
    </div>
  {/if}
  <Button type="submit" onclick={submitSnapshot} class="w-full1">Submit</Button>
</form>
