<script lang="ts">
  import { Card, Chart } from 'flowbite-svelte';
  import { Account, Snapshot } from '../../../models/src-js';
  import { dateWrapper2Date } from '$lib/utils';

  let { account, snapshots }: { account: Account; snapshots: Snapshot[] } = $props();

  let dates = snapshots.map((snapshot) => {
    const date_value = dateWrapper2Date(snapshot.date_value());
    let y = snapshot.amount().amount().amount();
    return { x: date_value.getTime(), y };
  });

  let options = {
    series: [
      {
        name: account.name(),
        data: dates, // FIXME: Make this variable reactive
      },
    ],
    chart: {
      type: 'area',
      stacked: false,
      height: 350,
      zoom: {
        type: 'x',
        enabled: true,
        autoScaleYaxis: true,
      },
      toolbar: {
        autoSelected: 'zoom',
      },
    },
    dataLabels: {
      enabled: false,
    },
    markers: {
      size: 1,
    },
    // title: {
    //   text: `${account.custodian} / ${account.name}`,
    //   align: "left",
    //   style: {
    //     //   cssClass: "text-xs font-normal fill-gray-500 dark:fill-gray-400",
    //     //   color: "gray-500 dark:gray-100"
    //     },
    // },
    fill: {
      type: 'gradient',
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
        style: {
          cssClass: 'text-xs font-normal fill-gray-500 dark:fill-gray-400',
        },
      },
      title: {
        text: `Snapshot (${account.ccy()})`,
        style: {
          cssClass: 'text-xs font-normal fill-gray-500 dark:fill-gray-400',
        },
      },
    },
    xaxis: {
      type: 'datetime',
      labels: {
        style: {
          cssClass: 'text-xs font-normal fill-gray-500 dark:fill-gray-400',
        },
      },
    },
    stroke: {
      curve: 'smooth',
    },
    tooltip: {
      shared: false,
      y: {
        formatter: function (val) {
          return `${val} ${account.ccy()}`;
        },
      },
    },
  };
</script>

<Card size="xl" class="p-4 sm:p-6">
  <Chart {options}></Chart>
</Card>
