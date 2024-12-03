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
    show_date,
    all_accounts_list,
    all_movement_types,
  }: {
    account: Account | undefined;
    movement_type: MovementType | undefined;
    date_value: Date | undefined;
    amount: number | undefined;
    quantity: number | undefined;
    unit_value: number | undefined;
    show_date: boolean;
    all_accounts_list: Account[];
    all_movement_types: MovementType[];
  } = $props();

  let total_amount = $derived((quantity ? quantity : 0) * (unit_value ? unit_value : 0));

  let accounts = all_accounts_list.map((value) => {
    return { value: value, name: value.name };
  });

  let movement_types = all_movement_types.map((value) => {
    return { value: value, name: value.breadcrumb || value.name };
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
    <Select class="mt-2" items={accounts} bind:value={account} />
  </Label>
  <Label class="space-y-2">
    <span>Type</span>
    <Select class="mt-2" items={movement_types} bind:value={movement_type} />
  </Label>
  {#if show_date}
    <Label class="space-y-2">
      <span>Date value</span>
      <Datepicker required bind:value={date_value} {dateFormat} />
    </Label>
  {/if}

  {#if account}
    {#if account.is_numerable}
      <div class="flex items-center w-full">
        <Label>
          <span>Quantity</span>
          <Input type="number" required placeholder="12" bind:value={quantity} />
        </Label>
        <Label class="ml-4">
          <span>Unit value</span>
          <ButtonGroup>
            <InputAddon>{account.ccy}</InputAddon>
            <Input type="number" required placeholder="1234,56" bind:value={unit_value} />
          </ButtonGroup>
        </Label>
        <Label class="ml-4">
          <span>Total amount</span>
          <ButtonGroup>
            <InputAddon>{account.ccy}</InputAddon>
            <Input disabled type="number" required value={total_amount} />
          </ButtonGroup>
        </Label>
      </div>
    {:else}
      <div class="flex items-center w-full">
        <Label>
          <span>Amount</span>
          <ButtonGroup class="w-full">
            <InputAddon>{account.ccy}</InputAddon>
            <Input type="number" required placeholder="1234,56" bind:value={amount} />
          </ButtonGroup>
        </Label>
      </div>
    {/if}
  {/if}
</form>
