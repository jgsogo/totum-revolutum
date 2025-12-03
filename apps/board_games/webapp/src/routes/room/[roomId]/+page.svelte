<script lang="ts">
  import { onMount } from 'svelte';
  import { connectToRoom } from '$lib/api/socket';
  import { page } from '$app/state';
  import { qr } from '@svelte-put/qr/svg';
  import { goto } from '$app/navigation';

  const roomId = page.params.roomId!;
  const PORT = process.env.PORT || 3000;
  const DOMAIN_NAME = process.env.DOMAIN_NAME || "192.168.1.38";  // FIXME: This is my IP!!

  let room_data = $state({});
  const on_room_update = async (payload: JSON) => {
    console.log(`[frontend] Get room_update for room ${roomId}`);
    room_data = payload;
  };

  const on_game_update = async (payload: { game_type_id: string } | undefined) => {
    console.log(`[frontend] Get game update for room ${roomId}`);
    if (payload !== undefined) {
      goto(`/${payload.game_type_id}/${roomId}/board`);
    }
  };

  onMount(() => {
    connectToRoom(roomId, on_room_update, async () => {}, on_game_update);
  });

  const full_domain = `http://${DOMAIN_NAME}:${PORT}`;
  const admin_url = full_domain + `/room/${roomId}/admin`;
</script>

<h3>= Room view =</h3>

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
