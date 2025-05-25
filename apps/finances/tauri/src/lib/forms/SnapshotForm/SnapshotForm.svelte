<script lang="ts">
  import { Button, Input, Label, ButtonGroup, InputAddon, Helper } from 'flowbite-svelte';
  import { NewSnapshot } from './NewSnapshot.svelte';
  import { Account } from '../../../../models/src-js';
  import { DateWrapper, sort_date_wrapper } from '../../../../../../../libraries/googleapis/src-js';
  import Datepicker from '../Datepicker.svelte';

  interface Props {
    account: Account;
    method: 'dialog' | 'get' | 'post';
    handleSubmit: (arg0: NewSnapshot) => void;
  }

  let { account, method, handleSubmit }: Props = $props();
  const last_snapshot = account.last_snapshot();

  let date_value = $state<Date>(new Date());
  // let date_value_error: string | undefined = $state(undefined);

  let amount = $state<number | undefined>(last_snapshot ? last_snapshot.amount().amount().amount() : undefined);
  // let amount_error: string | undefined = $state(undefined);

  let quantity = $state<number | undefined>(undefined);
  // let quantity_error: string | undefined = $state(undefined);

  let unit_value = $state<number | undefined>(undefined);
  // let unit_value_error: string | undefined = $state(undefined);

  if (last_snapshot && account.is_numerable()) {
    let numerable = last_snapshot.amount().as_numerable()!;
    quantity = numerable.quantity().as_number();
    unit_value = numerable.unit_value().amount();
  }

  let date_value_error = $derived.by<string | undefined>(() => {
    // Validate date_value
    let _date_value = DateWrapper.create_from_date(date_value);
    if (sort_date_wrapper(_date_value, account.open()) < 0) {
      return 'Cannot take an snapshot before the account was opened.';
    }
    let today = new Date();
    let today_date = DateWrapper.create_from_date(today);
    if (sort_date_wrapper(today_date, _date_value) < 0) {
      return 'Cannot take an snapshot of the future.';
    }
  });

  let amount_error = $derived.by<string | undefined>(() => {
    // Validate amount, quantity and unit_value
    if (!account.is_numerable()) {
      if (amount === undefined || amount < 0) {
        return 'Positive value required.';
      }
    }
  });

  let quantity_error = $derived.by<string | undefined>(() => {
    // Validate amount, quantity and unit_value
    if (account.is_numerable()) {
      if (quantity === undefined || quantity < 0) {
        return 'Positive value required.';
      }
    }
  });

  let unit_value_error = $derived.by<string | undefined>(() => {
    // Validate amount, quantity and unit_value
    if (account.is_numerable()) {
      if (unit_value === undefined || unit_value < 0) {
        return 'Positive value required.';
      }
    }
  });

  function validate_and_submit(e: MouseEvent) {
    e.preventDefault();

    let snapshot = new NewSnapshot(account, amount, quantity, unit_value);
    handleSubmit(snapshot);
  }
</script>

<form class="flex flex-col space-y-6" {method} action="#">
  <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Add snapshot for {account.name()}</h3>

  <Label class="space-y-2">
    <!-- <Datepicker required={true} inline={true} bind:value={date_value} /> -->
    <Datepicker required={true} inline={true} bind:value={date_value} />
    {#if date_value_error}
      <Helper class="mt-2" color="red"><span class="font-medium">Error!</span> {date_value_error}</Helper>
    {/if}
  </Label>
  {#if !account.is_numerable()}
    <Label class="space-y-2">
      <span>Amount</span>
      <ButtonGroup class="w-full">
        <InputAddon>{account.ccy()}</InputAddon>
        <Input type="number" required placeholder="amount" bind:value={amount} />
      </ButtonGroup>
      {#if amount_error}
        <Helper class="mt-2" color="red"><span class="font-medium">Error!</span> {amount_error}</Helper>
      {/if}
    </Label>
  {:else}
    <div class="flex items-start w-full">
      <Label>
        <span>Quantity</span>
        <Input type="number" required placeholder="quantity" bind:value={quantity} />
        {#if quantity_error}
          <Helper class="mt-2" color="red"><span class="font-medium">Error!</span> {quantity_error}</Helper>
        {/if}
      </Label>
      <Label class="ml-4">
        <span>Unit value</span>
        <ButtonGroup>
          <InputAddon>{account.ccy()}</InputAddon>
          <Input type="number" required placeholder="unit_value" bind:value={unit_value} />
        </ButtonGroup>
        {#if unit_value_error}
          <Helper class="mt-2" color="red"><span class="font-medium">Error!</span> {unit_value_error}</Helper>
        {/if}
      </Label>
    </div>
  {/if}
  <Button
    disabled={date_value_error || amount_error || quantity_error || unit_value_error}
    onclick={validate_and_submit}
    type="submit"
    class="w-full, mt-4">Submit</Button
  >
</form>
