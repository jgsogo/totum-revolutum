<script lang="ts">
  import AccountsTable from "$lib/components/AccountsTable.svelte";
  import type { Account, Holder } from "../../../../models/src-js";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();
  // let holder: Holder = data.holder_context.holder();
  // let accounts: Account[] = data.holder_context.accounts();
</script>

<div class="mt-px space-y-4">
  children.selectedAccounts: {data} ---cccc
  children.selectedAccounts: {data.selectedAccounts} ---cccc
  <!-- {#snippet children({ selectedAccounts: Account[] })}
    children.selectedAccounts: {selectedAccounts} ---cccc
  {/snippet} -->

  <!-- <AccountsTable {holder} accounts={data.selectedAccounts} /> -->
</div>
