<script>
  import { onMount } from "svelte";
  import * as Plot from "@observablehq/plot";
  import * as d3 from "d3";

  let data;
  onMount(async () => {
    data = await d3.csv(
      "https://static.observableusercontent.com/files/ca429fb9d06a65bfd4f163364080fa45b685f8838f9ed0a0134ed7f2985aea3e6f553a18822528dff50cb0d1e2d91c4cc26a88e5d9a0625e5c41cd9121da3986?response-content-disposition=attachment%3Bfilename*%3DUTF-8%27%27crimean-war.csv"
    );
    data = data.columns
      .slice(2)
      .flatMap((cause) =>
        data.map(({ date, [cause]: deaths }) => ({ date, cause, deaths }))
      ); // pivot taller
  });

  $: crimea = data;

  let div;

  $: {
    div?.firstChild?.remove(); // remove old chart, if any
    div?.append(
      Plot.plot({
        style: "overflow: visible;",
        y: { grid: true },
        color: { legend: true },
        marks: [

          Plot.axisY({ fontSize: 16 }),
          Plot.ruleY([0]),
          Plot.lineY(crimea, {
            x: "date",
            y: "deaths",
            stroke: "cause",
            marker: true,
          }),
          Plot.crosshair(crimea, { x: "date", y: "deaths" }),
        ],
      })
    ); // add the new chart
  }
</script>

<div bind:this={div} role="img"></div>
