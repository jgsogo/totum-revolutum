<script lang="ts">
  import MovementForm from "$lib/forms/MovementForm.svelte";
  import type { Account } from "$lib/models/Account.js";
  import { NewMovement } from "$lib/models/Movement";
  import TransactionForm from "$lib/forms/TransactionForm.svelte";
  import { Card, Indicator, Heading } from "flowbite-svelte";
  import {
    CirclePlusSolid,
    CircleMinusSolid,
    PlusOutline,
    CircleMinusOutline,
    MinusOutline,
  } from "flowbite-svelte-icons";
  import type { TransactionGroup } from "$lib/models/TransactionGroup.js";
  import type { MovementType } from "$lib/models/MovementType.js";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();

  let movements_from: {
    account?: Account;
    movement_type?: MovementType;
    date_value?: Date;
    amount?: number;
    quantity?: number;
    unit_value?: number;
  }[] = $state([]);
  let movements_to: {
    account?: Account;
    movement_type?: MovementType;
    date_value?: Date;
    amount?: number;
    quantity?: number;
    unit_value?: number;
  }[] = $state([]);
  let transation_data: { group?: TransactionGroup; name?: string; description?: string; date_value?: Date } = $state(
    {date_value: new Date()}
  );
  let show_transaction_date = $state(true);
  let show_individual_dates = $derived(!show_transaction_date);

  function add_movement_from(account?: Account) {
    movements_from = movements_from.concat({
      account: account,
    //   movement_type: undefined,
      date_value: new Date(),
    //   amount: undefined,
    //   quantity: undefined,
    //   unit_value: undefined,
    });
  }
  function add_movement_to(account?: Account) {
    movements_to = movements_to.concat({
      account: account,
    //   movement_type: undefined,
      date_value: new Date(),
    //   amount: undefined,
    //   quantity: undefined,
    //   unit_value: undefined,
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

  let all_movementtypes = data.all_movementtypes.map((value) => {
    return { value: value, name: value.getBreadcrumbs()!.join(" / ") };
  }).sort((lhs, rhs) => lhs.name < rhs.name ? - 1 : lhs.name > rhs.name ? 1 : 0);

  let all_accounts = data.all_accounts.map((value) => {
    return { value: value, name: `${value.custodian} | ${value.name}` };
  }).sort((lhs, rhs) => lhs.name < rhs.name ? - 1 : lhs.name > rhs.name ? 1 : 0);

</script>

<Heading tag="h1" class="mb-4" customSize="text-3xl font-extrabold  md:text-4xl lg:text-5xl">New transaction</Heading>

<div class="mt-px space-y-4">
  <Card size="xl" class="mt-6">
    <TransactionForm
      bind:transaction_name={transation_data.name}
      bind:transaction_description={transation_data.description}
      bind:transaction_date={transation_data.date_value}
      bind:transaction_group={transation_data.group}
      bind:show_date={show_transaction_date}
      all_transaction_groups={all_transaction_groups}
    />
  </Card>
</div>

<div class="mt-4 space-y-4">
  <div class="grid gap-4 grid-cols-2">

    <!-- from movements -->
    <div>
      <Heading tag="h2" class="mb-4" customSize="text-2xl font-extrabold  md:text-3xl lg:text-4xl"
        >Source accounts</Heading
      >

      {#each movements_from as mov, i}
        <Card size="xl" class="mt-6">
          <MovementForm
            bind:account={mov.account}
            bind:movement_type={mov.movement_type}
            bind:date_value={mov.date_value}
            bind:amount={mov.amount}
            bind:quantity={mov.quantity}
            bind:unit_value={mov.unit_value}
            show_date={show_individual_dates}
            {all_accounts}
            {all_movementtypes}
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
      <Heading tag="h2" class="mb-4" customSize="text-2xl font-extrabold  md:text-3xl lg:text-4xl"
        >Target accounts</Heading
      >
      {#each movements_to as mov, i}
        <Card size="xl" class="mt-6">
          <MovementForm
            bind:account={mov.account}
            bind:movement_type={mov.movement_type}
            bind:date_value={mov.date_value}
            bind:amount={mov.amount}
            bind:quantity={mov.quantity}
            bind:unit_value={mov.unit_value}
            show_date={show_individual_dates}
            {all_accounts}
            {all_movementtypes}
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
