<script lang="ts">
  import { page } from '$app/state';
  //   import TicketToRideBoard from '$lib/ticket_to_ride/components/TicketToRideBoard.svelte';
  import { MapData } from '../../../../../../games/ticket_to_ride/maps/js';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  // const usa_data: MapData = page.data.usa_data;
  const usa_data: MapData = data.map_data;

  //   let refBoard: typeof TicketToRideBoard;
</script>

<h3>= Board view =</h3>
<hr />
<hr />

<h1>{usa_data.name()}</h1>

<!-- <TicketToRideBoard bind:this={refBoard} map="usa" /> -->
<!--
<img alt="The project logo" src={USAMap} />
<img alt="The project logo" src="/ticket_to_ride/maps/maps/usa.svg" /> -->
