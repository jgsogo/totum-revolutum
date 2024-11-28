<script lang="ts">
  import { Card, Chart } from "flowbite-svelte";
  import type { Account } from "$lib/models/Account";
  import type { Snapshot } from "$lib/models/Snapshot";
  import type { Movement } from "$lib/models/Movement";

  let { account = $bindable(), snapshots = $bindable() }: { account: Account; snapshots: Snapshot[] } = $props();

  let dates = snapshots.map((snapshot) => {
    let x = new Date(snapshot.date_value).getTime();
    let y = snapshot.amount;
    return { x, y };
  });

  let options = {
    series: [
      {
        name: "XYZ MOTORS",
        data: dates,
      },
    ],
    chart: {
      type: "area",
      stacked: false,
      height: 350,
      zoom: {
        type: "x",
        enabled: true,
        autoScaleYaxis: true,
      },
      toolbar: {
        autoSelected: "zoom",
      },
    },
    dataLabels: {
      enabled: false,
    },
    markers: {
      size: 1,
    },
    title: {
      text: `${account.custodian} / ${account.name}`,
      align: "left",
    },
    fill: {
      type: "gradient",
      gradient: {
        shadeIntensity: 1,
        inverseColors: false,
        opacityFrom: 0.5,
        opacityTo: 0,
        stops: [0, 90, 100],
      },
    },
    yaxis: {
      labels: {
        formatter: function (val) {
          return val;
        },
      },
      title: {
        text: `Snapshot (${account.ccy})`,
      },
    },
    xaxis: {
      type: "datetime",
    },
    stroke: {
      curve: "smooth",
    },
    tooltip: {
      shared: false,
      y: {
        formatter: function (val) {
          return val;
        },
      },
    },
  };
</script>

<Card size="xl" class="w-full max-w-none 2xl:col-span-2">
  <Chart {options}></Chart>
  <div class="mt-4 flex items-center justify-between border-t border-gray-200 pt-3 dark:border-gray-700 sm:pt-6">
    <!-- <LastRange />
		<More title="Sales Report" href="#top" /> -->
  </div>
</Card>
