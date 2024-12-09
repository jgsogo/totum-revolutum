<script lang="ts">
  import MovementForm from "$lib/forms/MovementForm.svelte";
  import type { Account } from "$lib/models/Account.js";
  import TransactionForm from "$lib/forms/TransactionForm.svelte";
  import { Alert, Button, Card, Heading } from "flowbite-svelte";
  import { PlusOutline, MinusOutline, InfoCircleSolid } from "flowbite-svelte-icons";
  import { NewTransaction, type TransactionGroup } from "$lib/models/TransactionGroup.js";
  import type { MovementType } from "$lib/models/MovementType.js";
  import { NewMovement } from "$lib/models/Movement.js";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();

  type MovementData = {
    account?: Account;
    movement_type?: MovementType;
    date_value?: Date;
    amount?: number;
    quantity?: number;
    unit_value?: number;
    fx?: number;
    is_valid: boolean;
  };

  let movements_from: MovementData[] = $state([]);
  let movements_to: MovementData[] = $state([]);
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
  function movement_total(mov: MovementData): number {
    if (!mov.account) return 0;
    let total = 0;
    if (mov.account.is_numerable) {
      if (!mov.quantity || !mov.unit_value) return 0;
      total = mov.quantity * mov.unit_value;
    } else {
      if (!mov.amount) return 0;
      total = mov.amount;
    }
    // Apply FX
    if (mov.fx) {
      total = total / mov.fx;
    }
    return total;
  }

  let total_source = $derived.by(() => {
    const total_sum_initial = 0;
    return movements_from.reduce((acc, mov: MovementData) => acc + movement_total(mov), total_sum_initial);
  });
  let total_target = $derived.by(() => {
    const total_sum_initial = 0;
    return movements_to.reduce((acc, mov: MovementData) => acc + movement_total(mov), total_sum_initial);
  });

  function add_movement_from(account?: Account) {
    movements_from = movements_from.concat({
      account: account,
      date_value: new Date(),
      is_valid: false,
      movement_type: undefined,
    });
  }
  function add_movement_to(account?: Account) {
    movements_to = movements_to.concat({
      account: account,
      date_value: new Date(),
      is_valid: false,
      movement_type: undefined,
    });
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
    // Collect movements_from -> NewMovements
    let movs_from: NewMovement[] = movements_from.map((v: MovementData) => {
      // TODO: Errors if some fields are null
      return new NewMovement(v.account!, v.movement_type!, v.amount, v.date_value, v.quantity, v.unit_value, v.fx);
    });

    // Collect movements_to -> NewMovements
    let movs_to: NewMovement[] = movements_to.map((v: MovementData) => {
      // TODO: Errors if some fields are null
      return new NewMovement(v.account!, v.movement_type!, v.amount, v.date_value, v.quantity, v.unit_value, v.fx);
    });

    // Create NewTransaction
    // TODO: Errors if some fields are null
    let new_transacion = new NewTransaction(
      transation_data.name!,
      movs_from,
      movs_to,
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
        <Heading tag="h2" class="mb-4" customSize="text-2xl font-extrabold  md:text-3xl lg:text-4xl">
          Source accounts
        </Heading>

        {#each movements_from as mov, i}
          <Card size="xl" class="mt-6 {mov.is_valid ? '' : card_error_style}">
            <MovementForm
              bind:account={mov.account}
              bind:movementtype={mov.movement_type}
              bind:date_value={mov.date_value}
              bind:amount={mov.amount}
              bind:quantity={mov.quantity}
              bind:unit_value={mov.unit_value}
              bind:fx={mov.fx}
              bind:is_valid={mov.is_valid}
              base_ccy={data.base_ccy}
              show_date={show_individual_dates}
              all_accounts={data.all_accounts}
              all_movementtypes={data.all_movementtypes}
            />
            <button
              onclick={() => remove_movement_from(i)}
              class="dark:ring-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none rounded-lg p-2.5"
            >
              <MinusOutline />
            </button>
          </Card>
        {/each}
        <Card size="xl" class="mt-6">
          <button
            onclick={() => add_movement_from()}
            class="dark:ring-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none rounded-lg p-2.5"
          >
            <PlusOutline />
          </button>
        </Card>
      </div>

      <!-- to movements -->
      <div>
        <Heading tag="h2" class="mb-4" customSize="text-2xl font-extrabold  md:text-3xl lg:text-4xl">
          Target accounts
        </Heading>
        {#each movements_to as mov, i}
          <Card size="xl" class="mt-6 {mov.is_valid ? '' : card_error_style}">
            <MovementForm
              bind:account={mov.account}
              bind:movementtype={mov.movement_type}
              bind:date_value={mov.date_value}
              bind:amount={mov.amount}
              bind:quantity={mov.quantity}
              bind:unit_value={mov.unit_value}
              bind:fx={mov.fx}
              bind:is_valid={mov.is_valid}
              base_ccy={data.base_ccy}
              show_date={show_individual_dates}
              all_accounts={data.all_accounts}
              all_movementtypes={data.all_movementtypes}
            />
            <button
              onclick={() => remove_movement_to(i)}
              class="dark:ring-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none rounded-lg p-2.5"
            >
              <MinusOutline />
            </button>
          </Card>
        {/each}
        <Card size="xl" class="mt-6">
          <button
            onclick={() => add_movement_to()}
            class="dark:ring-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none rounded-lg p-2.5"
          >
            <PlusOutline />
          </button>
        </Card>
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
      <Button onclick={submit} disabled={submit_disabled}>Submit</Button>
    </Card>
  </div>
</form>
