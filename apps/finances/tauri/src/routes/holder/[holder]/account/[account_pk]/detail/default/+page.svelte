<script lang="ts">
  import AccountChart from "$lib/components/AccountChart.svelte";
  import AccountDetail from "$lib/components/AccountDetail.svelte";
  import AccountMovements from "$lib/components/AccountMovements.svelte";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();

  let last_snapshot = $state(data.snapshots.at(0));
</script>

<div class="mt-px space-y-4">
  <div class="grid gap-4 xl:grid-cols-2 2xl:grid-cols-3">
    <AccountDetail holder={data.holder_context.holder()} base_media_url={data.base_media_url} bind:account={data.account} bind:last_snapshot></AccountDetail>
    <AccountChart bind:account={data.account} bind:snapshots={data.snapshots}></AccountChart>
  </div>
  <AccountMovements bind:account={data.account} bind:snapshots={data.snapshots} bind:movements={data.movements}
  ></AccountMovements>
</div>
