<script lang="ts">
  import { onMount } from 'svelte';
  import { connectToRoom } from '$lib/api/socket';
  import { page, get } from '$app/stores';
  import { sendCommand } from '$lib/api/send_command';
  import { qr } from '@svelte-put/qr/svg';
  import { player_id } from '$lib/player_id';

  const roomId = $page.params.roomId!;
  onMount(() => {
    connectToRoom(roomId);
  });

  const full_domain = 'http://localhost:3000'; // FIXME: Read from envvar
//   const game_url = full_domain + `room/${roomId}/join`;


</script>

<p>Player ID: {JSON.stringify(player_id)}</p>

<h1>Room: {$page.params.roomId}</h1>
<hr />
<hr />

<h2>Join the game</h2>
