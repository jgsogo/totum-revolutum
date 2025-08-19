<script lang="ts">
  import { onMount } from 'svelte';
  import { connectToRoom } from '$lib/api/socket';
  import { page } from '$app/stores';
  import { sendCommand } from '$lib/api/send_command';
  import { qr } from '@svelte-put/qr/svg';
  import { player_id } from '$lib/player_id';
  import { goto } from '$app/navigation';


  const roomId = $page.params.roomId!;

  let room_data = $state({});
  const on_room_update = async (payload: JSON) => {
    room_data = payload;
  }

  let participants_data = $state({});
  const on_participants_update = async (payload: JSON) => {
    participants_data = payload;
  }

  let game_data = $state({});
  const on_game_update = async (payload: JSON) => {
    game_data = payload;
  }

  onMount(() => {
    connectToRoom(roomId, on_room_update, on_participants_update, on_game_update);
  });

  async function handleClick() {
    try {
      let i = Math.floor(Math.random() * 10);
	  console.log(`[browser] Random int (browser): ${i}`);
      let myarr = new Uint8Array([i, 2, 3]);
      let r = await sendCommand(roomId, myarr);
      console.log('[browser] Command sent!');
      console.log(`[browser] Response: ${JSON.stringify(r)}`);
    } catch (err) {
      console.error('[browser] Failed to send command', err);
    }
  }

  async function becomeObserver() {
    goto($page.url.pathname + "/observe");
  }

  async function joinGame() {
    goto($page.url.pathname + "/join");
  }

  const full_domain = 'http://localhost:3000'; // FIXME: Read from envvar
  const join_url = full_domain + $page.url.pathname + '/join';
</script>

<p>Player ID: {JSON.stringify(player_id)}</p>

<h1>Room: {$page.params.roomId}</h1>
<hr />
<hr />

<h2>Join the game</h2>
Join: {join_url}
<svg
  use:qr={{
    data: join_url,
    logo: 'https://svelte-put.vnphanquang.com/images/svelte-put-logo.svg',
    shape: 'circle',
  }}
  width="200px"
  height="200px"
/>
<hr />
<hr />

<h2>Room data</h2>
{JSON.stringify(room_data)}
<hr />

<h2>Participants data</h2>
{JSON.stringify(participants_data)}
<hr />

<h2>Game data</h2>
{JSON.stringify(game_data)}
<hr />

<h2>Commands</h2>
<button on:click={handleClick} class="px-4 py-2 bg-blue-600 text-white rounded"> Send Test Command </button>
<button on:click={becomeObserver} class="px-4 py-2 bg-blue-600 text-white rounded"> Become observer </button>
<button on:click={joinGame} class="px-4 py-2 bg-blue-600 text-white rounded"> Join the game </button>

<hr />
<hr />
