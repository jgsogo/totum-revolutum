<script lang="ts">
  import MovementForm from "$lib/forms/MovementForm/MovementForm.svelte";
  import TransactionForm from "$lib/forms/TransactionForm/TransactionForm.svelte";
  import { Alert, Button, Card, Heading, Secondary, TextPlaceholder } from "flowbite-svelte";
  import { InfoCircleSolid } from "flowbite-svelte-icons";
  import { NewTransaction } from "$lib/forms/TransactionForm/NewTransaction.svelte.js";
  import { NewMovement, NewMovementType } from "$lib/forms/MovementForm/NewMovement.svelte.js";
  import { create_transaction } from "$lib/commands.js";
  import { goToAccountDetail } from "$lib/utils.js";
  import type {
    MainContext,
    Account,
    AppState,
    AccountContext,
    HolderContext,
    Transaction,
  } from "../../../../../../../../models/src-js/index.js";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();

  let app_state: AppState = data.app_state;
  let holder_context: HolderContext = data.holder_context;
  let account_context: AccountContext = data.account_context;
  let main_context: MainContext = data.main_context;

  let from_account: Account | null = data.from_account;
  let to_account: Account | null = data.to_account;
  let last_transactions: Transaction[] = data.last_transactions;

  let initial_movements_from = from_account
    ? [
        new NewMovement(
          from_account.is_numerable() ? NewMovementType.Numerable : NewMovementType.NonNumerable,
          from_account,
          new Date()
        ),
      ]
    : [];
  let initial_movements_to = to_account
    ? [
        new NewMovement(
          to_account.is_numerable() ? NewMovementType.Numerable : NewMovementType.NonNumerable,
          to_account,
          new Date()
        ),
      ]
    : [];

  let common_date = $state(new Date());
  let transaction: NewTransaction = $state(
    new NewTransaction(new Date(), initial_movements_from, initial_movements_to)
  );
  let show_transaction_date = $state(true);
  let show_individual_dates = $derived(!show_transaction_date);

  // Form validation and submit
  let submit_disabled = $derived(!transaction.is_valid(show_transaction_date, app_state.base_ccy()));

  $effect(() => {
    transaction.set_date(common_date);
  });

  const submit = async (e: MouseEvent) => {
    e.preventDefault();
    if (transaction.is_valid(show_transaction_date, app_state.base_ccy())) {
      await create_transaction(transaction.toMessage());
      // TODO: Show error when it fails
      await goToAccountDetail(holder_context.holder(), account_context.account());
    }
  };

  let card_error_style = "border-red-600 dark:border-red-600";
</script>

<Heading tag="h1" class="mb-4" customSize="text-3xl font-extrabold  md:text-4xl lg:text-5xl">New transaction</Heading>

<form>
  {#each last_transactions as transaction}
    <p>{transaction}</p>
  {/each}

  <div class="mt-px space-y-4">
    <Card
      size="xl"
      class="mt-6 {transaction.is_valid(show_transaction_date, app_state.base_ccy()) ? '' : card_error_style}"
    >
      <TransactionForm
        bind:transaction
        bind:show_date={show_transaction_date}
        bind:date={common_date}
        all_transaction_groups={main_context.transaction_groups()}
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
            <button onclick={() => transaction.add_movement_from()} class="ml-4">
              <Secondary class="text-xs">Add movement</Secondary>
            </button>
          </div>
        </Heading>

        {#each transaction.movements_from as mov, i}
          <Card
            size="xl"
            class="mt-6 {mov.is_valid(show_individual_dates, app_state.base_ccy()) ? '' : card_error_style}"
          >
            <MovementForm
              bind:new_movement={transaction.movements_from[i]}
              base_ccy={app_state.base_ccy()}
              show_date={show_individual_dates}
              all_accounts={main_context.accounts()}
              all_movementtypes={main_context.movement_types()}
            />
            <div class="flex flex-col text-right text-xs mt-2">
              <span class="font-semibold text-primary-500"
                ><button onclick={() => transaction.remove_movement_from(i)}>Remove</button></span
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
            <button onclick={() => transaction.add_movement_to()} class="ml-4">
              <Secondary class="text-xs">Add movement</Secondary>
            </button>
          </div>
        </Heading>
        {#each transaction.movements_to as mov, i}
          <Card
            size="xl"
            class="mt-6 {mov.is_valid(show_individual_dates, app_state.base_ccy()) ? '' : card_error_style}"
          >
            <MovementForm
              bind:new_movement={transaction.movements_to[i]}
              base_ccy={app_state.base_ccy()}
              show_date={show_individual_dates}
              all_accounts={main_context.accounts()}
              all_movementtypes={main_context.movement_types()}
            />
            <div class="flex flex-col text-right text-xs mt-2">
              <span class="font-semibold text-primary-500"
                ><button onclick={() => transaction.remove_movement_to(i)}>Remove</button></span
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
      {#if transaction.total_from(app_state.base_ccy()) != transaction.total_to(app_state.base_ccy())}
        <Alert class="mb-6">
          <InfoCircleSolid slot="icon" class="w-5 h-5" />
          <span class="font-medium">Source and target mismatch!</span>
          Source total is EUR {transaction.total_from(app_state.base_ccy())} while target total is EUR {transaction.total_to(
            app_state.base_ccy()
          )}.
        </Alert>
      {/if}
      <Button onclick={submit} disabled={submit_disabled}>
        Submit (Total: {transaction.total_from(app_state.base_ccy())} EUR)
      </Button>
    </Card>
  </div>
</form>
