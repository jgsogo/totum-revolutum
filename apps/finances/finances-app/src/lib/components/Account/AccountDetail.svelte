<script lang="ts">
  import { DataHandler } from "@vincjo/datatables";
  import { Account } from "$lib/models/Account";
  import AccountChart from "$lib/components/Account/AccountChart.svelte"

  // TODO: Trying different line-chart implementations
  //    https://layercake.graphics/example/MultiLine
  //   https://www.sveltecharts.com/charts/b92aa052-b9f3-4da0-93f1-199ae898ff6e
  //  https://medium.com/@stefano.agresti19/building-an-interactive-line-chart-using-svelte-and-d3-71841cf5703c
  //  https://layercake.graphics/
  // import LineChart from "$lib/components/Charts/LineChart/LineChart.svelte";

  let { account = $bindable() }: { account: Account } = $props();

</script>

<div class="flex flex-col p-5">
  <!-- Title -->
  <div class="flex w-full gap-4">
    <div class="basis-8/12">
      <h2 class="text-2xl leading-7 sm:truncate sm:text-3xl sm:tracking-tight">
        {account.name}
      </h2>
    </div>
    <div class="basis-4/12 text-right">
      <div class="btn-group variant-filled px-2 py-1 mt-1 space-x-2">
        <i class="fa-solid fa-circle-arrow-up text-[color]-400"></i>
        <i class="fa-solid fa-circle-arrow-down text-[color]-400"></i>
      </div>
      <div class="btn variant-filled px-2 py-1 mt-1 space-x-2">
        <i class="fa-solid fa-camera text-[color]-400"></i>
      </div>
    </div>
  </div>

  <!-- Horizontal rule separation -->
  <hr class="opacity-30 w-full" />

  <!-- Details -->
  <div class="flex flex-col mt-1 sm:mt-0 sm:flex-row sm:flex-wrap sm:space-x-6">
    <div class="mt-2 flex items-center text-sm text-[color]-500">
      <i class="fa-solid fa-bank mr-1.5 w-5 flex-shrink-0"></i>
      {account.holder}
    </div>
    <div class="mt-2 flex items-center text-sm text-[color]-500">
      <i class="fa-solid fa-folder-tree mr-1.5 w-5 flex-shrink-0"></i>
      {account.type}
    </div>
    <div class="mt-2 flex items-center text-sm text-[color]-500">
      <i class="fa-solid fa-calendar mr-1.5 w-5 flex-shrink-0"></i>
      {#await account.last_snapshot()}
        ..
      {:then snapshot}
        {snapshot} (@{snapshot.date_value})
      {/await}
    </div>
  </div>

  <!-- Summary: chart + in/out summary -->
  <div class="flex flex-col mt-4 sm:mt-4 sm:flex-row sm:flex-wrap">
    <div class="basis-6/12">
        <AccountChart/>
    </div>
    <div class="basis-6/12">b</div>
  </div>

  <!-- Movements -->
  <h3 class="text-xl mt-4 leading-5 sm:truncate sm:text-2xl sm:tracking-tight">
    Movements
  </h3>
  <div class="table-container mt-4">
    <!-- Native Table Element -->
    <table class="table table-hover">
      <thead>
        <tr>
          <th>Position</th>
          <th>Name</th>
          <th>Symbol</th>
          <th>Weight</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>a1</td>
          <td>a1</td>
          <td>a1</td>
          <td>a1</td>
        </tr>
      </tbody>
      <tfoot>
        <tr>
          <th colspan="3">Calculated Total Weight</th>
          <td>totalWeight</td>
        </tr>
      </tfoot>
    </table>
  </div>
</div>

<style>
  /*
      The wrapper div needs to have an explicit width and height in CSS.
      It can also be a flexbox child or CSS grid element.
      The point being it needs dimensions since the <LayerCake> element will
      expand to fill it.
    */
  .chart-container {
    width: 100%;
    height: 300px;
  }
</style>
