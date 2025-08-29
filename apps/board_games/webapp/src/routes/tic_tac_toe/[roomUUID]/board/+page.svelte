<script lang="ts">
  import { onMount } from 'svelte';
  import { connectToRoom } from '$lib/api/socket';
  import { page } from '$app/state';
  import { qr } from '@svelte-put/qr/svg';
  import { Board } from '../../../../../../games/tic_tac_toe/webapp/src/index';
  import TicTacToeBoard from '$lib/tictactoe/components/TicTacToeBoard.svelte';

  type Participant = {
    // TODO: Move types to some common file
    id: string;
    role: string;
    player_number: number;
  };

  const roomUUID = page.params.roomUUID!;
  let refBoard: typeof TicTacToeBoard;
  let playerX: Participant | undefined = $state(undefined);
  let playerO: Participant | undefined = $state(undefined);

  let room_data = $state({});
  const on_room_update = async (payload: JSON) => {
    console.log(`[frontend] Get room_update for room ${roomUUID}`);
    room_data = payload;
  };

  let participants_data: Participant[] = $state([]);
  const on_participants_update = async (payload: Participant[]) => {
    console.log(`[frontend] Get participants update for room ${roomUUID}`);
    participants_data = payload;
    playerX = payload.find((v) => v.player_number == 0);
    playerO = payload.find((v) => v.player_number == 1);
  };

  let game_data = $state({});
  let board: Board | undefined = $state();
  const on_game_update = async (payload: JSON) => {
    console.log(`[frontend] Get game update for room ${roomUUID}`);
    game_data = payload;
    board = Board.create_from_array(game_data.state_data);
    refBoard.updateBoard([...board.status()]);

    if (board.winner() !== undefined) {
      const winnning_line = board.winner()!.line();
      refBoard.winningLine(winnning_line[0], winnning_line[2]);
    }
  };

  onMount(() => {
    connectToRoom(roomUUID, on_room_update, on_participants_update, on_game_update);
  });

  const full_domain = 'http://localhost:3000'; // FIXME: Read from envvar
  const playX_url = full_domain + `/tic_tac_toe/${roomUUID}/play/X`;
  const playO_url = full_domain + `/tic_tac_toe/${roomUUID}/play/O`;
</script>

<h3>= Board view =</h3>
<hr />
<hr />

<h1>Scan the following QRs to play</h1>
<div style="width: 100%; display: table;">
  <div style="display: table-row">
    <div style="width: 600px; display: table-cell;">
      <p>Player X</p>
      {#if playerX}
        <table>
          <tbody>
            <tr><td>ID:</td><td>{playerX.id}</td></tr>
            <tr><td>Number:</td><td>{playerX.player_number}</td></tr>
            <tr><td>Role</td><td>{playerX.role}</td></tr>
          </tbody>
        </table>
      {:else}
        <a href={playX_url} target="_blank">
          <svg
            use:qr={{
              data: playX_url,
              logo: 'https://svelte-put.vnphanquang.com/images/svelte-put-logo.svg',
              shape: 'circle',
            }}
            width="200px"
            height="200px"
          />
        </a>
      {/if}
    </div>
    <div style="display: table-cell;">
      <p>Player O</p>
      {#if playerO}
        <table>
          <tbody>
            <tr><td>ID:</td><td>{playerO.id}</td></tr>
            <tr><td>Number:</td><td>{playerO.player_number}</td></tr>
            <tr><td>Role</td><td>{playerO.role}</td></tr>
          </tbody>
        </table>
      {:else}
        <a href={playO_url} target="_blank">
          <svg
            use:qr={{
              data: playO_url,
              logo: 'https://svelte-put.vnphanquang.com/images/svelte-put-logo.svg',
              shape: 'circle',
            }}
            width="200px"
            height="200px"
          />
        </a>
      {/if}
    </div>
  </div>
</div>

<hr />

<TicTacToeBoard bind:this={refBoard} size={420} />

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
  <p>Board.status: '{board.status()}' ({board.status().length})</p>
  <p>Board.current_turn: '{board.current_turn()}'</p>
  <p>Board.winner: '{board.winner()}'</p>
  <p>Board.draw: '{board.draw()}'</p>
{/if}
<hr />
