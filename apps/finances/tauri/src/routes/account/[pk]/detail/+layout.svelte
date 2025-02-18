<script lang="ts">
  import AccountChart from "$lib/components/AccountChart.svelte";
  import AccountDetail from "$lib/components/AccountDetail.svelte";
  import type { Account, AccountContext, AppState, MainContext } from "../../../../../../models/src-js";

  /** @type {{ data: import('./$types').PageData }} */
  let { data, children } = $props();

  let app_state: AppState = data.app_state;

  let account_context: AccountContext = data.account_context;
  let account: Account = account_context.account();
</script>

<div class="mt-px space-y-4">
  <div class="grid gap-4 xl:grid-cols-2 2xl:grid-cols-3">
    <AccountDetail base_media_url={app_state.base_media_url()} {account}></AccountDetail>
    <AccountChart {account} snapshots={account_context.snapshots()}></AccountChart>
  </div>

  {@render children()}
</div>
