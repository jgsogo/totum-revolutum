<script lang="ts">
  import { onMount } from 'svelte';

  type TicketToRideBoardProps = {
    map: string;
  };

  let { map }: TicketToRideBoardProps = $props();

  onMount(() => {
    console.log(`[frontend] ticket_to_ride | onMount()`);
  });

  export function updateBoard() {
    console.log(`[frontend] ticket_to_ride | updateBoard()`);
  }
</script>

<svg width="1200" height="773" xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink'>
    <image href="/ticket_to_ride/maps/{map}-gen.svg" ></image>
    <image href="/ticket_to_ride/maps/{map}-background.svg" ></image>
</svg>
<!-- <img alt="Ticket to Ride {map}" src="/ticket_to_ride/maps/{map}-gen.svg" /> -->
