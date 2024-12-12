<script lang="ts">
  import type { Account } from "$lib/models/Account";
  import { Input, Label, ButtonGroup, InputAddon } from "flowbite-svelte";
  import { MovementType } from "$lib/models/MovementType";
  import AccountDropdown from "../AccountDropdown/AccountDropdown.svelte";
  import MovementTypeDropdown from "../MovementTypeDropdown/MovementTypeDropdown.svelte";
  import type { NewMovement } from "./NewMovement.svelte";
  import Datepicker from "../Datepicker.svelte";

  let {
    new_movement = $bindable(),
    base_ccy,
    show_date,
    all_accounts,
    all_movementtypes,
  }: {
    new_movement: NewMovement;
    base_ccy: string;
    show_date: boolean;
    all_accounts: Account[];
    all_movementtypes: MovementType[];
  } = $props();

  function ccy_symbol(ccy: string): string {
    if (ccy === "EUR") {
      return "€";
    } else if (ccy === "USD") {
      return "$";
    } else {
      return ccy;
    }
  }
</script>

<div class="flex flex-col space-y-6" action="#">
  <AccountDropdown bind:account={new_movement.account} {all_accounts} />
  <MovementTypeDropdown bind:movementtype={new_movement.mov_type} {all_movementtypes} />

  {#if show_date}
    <Label class="space-y-2">
      <span>Date value</span>
      <Datepicker required bind:value={new_movement.date_value} />
    </Label>
  {/if}
  {#if new_movement.account}
    <div class="flex items-center w-full">
      {#if new_movement.account.is_numerable}
        <Label>
          <span>Quantity</span>
          <Input type="number" required placeholder="quantity" bind:value={new_movement.quantity} />
        </Label>
        <Label class="ml-4">
          <span>Unit value</span>
          <ButtonGroup>
            <InputAddon>{ccy_symbol(new_movement.account.ccy)}</InputAddon>
            <Input type="number" required placeholder="unit_value" bind:value={new_movement.unit_value} />
          </ButtonGroup>
        </Label>
      {:else}
        <Label>
          <span>Amount</span>
          <ButtonGroup class="w-full">
            <InputAddon>{ccy_symbol(new_movement.account.ccy)}</InputAddon>
            <Input type="number" required placeholder="amount" bind:value={new_movement.amount} />
          </ButtonGroup>
        </Label>
      {/if}
      {#if new_movement.account.ccy != base_ccy}
        <Label class="ml-4">
          <span>FX</span>
          <ButtonGroup>
            <InputAddon>{ccy_symbol(base_ccy)}/{ccy_symbol(new_movement.account.ccy)}</InputAddon>
            <Input type="number" required placeholder="fx" bind:value={new_movement.fx} />
          </ButtonGroup>
        </Label>
      {/if}
      <Label class="ml-4">
        <span>Total</span>
        <ButtonGroup>
          <InputAddon>{ccy_symbol(base_ccy)}</InputAddon>
          <Input disabled type="number" required value={new_movement.total(base_ccy)} />
        </ButtonGroup>
      </Label>
    </div>
  {/if}
</div>
