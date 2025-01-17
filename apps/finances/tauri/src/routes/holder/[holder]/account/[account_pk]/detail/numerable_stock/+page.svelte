<script lang="ts">
  import AccountChart from "$lib/components/AccountChart.svelte";
  import AccountDetail from "$lib/components/AccountDetail.svelte";
  import AccountMovements from "$lib/components/AccountMovements.svelte";
  import {
    Account,
    AccountContext,
    type AppState,
    type Holder,
    type HolderContext,
  } from "../../../../../../../../models/src-js";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();

  let app_state: AppState = data.app_state;

  let holder_context: HolderContext = data.holder_context;
  let holder: Holder = holder_context.holder();

  let account_context: AccountContext = data.account_context;
  let account: Account = account_context.account();
  let last_snapshot = account_context.snapshots().at(0);
</script>

<div class="mt-px space-y-4">
  <div class="grid gap-4 xl:grid-cols-2 2xl:grid-cols-3">
    <AccountDetail {holder} base_media_url={app_state.base_media_url()} {account} {last_snapshot}></AccountDetail>
    <AccountChart {account} snapshots={account_context.snapshots()}></AccountChart>
  </div>
  <AccountMovements snapshots={account_context.snapshots()} movements={account_context.movements()}
  ></AccountMovements>
</div>
