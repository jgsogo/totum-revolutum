<script lang="ts">
  import { page } from '$app/state';
  import { onMount } from 'svelte';
  import { connectToRoom } from '$lib/api/socket';
  import { Board } from '../../../../../../../games/tic_tac_toe/webapp/src/index';
  import TicTacToeBoard from '$lib/tictactoe/components/TicTacToeBoard.svelte';
  import { placeMark } from '$lib/tictactoe/api/place_mark';

  const roomUUID = page.params.roomUUID!;
  let refBoard: typeof TicTacToeBoard;

  let game_data = $state({});
  let board: Board | undefined = $state();
  const on_game_update = async (payload: JSON) => {
    console.log(`[frontend] Get game update for room ${roomUUID}`);
    game_data = payload;
    board = Board.create_from_array(game_data.state_data);
    refBoard.updateBoard([...board.status()]);
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
    console.log(`You clicked on cell ${cellId}`);
    try {
      let r = await placeMark(roomUUID, page.data.session_id, cellId);
      console.log(`[frontend] Response: ${JSON.stringify(r)}`);
    } catch (err) {
      console.error('[frontend] Failed to send place_mark command', err);
    }
  };
</script>

<h3>= Play view =</h3>
<hr />
Session-id: {page.data.session_id}
<hr />
<hr />
Participant: {JSON.stringify(page.data.participant)}
<hr />

<TicTacToeBoard {onCellClick} bind:this={refBoard} />
