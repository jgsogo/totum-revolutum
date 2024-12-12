<script lang="ts">
  import type { TransactionGroup } from "$lib/models/TransactionGroup";
  import { Input, Checkbox, Label, Select, Textarea } from "flowbite-svelte";
  import type { NewTransaction } from "./NewTransaction.svelte";
  import Datepicker from "../Datepicker.svelte";

  let {
    transaction = $bindable(),
    show_date = $bindable(),
    date = $bindable(),
    all_transaction_groups,
  }: {
    transaction: NewTransaction;
    show_date: boolean;
    date: Date;
    all_transaction_groups: TransactionGroup[];
  } = $props();

  let transaction_groups = all_transaction_groups.map((value) => {
    return { value: value, name: value.name };
  });
</script>

<div class="px-2">
  <div class="flex -mx-2">
    <div class="w-1/2 px-2">
      <Label class="space-y-2 py-2">
        <span>Name</span>
        <Input type="text" required placeholder="Transaction name" bind:value={transaction.name} />
      </Label>

      <Label class="space-y-2 py-2">
        <span>Group</span>
        <Select items={transaction_groups} bind:value={transaction.transaction_group} />
      </Label>

      {#if show_date}
        <Label class="space-y-2 pt-2">
          <span>Date value</span>
          <Datepicker required bind:value={date} />
        </Label>
      {/if}
    </div>

    <div class="w-1/2 px-2">
      <Label class="space-y-2 py-2">
        <span>Description</span>
        <Textarea placeholder="Long description" rows="8" bind:value={transaction.description} />
      </Label>
      <Checkbox bind:checked={show_date}>All movements the same date</Checkbox>
    </div>
  </div>
</div>
