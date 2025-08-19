<script lang="ts">
  import { page } from '$app/stores';
  import { player_id } from '$lib/player_id';
  import type { PageProps } from './$types';
  import { startGame } from '$lib/api/start_game';

  let { data }: PageProps = $props();
  const roomUUID: string = $page.params.roomId!;

  async function selectGame(game_type_slug: string) {
    try {
      console.log(`[frontend] Select game_type '${game_type_slug}'`);
      let r = await startGame(roomUUID, game_type_slug);
      console.log(`[frontend] Response: ${JSON.stringify(r)}`);

      // TODO: Show 'Do you want to play/join?' link
    } catch (err) {
      console.error('[frontend] Failed to select game', err);
    }
  }

</script>

<h1>Admin view</h1>
<hr />

<p>Player ID: {JSON.stringify(player_id)}</p>
<h1>Room: {roomUUID}</h1>

<hr/>
<hr/>
{#each data.game_types as game_type}
  <p>Game type: {game_type.name} - {game_type.slug} - {game_type.description}</p>
  <button onclick={() => selectGame(game_type.slug)} class="px-4 py-2 bg-blue-600 text-white rounded"> {game_type.name} </button>
  <hr/>
{/each}
<hr/>

Wanna join? Click the link:
<a href="/room/{roomUUID}/join">Join the game!</a>
