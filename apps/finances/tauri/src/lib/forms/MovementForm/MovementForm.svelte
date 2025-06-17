<script lang="ts">
  import { Input, Label, ButtonGroup, InputAddon, Radio, Helper } from 'flowbite-svelte';
  import { NewMovementType, type NewMovement } from './NewMovement.svelte';
  import Datepicker from '../Datepicker.svelte';
  import { Account, Snapshot, MovementType } from '../../../../models/src-js';
  import { ccy_symbol, CurrencyCode, DateWrapper, sort_date_wrapper } from '../../../../../../../libraries/googleapis/src-js';
  import { get_account_snapshots } from '$lib/commands';
  import DropdownWithSearch from '../DropdownWithSearch.svelte';

  let {
    new_movement = $bindable(),
    base_ccy,
    show_date,
    all_accounts,
    all_movementtypes,
  }: {
    new_movement: NewMovement;
    base_ccy: CurrencyCode;
    show_date: boolean;
    all_accounts: Account[];
    all_movementtypes: MovementType[];
  } = $props();

  async function handleDividendDateSnapshot() {
    console.log(`handleDividendDateSnapshot!!!`);
    if (!new_movement.ex_dividend_date) {
      console.log(` - no ex_dividend_date, so ex_dividend_snapshot is undefined`);
      new_movement.ex_dividend_snapshot = undefined;
    } else {
      // Get the closest (equal or before) snapshot to the given date
      let ex_dividend_date = DateWrapper.create_from_date(new_movement.ex_dividend_date);
      console.log(` - get all snapshots for account ${new_movement.account} until date ${ex_dividend_date}`);
      let snapshots: Snapshot[] = await get_account_snapshots(new_movement.account!, undefined, ex_dividend_date);
      console.log(` - found ${snapshots.length} snapshots`);
      let snapshot = snapshots?.find((s: Snapshot) => {
        return sort_date_wrapper(s.date_value(), ex_dividend_date) <= 0;
      });
      console.log(' - found snapshot: ', snapshot);
      new_movement.ex_dividend_snapshot = snapshot;
    }
  }
  const unique_id = '_' + Math.random().toString(36).slice(2, 9);

  let filtered_accounts = $derived.by(() => {
    switch (new_movement.type) {
      case NewMovementType.NonNumerable:
        return all_accounts.filter((v) => !v.is_numerable());
      case NewMovementType.Numerable:
      case NewMovementType.Dividend:
        return all_accounts.filter((v) => v.is_numerable());
    }
  });
</script>

<div class="flex flex-col space-y-6" action="#">
  <!-- Radio button to choose the movement type -->
  <ul
    class="items-center w-full rounded-lg border border-gray-200 sm:flex dark:bg-gray-800 dark:border-gray-600 divide-x rtl:divide-x-reverse divide-gray-200 dark:divide-gray-600"
  >
    {#each Object.values(NewMovementType) as value}
      <li class="w-full">
        <Radio bind:group={new_movement.type} {value} name={unique_id} class="p-3">{value} | {new_movement.type === value}</Radio>
      </li>
    {/each}
  </ul>

  <DropdownWithSearch
    bind:value={new_movement.account}
    bind:items={filtered_accounts}
    searchEl="Account"
    labelForItem={(v: Account | undefined) => `${v?.custodian().name()} | ${v?.name()}`}
  />
  <!-- TODO: Filter 'all_movementtypes' to use only the ones allowed for the selected account -->
  <DropdownWithSearch
    bind:value={new_movement.mov_type}
    bind:items={all_movementtypes}
    searchEl="Mov type"
    labelForItem={(v: MovementType | undefined) => v?.breadcrumb()!.toString()}
  />

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
          <Datepicker required bind:value={new_movement.ex_dividend_date} onselect={handleDividendDateSnapshot} />
          <Helper
            >snapshot @ {new_movement.ex_dividend_snapshot?.date_value().toString()} ({new_movement.ex_dividend_snapshot
              ?.amount()
              .as_numerable()
              ?.quantity()
              .as_number()} ud.)</Helper
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
      <span class="font-semibold">Total: {new_movement.total(base_ccy)}</span>
    </div>
  {/if}
</div>
