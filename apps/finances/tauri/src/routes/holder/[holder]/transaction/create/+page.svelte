<script lang="ts">
  import MovementForm from "$lib/forms/MovementForm/MovementForm.svelte";
  import type { Account } from "$lib/models/Account.js";
  import TransactionForm from "$lib/forms/TransactionForm.svelte";
  import {
    Alert,
    Badge,
    Button,
    Card,
    CardPlaceholder,
    Heading,
    ListPlaceholder,
    Secondary,
    Skeleton,
    TextPlaceholder,
  } from "flowbite-svelte";
  import { PlusOutline, MinusOutline, InfoCircleSolid } from "flowbite-svelte-icons";
  import { NewTransaction, type TransactionGroup } from "$lib/models/TransactionGroup.js";
  import type { MovementType } from "$lib/models/MovementType.js";
  import { NewMovement } from "$lib/forms/MovementForm/NewMovement.svelte.js";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();

  let movements_from: NewMovement[] = $state([]);
  let movements_to: NewMovement[] = $state([]);
  let transation_data: {
    group?: TransactionGroup;
    name?: string;
    description?: string;
    date_value?: Date;
    is_valid: boolean;
  } = $state({
    date_value: new Date(),
    is_valid: false,
  });
  let show_transaction_date = $state(true);
  let show_individual_dates = $derived(!show_transaction_date);

  function accumulate_movements_total(movements: NewMovement[]): number {
    const total_sum_initial = 0;
    return movements.reduce((acc, mov: NewMovement) => {
      let total = mov.total(data.base_ccy);
      return total === undefined ? Number.NEGATIVE_INFINITY : acc + total;
    }, total_sum_initial);
  }

  let total_source = $derived(accumulate_movements_total(movements_from));
  let total_target = $derived(accumulate_movements_total(movements_to));

  function add_movement_from(account?: Account) {
    let new_mov = new NewMovement();
    new_mov.account = account;
    new_mov.date_value = new Date();
    movements_from = movements_from.concat(new_mov);
  }
  function add_movement_to(account?: Account) {
    let new_mov = new NewMovement();
    new_mov.account = account;
    new_mov.date_value = new Date();
    movements_to = movements_to.concat(new_mov);
  }

  if (data.from_account) {
    add_movement_from(data.from_account);
  }

  if (data.to_account) {
    add_movement_to(data.to_account);
  }

  function remove_movement_from(index: number) {
    let pre_list = movements_from.slice(0, index);
    movements_from = pre_list.concat(movements_from.slice(index + 1, movements_from.length));
  }
  function remove_movement_to(index: number) {
    let pre_list = movements_to.slice(0, index);
    movements_to = pre_list.concat(movements_to.slice(index + 1, movements_to.length));
  }

  let all_transaction_groups: TransactionGroup[] = [];

  // Form validation and submit
  let is_valid = $derived(
    transation_data.is_valid && movements_from.every((v) => v.is_valid) && movements_to.every((v) => v.is_valid)
  );
  let submit_disabled = $derived(!is_valid || total_source != total_target);

  function submit() {
    // Create NewTransaction
    // TODO: Errors if some fields are null
    let new_transacion = new NewTransaction(
      transation_data.name!,
      movements_from,
      movements_to,
      transation_data.description,
      transation_data.date_value,
      transation_data.group
    );
    // Send to the backend
    console.log(JSON.stringify(new_transacion));
  }

  let card_error_style = "border-red-600 dark:border-red-600";
</script>

<Heading tag="h1" class="mb-4" customSize="text-3xl font-extrabold  md:text-4xl lg:text-5xl">New transaction</Heading>

<form>
  <div class="mt-px space-y-4">
    <Card size="xl" class="mt-6 {transation_data.is_valid ? '' : card_error_style}">
      <TransactionForm
        bind:transaction_name={transation_data.name}
        bind:transaction_description={transation_data.description}
        bind:transaction_date={transation_data.date_value}
        bind:transaction_group={transation_data.group}
        bind:show_date={show_transaction_date}
        bind:is_valid={transation_data.is_valid}
        {all_transaction_groups}
      />
    </Card>
  </div>

  <div class="mt-4 space-y-4">
    <div class="grid gap-4 grid-cols-2">
      <!-- from movements -->
      <div>
        <Heading tag="h2" class="mb-4" customSize="text-2xl font-extrabold md:text-3xl lg:text-4xl">
          <div class="flex">
            <span> Source accounts </span>
            <button onclick={() => add_movement_from()} class="ml-4">
              <Secondary class="text-xs">Add movement</Secondary>
            </button>
          </div>
        </Heading>

        {#each movements_from as mov, i}
          <Card size="xl" class="mt-6 {mov.is_valid(show_individual_dates, data.base_ccy) ? '' : card_error_style}">
            <MovementForm
              bind:account={mov.account}
              bind:movementtype={mov.mov_type}
              bind:date_value={mov.date_value}
              bind:amount={mov.amount}
              bind:quantity={mov.quantity}
              bind:unit_value={mov.unit_value}
              bind:fx={mov.fx}
              base_ccy={data.base_ccy}
              show_date={show_individual_dates}
              all_accounts={data.all_accounts}
              all_movementtypes={data.all_movementtypes}
            />
            <div class="flex flex-col text-right text-xs mt-2">
              <span class="font-semibold text-primary-500"
                ><button onclick={() => remove_movement_from(i)}>Remove</button></span
              >
            </div>
          </Card>
        {:else}
          <Card size="xl" class="mt-6">
            <TextPlaceholder size="xxxl" class="mt-4" />
          </Card>
        {/each}
      </div>

      <!-- to movements -->
      <div>
        <Heading tag="h2" class="mb-4" customSize="text-2xl font-extrabold  md:text-3xl lg:text-4xl">
          <div class="flex">
            <span> Target accounts </span>
            <button onclick={() => add_movement_to()} class="ml-4">
              <Secondary class="text-xs">Add movement</Secondary>
            </button>
          </div>
        </Heading>
        {#each movements_to as mov, i}
          <Card size="xl" class="mt-6 {mov.is_valid(show_individual_dates, data.base_ccy) ? '' : card_error_style}">
            <MovementForm
              bind:account={mov.account}
              bind:movementtype={mov.mov_type}
              bind:date_value={mov.date_value}
              bind:amount={mov.amount}
              bind:quantity={mov.quantity}
              bind:unit_value={mov.unit_value}
              bind:fx={mov.fx}
              base_ccy={data.base_ccy}
              show_date={show_individual_dates}
              all_accounts={data.all_accounts}
              all_movementtypes={data.all_movementtypes}
            />
            <div class="flex flex-col text-right text-xs mt-2">
              <span class="font-semibold text-primary-500"
                ><button onclick={() => remove_movement_to(i)}>Remove</button></span
              >
            </div>
          </Card>
        {:else}
          <Card size="xl" class="mt-6">
            <TextPlaceholder size="xxxl" class="mt-8" />
          </Card>
        {/each}
      </div>
    </div>
  </div>

  <div class="mt-px space-y-4">
    <Card size="xl" class="mt-6">
      {#if movements_from.length == 0 || movements_to.length == 0}
        <Alert class="mb-6">
          <InfoCircleSolid slot="icon" class="w-5 h-5" />
          <span class="font-medium">Empty movements list!</span>
          Cannot create empty transactions.
        </Alert>
      {/if}
      {#if total_source != total_target}
        <Alert class="mb-6">
          <InfoCircleSolid slot="icon" class="w-5 h-5" />
          <span class="font-medium">Source and target mismatch!</span>
          Source total is EUR {total_source} while target total is EUR {total_target}.
        </Alert>
      {/if}
      <Button onclick={submit} disabled={submit_disabled}>Submit (Total: {total_source} EUR)</Button>
    </Card>
  </div>
</form>
