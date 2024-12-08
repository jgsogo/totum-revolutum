<script lang="ts">
  import type { Account } from "$lib/models/Account";
  import { NewMovement } from "$lib/models/Movement";
  import type { Snapshot } from "$lib/models/Snapshot";
  import { Button, Input, Label, ButtonGroup, InputAddon, Datepicker, Helper, Alert, Select } from "flowbite-svelte";
  import { InfoCircleSolid } from "flowbite-svelte-icons";
  import { MovementType } from "$lib/models/MovementType";

  let {
    account = $bindable(),
    movement_type = $bindable(),
    date_value = $bindable(),
    amount = $bindable(),
    quantity = $bindable(),
    unit_value = $bindable(),
    fx = $bindable(),
    base_ccy,
    show_date,
    all_accounts,
    all_movementtypes,
  }: {
    account: Account | undefined;
    movement_type: MovementType | undefined;
    date_value: Date | undefined;
    amount: number | undefined;
    quantity: number | undefined;
    unit_value: number | undefined;
    fx: number | undefined;
    base_ccy: string;
    show_date: boolean;
    all_accounts: { value: Account; name: string }[];
    all_movementtypes: { value: MovementType; name: string }[];
  } = $props();

  let total_amount = $derived.by(() => {
    let total = 0;

    if (account?.is_numerable) {
      total = (quantity ? quantity : 0) * (unit_value ? unit_value : 0);
    } else {
      total = amount ? amount : 0;
    }

    // apply FX
    if (fx) {
      total = total / fx;
    }

    return total;
  });

  let dateFormat: Intl.DateTimeFormatOptions = {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  };
</script>

<form class="flex flex-col space-y-6" action="#">
  <Label class="space-y-2">
    <span>Account</span>
    <!-- FIXME: Why initial 'account' is not working here? -->
    <Select class="mt-2" items={all_accounts} bind:value={account} />
  </Label>
  <Label class="space-y-2">
    <span>Type</span>
    <Select class="mt-2" items={all_movementtypes} bind:value={movement_type} />
  </Label>
  {#if show_date}
    <Label class="space-y-2">
      <span>Date value</span>
      <Datepicker required bind:value={date_value} {dateFormat} />
    </Label>
  {/if}
  {#if account}
    <div class="flex items-center w-full">
      {#if account.is_numerable}
        <Label>
          <span>Quantity</span>
          <Input type="number" required placeholder="quantity" bind:value={quantity} />
        </Label>
        <Label class="ml-4">
          <span>Unit value</span>
          <ButtonGroup>
            <InputAddon>{account.ccy}</InputAddon>
            <Input type="number" required placeholder="unit_value" bind:value={unit_value} />
          </ButtonGroup>
        </Label>
      {:else}
        <Label>
          <span>Amount</span>
          <ButtonGroup class="w-full">
            <InputAddon>{account.ccy}</InputAddon>
            <Input type="number" required placeholder="1234,56" bind:value={amount} />
          </ButtonGroup>
        </Label>
      {/if}
      {#if account.ccy != base_ccy}
        <Label class="ml-4">
          <span>FX</span>
          <ButtonGroup>
            <InputAddon>{base_ccy}/{account.ccy}</InputAddon>
            <Input type="number" required placeholder="fx" bind:value={fx} />
          </ButtonGroup>
        </Label>
      {/if}
      <Label class="ml-4">
        <span>Total</span>
        <ButtonGroup>
          <InputAddon>{base_ccy}</InputAddon>
          <Input disabled type="number" required value={total_amount} />
        </ButtonGroup>
      </Label>
    </div>
  {/if}
</form>
