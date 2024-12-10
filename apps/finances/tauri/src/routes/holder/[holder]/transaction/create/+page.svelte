<script lang="ts">
  import MovementForm from "$lib/forms/MovementForm/MovementForm.svelte";
  import type { Account } from "$lib/models/Account.js";
  import TransactionForm from "$lib/forms/TransactionForm/TransactionForm.svelte";
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
  import { type TransactionGroup } from "$lib/models/TransactionGroup.js";
  import type { MovementType } from "$lib/models/MovementType.js";
  import { NewMovement } from "$lib/forms/MovementForm/NewMovement.svelte.js";
  import { NewTransaction } from "$lib/forms/TransactionForm/NewTransaction.svelte.js";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();

  let transaction: NewTransaction = $state(new NewTransaction());
  transaction.date_value = new Date();
  let show_transaction_date = $state(true);
  let show_individual_dates = $derived(!show_transaction_date);

  function add_movement_from(account?: Account) {
    let new_mov = new NewMovement();
    new_mov.account = account;
    new_mov.date_value = new Date();
    transaction.movements_from = transaction.movements_from.concat(new_mov);
  }
  function add_movement_to(account?: Account) {
    let new_mov = new NewMovement();
    new_mov.account = account;
    new_mov.date_value = new Date();
    transaction.movements_to = transaction.movements_to.concat(new_mov);
  }

  if (data.from_account) {
    add_movement_from(data.from_account);
  }

  if (data.to_account) {
    add_movement_to(data.to_account);
  }

  function remove_movement_from(index: number) {
    let pre_list = transaction.movements_from.slice(0, index);
    transaction.movements_from = pre_list.concat(
      transaction.movements_from.slice(index + 1, transaction.movements_from.length)
    );
  }
  function remove_movement_to(index: number) {
    let pre_list = transaction.movements_to.slice(0, index);
    transaction.movements_to = pre_list.concat(
      transaction.movements_to.slice(index + 1, transaction.movements_to.length)
    );
  }

  let all_transaction_groups: TransactionGroup[] = [];

  // Form validation and submit
  let submit_disabled = $derived(!transaction.is_valid(show_transaction_date, data.base_ccy));

  function submit() {
    // Send to the backend
    console.log(JSON.stringify(transaction));
  }

  let card_error_style = "border-red-600 dark:border-red-600";
</script>

<Heading tag="h1" class="mb-4" customSize="text-3xl font-extrabold  md:text-4xl lg:text-5xl">New transaction</Heading>

<form>
  <div class="mt-px space-y-4">
    <Card size="xl" class="mt-6 {transaction.is_valid(show_transaction_date, data.base_ccy) ? '' : card_error_style}">
      <TransactionForm bind:transaction bind:show_date={show_transaction_date} {all_transaction_groups} />
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

        {#each transaction.movements_from as mov, i}
          <Card size="xl" class="mt-6 {mov.is_valid(show_individual_dates, data.base_ccy) ? '' : card_error_style}">
            <MovementForm
              bind:new_movement={transaction.movements_from[i]}
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
        {#each transaction.movements_to as mov, i}
          <Card size="xl" class="mt-6 {mov.is_valid(show_individual_dates, data.base_ccy) ? '' : card_error_style}">
            <MovementForm
              bind:new_movement={transaction.movements_to[i]}
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
      {#if transaction.movements_from.length == 0 || transaction.movements_to.length == 0}
        <Alert class="mb-6">
          <InfoCircleSolid slot="icon" class="w-5 h-5" />
          <span class="font-medium">Empty movements list!</span>
          Cannot create empty transactions.
        </Alert>
      {/if}
      {#if transaction.total_from(data.base_ccy) != transaction.total_to(data.base_ccy)}
        <Alert class="mb-6">
          <InfoCircleSolid slot="icon" class="w-5 h-5" />
          <span class="font-medium">Source and target mismatch!</span>
          Source total is EUR {transaction.total_from(data.base_ccy)} while target total is EUR {transaction.total_to(
            data.base_ccy
          )}.
        </Alert>
      {/if}
      <Button onclick={submit} disabled={submit_disabled}>
        Submit (Total: {transaction.total_from(data.base_ccy)} EUR)
      </Button>
    </Card>
  </div>
</form>
