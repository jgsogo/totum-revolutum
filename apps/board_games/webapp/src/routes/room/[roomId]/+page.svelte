<script lang="ts">
	import { onMount } from 'svelte';
	import { connectToRoom } from '$lib/api/socket';
	import { page } from '$app/stores';

	onMount(() => {
		const roomId = $page.params.roomId;
		connectToRoom(roomId);
	});
</script>

<h1>Room: {$page.params.roomId}</h1>
<p>Waiting for updates...</p>
