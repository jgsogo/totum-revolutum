<script lang="ts">
	import { onMount } from 'svelte';
	import { connectToRoom } from '$lib/api/socket';
	import { page, get } from '$app/stores';
	import { sendCommand } from '$lib/api/send_command';

	const roomId = $page.params.roomId;
	onMount(() => {
		connectToRoom(roomId);
	});

  async function handleClick() {
    try {
	let myarr = new Uint8Array([1,2,3,])	;
      let r = await sendCommand(roomId, myarr);
      console.log('Command sent!');
	  console.log(`Response: ${JSON.stringify(r)}`);
    } catch (err) {
      console.error('Failed to send command', err);
    }
  }
</script>

<h1>Room: {$page.params.roomId}</h1>
<p>Waiting for updates...</p>

<button on:click={handleClick} class="px-4 py-2 bg-blue-600 text-white rounded">
  Send Test Command
</button>
