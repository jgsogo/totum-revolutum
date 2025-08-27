<script lang="ts">
  import { onMount } from 'svelte';
  import { connectToRoom } from '$lib/api/socket';
  import { page } from '$app/state';
  import { qr } from '@svelte-put/qr/svg';
  import { Board } from '../../../../../../games/tic_tac_toe/webapp/src/index';
  import TicTacToeBoard from '$lib/tictactoe/components/TicTacToeBoard.svelte';

  const roomUUID = page.params.roomUUID!;

  let room_data = $state({});
  const on_room_update = async (payload: JSON) => {
    console.log(`[frontend] Get room_update for room ${roomUUID}`);
    room_data = payload;
  };

  let participants_data = $state([]);
  const on_participants_update = async (payload: JSON) => {
    console.log(`[frontend] Get participants update for room ${roomUUID}`);
    participants_data = payload;
  };

  let game_data = $state({});
  let board: Board | undefined = $state();
  const on_game_update = async (payload: JSON) => {
    console.log(`[frontend] Get game update for room ${roomUUID}`);
    game_data = payload;
    board = Board.create_from_array(game_data.state_data);
  };

  onMount(() => {
    connectToRoom(roomUUID, on_room_update, on_participants_update, on_game_update);
  });

  const full_domain = 'http://localhost:3000'; // FIXME: Read from envvar
  const play_url = full_domain + `/tic_tac_toe/${roomUUID}/play`;

  let refBoard: typeof TicTacToeBoard;
</script>

<h3>= Board view =</h3>
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
{#each participants_data as participant_data}
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
<table>
  {#each Object.entries(game_data) as [key, value]}
    <tr>
      <td>{key}</td>
      <td>{value}</td>
    </tr>
  {/each}
</table>
{#if board}
  <p>This is the board: '{board.status()}' ({board.status().length})</p>
{/if}
<hr />

<hr />
<h1>Play QR</h1>
<a href={play_url} target="_blank">
  <svg
    use:qr={{
      data: play_url,
      logo: 'https://svelte-put.vnphanquang.com/images/svelte-put-logo.svg',
      shape: 'circle',
    }}
    width="200px"
    height="200px"
  />
</a>
<hr />

<TicTacToeBoard bind:this={refBoard} size={420} />
