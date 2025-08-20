<script lang="ts">
  import { onMount } from 'svelte';
  import { connectToRoom } from '$lib/api/socket';
  import { page } from '$app/stores';
  import { qr } from '@svelte-put/qr/svg';
  import { player_id } from '$lib/player_id';

  const roomId = $page.params.roomId!;

  let room_data = $state({});
  const on_room_update = async (payload: JSON) => {
    console.log(`[frontend] Get room_update for room ${roomId}`)
    room_data = payload;
  };

  let participants_data = $state({});
  const on_participants_update = async (payload: JSON) => {
    console.log(`[frontend] Get participants update for room ${roomId}`)
    participants_data = payload;
  };

  let game_data = $state({});
  const on_game_update = async (payload: JSON) => {
    console.log(`[frontend] Get game update for room ${roomId}`)
    game_data = payload;
  };

  onMount(() => {
    connectToRoom(roomId, on_room_update, on_participants_update, on_game_update);
  });

  const full_domain = 'http://localhost:3000'; // FIXME: Read from envvar
  const join_url = full_domain + $page.url.pathname + '/join';
  const admin_url = full_domain + $page.url.pathname + '/admin';
</script>

<p>Player ID: {JSON.stringify(player_id)}</p>

<h1>Room: {$page.params.roomId}</h1>
<hr />
<hr />

<h2>Room data</h2>
<table>
  {#each Object.entries(room_data) as [key, value]}
    <tr>
        <td>{key}</td>
        <td>{value}</td>
    </tr>
{/each}
</table>
<hr />

<h2>Participants data</h2>
{#each Object.entries(participants_data) as participant_data }
  <p>-- Participant</p>
  <table>
  {#each Object.entries(participant_data) as [key, value]}
    <tr>
        <td>{key}</td>
        <td>{value}</td>
    </tr>
{/each}
</table>
{/each}
<hr />

<h2>Game data</h2>
{JSON.stringify(game_data)}
<table>
  {#each Object.entries(game_data) as [key, value]}
    <tr>
        <td>{key}</td>
        <td>{value}</td>
    </tr>
{/each}
</table>
<hr />

<hr />
<h1>Admin QR</h1>
<a href={admin_url} target="_blank">
  <svg
    use:qr={{
      data: admin_url,
      logo: 'https://svelte-put.vnphanquang.com/images/svelte-put-logo.svg',
      shape: 'circle',
    }}
    width="200px"
    height="200px"
  />
</a>
<hr />

<hr />
<h1>Join the game</h1>
<a href={join_url} target="_blank">
  <svg
    use:qr={{
      data: join_url,
      logo: 'https://svelte-put.vnphanquang.com/images/svelte-put-logo.svg',
      shape: 'circle',
    }}
    width="200px"
    height="200px"
  />
</a>
<hr />
<hr />
