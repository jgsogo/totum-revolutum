<script lang="ts">
    import TicketToRideBoard from '$lib/ticket_to_ride/components/TicketToRideBoard.svelte';

    let refBoard: typeof TicketToRideBoard;
</script>


<h3>= Board view =</h3>
<hr />
<hr />

<TicketToRideBoard bind:this={refBoard} map="usa" />
<!--
<img alt="The project logo" src={USAMap} />
<img alt="The project logo" src="/ticket_to_ride/maps/maps/usa.svg" /> -->
