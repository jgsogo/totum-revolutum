<script lang="ts">
  import { page } from '$app/state';
  import type { PageProps } from './$types';
  import { startGame } from '$lib/api/start_game';
  import { onMount } from 'svelte';
  import { connectToRoom } from '$lib/api/socket';
  import { goto } from '$app/navigation';

  let { data }: PageProps = $props();
  const roomUUID: string = page.params.roomId!;

  const on_game_update = async (payload: { game_type_id: string } | undefined) => {
    console.log(`[frontend] Get game update for room ${roomUUID}`);
    if (payload !== undefined) {
      goto(`/${payload.game_type_id}/${roomUUID}/admin`);
    }
  };

  onMount(() => {
    connectToRoom(
      roomUUID,
      async () => {},
      async () => {},
      on_game_update,
    );
  });

  async function selectGame(game_type_slug: string) {
    try {
      console.log(`[frontend] Select game_type '${game_type_slug}'`);
      let r = await startGame(roomUUID, game_type_slug);
      console.log(`[frontend] Response: ${JSON.stringify(r)}`);
    } catch (err) {
      console.error('[frontend] Failed to select game', err);
    }
  }
</script>

<h3>= Admin view =</h3>


<hr />
<hr />
{#each data.game_types as game_type}
  <p>Game type: {game_type.name} - {game_type.slug} - {game_type.description}</p>
  <button onclick={() => selectGame(game_type.slug)} class="px-4 py-2 bg-blue-600 text-white rounded"> {game_type.name} </button>
  <hr />
{/each}
<hr />
