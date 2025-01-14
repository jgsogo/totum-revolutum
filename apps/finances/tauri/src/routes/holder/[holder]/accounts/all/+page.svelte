<script lang="ts">
  import AccountsTable from "$lib/components/AccountsTable.svelte";
  import type { Account, Holder } from "../../../../../../models/src-js";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();
  let holder: Holder = data.holder_context.holder();
  let accounts: Account[] = data.holder_context.accounts();
</script>

<div class="mt-px space-y-4">
  <AccountsTable {holder} {accounts} />
</div>
