<script lang="ts">
  import { page } from '$app/state';
  import { onMount } from 'svelte';
  import { connectToRoom } from '$lib/api/socket';
  import { Board } from '../../../../../../../games/tic_tac_toe/webapp/src/index';
  import TicTacToeBoard from '$lib/tictactoe/components/TicTacToeBoard.svelte';
  import { placeMark } from '$lib/tictactoe/api/place_mark';
  import type { Participant as ParticipantProto } from '../../../../../../../engine/protocol/engine_pb';

  const roomUUID = page.params.roomUUID!;
  let refBoard: typeof TicTacToeBoard;
  const participant: ParticipantProto = page.data.participant;

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
    connectToRoom(
      roomUUID,
      async () => {},
      async () => {},
      on_game_update,
    );
  });

  const onCellClick = async (cellId: number) => {
    console.log(`[frontend] You clicked on cell ${cellId}`);
    if (board?.current_turn() !== participant.playerNumber) {
      alert("It's not your turn. Wait...");
      return;
    }

    try {
      let r = await placeMark(roomUUID, participant.uuid, cellId);
      console.log(`[frontend] Response: ${JSON.stringify(r)}`);
    } catch (err) {
      console.error('[frontend] Failed to send place_mark command', err);
    }
  };
</script>

<h3>= Play view =</h3>
<hr />
<p>Session/Participant-id: {participant.uuid}</p>
<p>Player number: {participant.playerNumber}</p>
<hr />
<hr />
Participant: {JSON.stringify(page.data.participant)}
<hr />

{#if board?.winner()}
  Player {board.winner()!.player()} won!
  {#if board.winner()!.player() === participant.playerNumber}
    It's you!
  {:else}
    You lost :/
  {/if}
{:else if board?.draw()}
  Draw. Noone won!
{:else if board?.current_turn() === participant.playerNumber}
  It's your turn!
{:else}
  It's NOT your turn :/
{/if}

<TicTacToeBoard {onCellClick} bind:this={refBoard} />
{#if board}
  <p>Board.status: '{board.status()}' ({board.status().length})</p>
  <p>Board.current_turn: '{board.current_turn()}'</p>
  <p>Board.winner.player: '{board.winner()?.player()}'</p>
  <p>Board.winner.line: '{board.winner()?.line()}'</p>
  <p>Board.draw: '{board.draw()}'</p>
{/if}
