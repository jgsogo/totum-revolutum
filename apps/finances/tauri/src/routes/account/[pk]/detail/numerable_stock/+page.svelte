<script lang="ts">
  import AccountMovements from "$lib/components/AccountMovements.svelte";
  import type { AccountContext, MainContext } from "../../../../../../models/src-js";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();

  let account_context: AccountContext = data.account_context;

  let main_context: MainContext = data.main_context;
</script>

<AccountMovements snapshots={account_context.snapshots()} movements={account_context.movements()} {main_context}
></AccountMovements>
