<script lang="ts">
  import { onMount } from 'svelte';
  import { connectToRoom } from '$lib/api/socket';
  import { page, get } from '$app/stores';
  import { sendCommand } from '$lib/api/send_command';
  import { qr } from '@svelte-put/qr/svg';
  import { player_id } from '$lib/player_id';
  import { redirect } from '@sveltejs/kit';


  const roomId = $page.params.roomId!;
  onMount(() => {
    connectToRoom(roomId);
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
    redirect(307, $page.url.pathname + "/observe");
  }

  async function joinGame() {
    redirect(307, $page.url.pathname + "/join");
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

<h2>Game status</h2>
<h3>Players</h3>
<h3>Board</h3>
<p>Waiting for updates...</p>
<hr />
<hr />

<h2>Commands</h2>
<button on:click={handleClick} class="px-4 py-2 bg-blue-600 text-white rounded"> Send Test Command </button>
<button on:click={becomeObserver} class="px-4 py-2 bg-blue-600 text-white rounded"> Become observer </button>
<button on:click={joinGame} class="px-4 py-2 bg-blue-600 text-white rounded"> Join the game </button>

<hr />
<hr />
