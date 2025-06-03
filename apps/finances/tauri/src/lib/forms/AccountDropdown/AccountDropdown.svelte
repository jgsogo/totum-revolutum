<script lang="ts">
  import { Account } from "../../../../models/src-js";
  import MultilevelDropdown from "../MultilevelDropdown.svelte";

  let {
    account = $bindable(),
    accounts = $bindable(),
  }: {
    account: Account | undefined;
    accounts: Account[];
  } = $props();

  const groupBy = (acc: Account) => acc.custodian().name();

  const labelInGroup = (acc: Account) => acc.name();

  const equalItems = (lhs: Account, rhs: Account) => {
    return lhs.pk() === rhs.pk();
  };

  const labelForItem = (it: Account) => `${it.custodian().name()} | ${it.name()}`;
</script>

<!-- Common fields -->
<MultilevelDropdown bind:value={account} items={accounts} {labelForItem} {equalItems} {groupBy} {labelInGroup} on:change />
