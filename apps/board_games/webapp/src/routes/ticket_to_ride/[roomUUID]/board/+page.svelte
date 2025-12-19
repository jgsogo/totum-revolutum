<script lang="ts">
  import { page } from '$app/state';
  //   import TicketToRideBoard from '$lib/ticket_to_ride/components/TicketToRideBoard.svelte';
  import { MapData } from '../../../../../../games/ticket_to_ride/maps/js';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  // const usa_data: MapData = page.data.usa_data;
  const routes_svg: string = data.routes_svg;
  const usa_data: MapData = data.map_data;

  const background = usa_data.background()

  //   let refBoard: typeof TicketToRideBoard;
</script>

<h3>= Board view =</h3>
<hr />
<hr />

<h1>{usa_data.name()}</h1>

<svg width={usa_data.size()[0]} height={usa_data.size()[1]} xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
  {#if background}
  <image href="/ticket_to_ride/maps/usa/{background.filename()}" transform="rotate({background.rotate()}) scale({background.scale()}) translate({background.translate()[0]}, {background.translate()[1]})"></image>
  {/if}
  <image href={routes_svg}></image>
</svg>
