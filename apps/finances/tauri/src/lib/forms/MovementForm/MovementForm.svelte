<script lang="ts">
  import { Input, Label, ButtonGroup, InputAddon, Radio, Helper } from "flowbite-svelte";
  import AccountDropdown from "../AccountDropdown/AccountDropdown.svelte";
  import MovementTypeDropdown from "../MovementTypeDropdown/MovementTypeDropdown.svelte";
  import { NewMovementType, type NewMovement } from "./NewMovement.svelte";
  import Datepicker from "../Datepicker.svelte";
  import { Account, Snapshot, MovementType } from "../../../../models/src-js";
  import { DateWrapper, sort_date_wrapper } from "../../../../../../../libraries/googleapis/src-js/date";

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

  // const movement_types = Object.keys(NewMovementType).map((key) => {
  //   return { label: key, value: key as NewMovementType };
  // });

  async function handleDividendDateSnapshot() {
    if (!new_movement.ex_dividend_date) {
      new_movement.ex_dividend_snapshot = undefined;
    } else {
      // Get the closest (equal or before) snapshot to the given date
      let snapshots: Snapshot[] = []; // FIXME: Retrieve the snapshosts for this account
      let ex_dividend_date = DateWrapper.create_from_yyyy_mm_dd(
        new_movement.ex_dividend_date.getFullYear(),
        new_movement.ex_dividend_date.getMonth() + 1,
        new_movement.ex_dividend_date.getDate()
      );
      let snapshot = snapshots?.find((s: Snapshot) => {
        return sort_date_wrapper(s.date_value(), ex_dividend_date) <= 0;
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

  // let new_movement_type = $state(movement_types.find((it) => it.label === new_movement.type)?.value)!;
  // $effect(() => {
  //   new_movement.type = new_movement_type;
  // });
  // let selected = $state();

  const unique_id = "_" + Math.random().toString(36).slice(2, 9);
</script>

<div class="flex flex-col space-y-6" action="#">
  <!-- Radio button to choose the movement type -->
  {new_movement.type?.toString()} -
  <ul
    class="items-center w-full rounded-lg border border-gray-200 sm:flex dark:bg-gray-800 dark:border-gray-600 divide-x rtl:divide-x-reverse divide-gray-200 dark:divide-gray-600"
  >
    {#each Object.values(NewMovementType) as value}
      <li class="w-full">
        <Radio bind:group={new_movement.type} {value} name={unique_id} class="p-3">{value} | {new_movement.type === value}</Radio
        >
      </li>
    {/each}
  </ul>

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
            <InputAddon>{ccy_symbol(new_movement.account.ccy())}</InputAddon>
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
            <InputAddon>{ccy_symbol(new_movement.account.ccy())}</InputAddon>
            <Input type="number" required placeholder="unit_value" bind:value={new_movement.unit_value} />
          </ButtonGroup>
        </Label>
      {:else if new_movement.type === NewMovementType.Dividend}
        <Label class="flex flex-col">
          <span>Ex dividend date</span>
          <Datepicker required bind:value={new_movement.ex_dividend_date} on:select={handleDividendDateSnapshot} />
          <Helper
            >snapshot @ {new_movement.ex_dividend_snapshot?.date_value().toString()} ({new_movement.ex_dividend_snapshot
              ?.amount()
              .as_numerable()
              ?.quantity()} ud.)</Helper
          >
        </Label>
        <Label class="ml-4 flex flex-col">
          <span>Unit value</span>
          <ButtonGroup>
            <InputAddon>{ccy_symbol(new_movement.account.ccy())}</InputAddon>
            <Input type="number" required placeholder="unit_value" bind:value={new_movement.unit_value} />
          </ButtonGroup>
        </Label>
      {:else}
        Invalid movement type {new_movement.account}
      {/if}

      {#if new_movement.account.ccy() != base_ccy}
        <Label class="ml-4 flex flex-col">
          <span>FX</span>
          <ButtonGroup>
            <InputAddon>{ccy_symbol(base_ccy)}/{ccy_symbol(new_movement.account.ccy())}</InputAddon>
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
