<script lang="ts">
  import AccountsTable from "$lib/components/AccountsTable.svelte";
  import type { Custodian, Account, Holder, AppState } from "../../../../../../../models/src-js";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();
  let app_state: AppState = data.app_state;
  let custodian: Custodian = data.custodian;
  let holder: Holder = data.holder_context.holder();
  let accounts: Account[] = data.accounts_for_custodian;
</script>

<div class="mt-px space-y-4">
  {custodian.name()}
  <img alt="Custodian {custodian.name()}" src="{app_state.base_media_url()}{custodian.photo()}" />

  <AccountsTable {holder} show_custodian={false} {accounts} />
</div>
