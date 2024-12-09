<script lang="ts">
  import type { Account } from "$lib/models/Account";
  import { NewMovement } from "$lib/models/Movement";
  import type { Snapshot } from "$lib/models/Snapshot";
  import type { TransactionGroup } from "$lib/models/TransactionGroup";
  import {
    Button,
    Input,
    Checkbox,
    Label,
    ButtonGroup,
    InputAddon,
    Datepicker,
    Helper,
    Alert,
    Select,
    Textarea,
  } from "flowbite-svelte";
  import { InfoCircleSolid } from "flowbite-svelte-icons";

  let {
    transaction_group = $bindable(),
    transaction_name = $bindable(),
    transaction_description = $bindable(),
    transaction_date = $bindable(),
    show_date = $bindable(),
    all_transaction_groups,
  }: {
    transaction_group: TransactionGroup | undefined;
    transaction_name: string | undefined;
    transaction_description: string | undefined;
    transaction_date: Date | undefined;
    show_date: boolean;
    all_transaction_groups: TransactionGroup[];
  } = $props();

  let transaction_groups = all_transaction_groups.map((value) => {
    return { value: value, name: value.name };
  });

  let dateFormat: Intl.DateTimeFormatOptions = {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  };
</script>

<div class="px-2">
  <div class="flex -mx-2">
    <div class="w-1/2 px-2">
      <Label class="space-y-2 py-2">
        <span>Name</span>
        <Input type="text" required placeholder="Transaction name" bind:value={transaction_name} />
      </Label>

      <Label class="space-y-2 py-2">
        <span>Group</span>
        <Select items={transaction_groups} bind:value={transaction_group} />
      </Label>

      {#if show_date}
        <Label class="space-y-2 pt-2">
          <span>Date value</span>
          <Datepicker required bind:value={transaction_date} {dateFormat} />
        </Label>
      {/if}

    </div>

    <div class="w-1/2 px-2">
      <Label class="space-y-2 py-2">
        <span>Description</span>
        <Textarea placeholder="Long description" rows="8" bind:value={transaction_description} />
      </Label>
      <Checkbox bind:checked={show_date}>All movements the same date</Checkbox>
    </div>
  </div>
</div>
