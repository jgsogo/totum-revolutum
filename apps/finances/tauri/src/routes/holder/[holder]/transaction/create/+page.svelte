<script lang="ts">
  import MovementForm from "$lib/forms/MovementForm.svelte";
  import type { Account } from "$lib/models/Account.js";
  import { NewMovement } from "$lib/models/Movement";
  import { Card } from "flowbite-svelte";
  import {
    CirclePlusSolid,
    CircleMinusSolid,
    PlusOutline,
    CircleMinusOutline,
    MinusOutline,
  } from "flowbite-svelte-icons";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();

  let movements_from: NewMovement[] = $state([]);
  let movements_to: NewMovement[] = $state([]);

  if (data.from_account) {
    let new_movement = new NewMovement();
    new_movement.account = data.from_account;
    movements_from.push(new_movement);
  }

  if (data.to_account) {
    let new_movement = new NewMovement();
    new_movement.account = data.to_account;
    movements_to.push(new_movement);
  }

  let removed_index = $state(0);

  const remove_movement_from = (index: number) => {
    removed_index = index;
    movements_from.splice(index, 1);
  };
  const add_movement_from = () => {
    movements_from.push(new NewMovement());
  };
  const remove_movement_to = (index: number) => {
    removed_index = index;
    movements_to.splice(index, 1);
  };
  const add_movement_to = () => {
    movements_to.push(new NewMovement());
  };
</script>

transaction/create<br />

removed_index: {removed_index}<br />
data.from_account: {data.from_account}
{Boolean(data.from_account)}<br />
data.to_account: {data.to_account}
{Boolean(data.to_account)}<br />

<div class="mt-px space-y-4">
  <div class="grid gap-4 grid-cols-2">
    <!-- from movements -->
    <div>
      Left column
      {#each movements_from as mov, i}
        <Card size="xl" class="mt-6">
          <MovementForm bind:new_movement={movements_from[i]} all_accounts_list={data.all_accounts_list} />
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
      Right column
      {#each movements_to as mov, i}
        <Card size="xl" class="mt-6">
          <MovementForm bind:new_movement={movements_to[i]} all_accounts_list={data.all_accounts_list} />
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
