<script lang="ts">
  import { Input, Label, ButtonGroup, InputAddon, Radio, Helper } from "flowbite-svelte";
  import AccountDropdown from "../AccountDropdown/AccountDropdown.svelte";
  import MovementTypeDropdown from "../MovementTypeDropdown/MovementTypeDropdown.svelte";
  import { NewMovementType, type NewMovement } from "./NewMovement.svelte";
  import Datepicker from "../Datepicker.svelte";
  import { Account, Snapshot, type MovementType } from "../../../../models/src-js";

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

  const movement_types = Object.keys(NewMovementType)
    .filter((v) => isNaN(Number(v)))
    .map((key) => {
      return { label: key, value: NewMovementType[key] };
    });

  async function handleDividendDateSnapshot() {
    if (!new_movement.ex_dividend_date) {
      new_movement.ex_dividend_snapshot = undefined;
    } else {
      // Get the closest (equal or before) snapshot to the given date
      let snapshots = await new_movement.account?.snapshots();
      let snapshot = snapshots?.find((s: Snapshot) => {
        return s.date_value.getDate() <= new_movement.ex_dividend_date!.getDate();
      });
      console.log("Found snapshot: ", snapshot);
      new_movement.ex_dividend_snapshot = snapshot;
    }
  }

  let total_str = $derived.by(() => {
    let _ = new_movement.ex_dividend_snapshot;

    let total = new_movement.total(base_ccy);
    let symbol = ccy_symbol(base_ccy);
    if (total === undefined) {
      return `- ${symbol}`;
    }
    return `${total} ${symbol}`;
  });
</script>

<div class="flex flex-col space-y-6" action="#">
  <!-- Radio button to choose the movement type -->
  <ul
    class="items-center w-full rounded-lg border border-gray-200 sm:flex dark:bg-gray-800 dark:border-gray-600 divide-x rtl:divide-x-reverse divide-gray-200 dark:divide-gray-600"
  >
    {#each movement_types as { label, value }, i}
      <li class="w-full"><Radio bind:group={new_movement.type} {value} name="hor-list" class="p-3">{label}</Radio></li>
    {/each}
  </ul>

  <!-- Common fields -->
  <AccountDropdown bind:account={new_movement.account} {all_accounts} on:change={handleDividendDateSnapshot} />
  <MovementTypeDropdown bind:movementtype={new_movement.mov_type} {all_movementtypes} />
  {#if show_date}
    <Label class="space-y-2">
      <span>Date value</span>
      <Datepicker required bind:value={new_movement.date_value} />
    </Label>
  {/if}

  {#if new_movement.account}
    <div class="flex items-center w-full">
      <!-- Different form fields depending on the type of movement we are creating -->
      {#if new_movement.type === NewMovementType.NonNumerable}
        <Label class="flex flex-col">
          <span>Amount</span>
          <ButtonGroup class="w-full">
            <InputAddon>{ccy_symbol(new_movement.account.ccy)}</InputAddon>
            <Input type="number" required placeholder="amount" bind:value={new_movement.amount} />
          </ButtonGroup>
        </Label>
      {:else if new_movement.type === NewMovementType.Numerable}
        <Label class="flex flex-col">
          <span>Quantity</span>
          <Input type="number" required placeholder="quantity" bind:value={new_movement.quantity} />
        </Label>
        <Label class="ml-4 flex flex-col">
          <span>Unit value</span>
          <ButtonGroup>
            <InputAddon>{ccy_symbol(new_movement.account.ccy)}</InputAddon>
            <Input type="number" required placeholder="unit_value" bind:value={new_movement.unit_value} />
          </ButtonGroup>
        </Label>
      {:else if new_movement.type === NewMovementType.Dividend}
        <Label class="flex flex-col">
          <span>Ex dividend date</span>
          <Datepicker required bind:value={new_movement.ex_dividend_date} on:select={handleDividendDateSnapshot} />
          <Helper
            >snapshot @ {new_movement.ex_dividend_snapshot?.date_value.toISOString().slice(0, 10)} ({new_movement
              .ex_dividend_snapshot?.quantity} ud.)</Helper
          >
        </Label>
        <Label class="ml-4 flex flex-col">
          <span>Unit value</span>
          <ButtonGroup>
            <InputAddon>{ccy_symbol(new_movement.account.ccy)}</InputAddon>
            <Input type="number" required placeholder="unit_value" bind:value={new_movement.unit_value} />
          </ButtonGroup>
        </Label>
      {:else}
        Invalid movement type {new_movement.account}
      {/if}

      {#if new_movement.account.ccy != base_ccy}
        <Label class="ml-4 flex flex-col">
          <span>FX</span>
          <ButtonGroup>
            <InputAddon>{ccy_symbol(base_ccy)}/{ccy_symbol(new_movement.account.ccy)}</InputAddon>
            <Input type="number" required placeholder="fx" bind:value={new_movement.fx} />
          </ButtonGroup>
        </Label>
      {/if}
    </div>

    <div class="flex flex-col text-left text-xs mt-2">
      <span class="font-semibold">Total: {total_str}</span>
    </div>
  {/if}
</div>
