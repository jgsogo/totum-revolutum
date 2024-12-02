<script lang="ts">
  import type { Account } from "$lib/models/Account";
  import type { NewMovement } from "$lib/models/Movement";
  import type { Snapshot } from "$lib/models/Snapshot";
  import { Button, Input, Label, ButtonGroup, InputAddon, Datepicker, Helper, Alert, Select } from "flowbite-svelte";
  import { InfoCircleSolid } from "flowbite-svelte-icons";

  let {
    new_movement = $bindable(),
    all_accounts_list,
  }: {
    new_movement: NewMovement;
    all_accounts_list: Account[];
  } = $props();

  let accounts = $derived(all_accounts_list.map(value => {return {value: value, name: value.name};} ));

</script>

<form class="flex flex-col space-y-6" action="#">
  <!-- <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Add snapshot</h3> -->

  <Label class="space-y-2">
    <span>Account</span>

    <Select class="mt-2" items={accounts} bind:value={new_movement.account} />
  </Label>


</form>
