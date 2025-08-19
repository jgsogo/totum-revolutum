<script lang="ts">
  import { onMount } from 'svelte';
  import { connectToRoom } from '$lib/api/socket';
  import { page } from '$app/stores';
  import { qr } from '@svelte-put/qr/svg';
  import { player_id } from '$lib/player_id';

  const roomId = $page.params.roomId!;

  let room_data = $state({});
  const on_room_update = async (payload: JSON) => {
    room_data = payload;
  };

  let participants_data = $state({});
  const on_participants_update = async (payload: JSON) => {
    participants_data = payload;
  };

  let game_data = $state({});
  const on_game_update = async (payload: JSON) => {
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
{JSON.stringify(room_data)}
<hr />

<h2>Participants data</h2>
{JSON.stringify(participants_data)}
<hr />

<h2>Game data</h2>
{JSON.stringify(game_data)}
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
