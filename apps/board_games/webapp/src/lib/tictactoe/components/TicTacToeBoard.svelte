<script lang="ts">
  import { onMount } from 'svelte';
  import { Spring } from 'svelte/motion'; // Svelte 5 runes friendly
  import TicTacToeMark from './TicTacToeMark.svelte';

  // Props
  export let size: number = 360; // overall px size of the board
  export let strokeWidth: number = 8;
  export let gridColor: string = '#222';

  // internal board model: array of 9 values: "X", "O", or "" (empty)
  // default empty board
  let board: string[] = Array(9).fill('');

  const cells = Array.from({ length: 9 }, (_, i: number) => {
    return { cell_id: i, size, instance: typeof TicTacToeMark, other: "u" };
  });
  console.log(`[frontend] cells: ${JSON.stringify(cells)}`);

  const margin = size * 0.02; // small outer padding
  const innerSize = size - margin * 2;
  const cellSize = innerSize / 3;

  // animation springs for each cell (scale)
  const scales = Array.from({ length: 9 }, () => new Spring(0, { stiffness: 0.25, damping: 0.7 }));

  export function updateBoard(nextBoard: string[]) {
    console.log(`[frontend] updateBoard(nextBoard='${nextBoard}')`);
    nextBoard.forEach((nextElem, idx) => {
        let cell = cells[idx];
        cell.instance.updateCell(nextElem);
    });
    // // detect newly placed marks to animate only those cells
    // for (let i = 0; i < 9; i++) {
    //   const prev = board[i] || '';
    //   const next = nextBoard[i] || '';
    //   if (prev === '' && (next === 'X' || next === 'O')) {
    //     // animate: spring from 0 -> 1
    //     scales[i].set(0);
    //     // small timeout to allow spring value reset to take effect
    //     setTimeout(() => scales[i].set(1), 10);
    //   } else if (prev !== next) {
    //     // changed or removed: just set to final
    //     scales[i].set(next ? 1 : 0);
    //   }
    // }
    // board = nextBoard.slice(); // copy
  }

  // Expose a convenience to set board directly from parent at mount
  export function setBoard(b: string[]) {
    if (Array.isArray(b) && b.length === 9) {
      board = b.slice();
      // set existing springs to 1 for existing marks, 0 for empty
      for (let i = 0; i < 9; i++) scales[i].set(board[i] ? 1 : 0);
    }
  }

  // When component mounts, ensure springs match board
  onMount(() => {
    console.log(`[frontend] Board mounted with size='${size}', strokeWidth='${strokeWidth}'`)
    for (let i = 0; i < 9; i++) scales[i].set(board[i] ? 1 : 0);
  });
</script>

<div class="board-wrapper" role="group" aria-label="Tic Tac Toe board">
  <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} aria-hidden="false" role="img">
    <!-- background -->
    <rect x="0" y="0" width={size} height={size} fill="transparent" />

    <!-- grid lines -->
    {#each [1, 2] as idx}
      <!-- vertical -->
      <line
        x1={margin + idx * cellSize}
        y1={margin}
        x2={margin + idx * cellSize}
        y2={margin + innerSize}
        stroke={gridColor}
        stroke-width={strokeWidth}
        stroke-linecap="round"
        opacity="0.95"
      />
      <!-- horizontal -->
      <line
        x1={margin}
        y1={margin + idx * cellSize}
        x2={margin + innerSize}
        y2={margin + idx * cellSize}
        stroke={gridColor}
        stroke-width={strokeWidth}
        stroke-linecap="round"
        opacity="0.95"
      />
    {/each}

    <!-- marks -->
    {#each cells as cell}
      <TicTacToeMark cell_id={cell.cell_id} size={cell.size} bind:this={cell.instance}  />
    {/each}
  </svg>
</div>

<style>
  .board-wrapper {
    display: inline-block;
    user-select: none;
    -webkit-user-select: none;
  }
  .status {
    font-family:
      system-ui,
      -apple-system,
      'Segoe UI',
      Roboto,
      'Helvetica Neue',
      Arial;
    margin-top: 0.5rem;
    font-size: 0.9rem;
    color: #444;
  }
  .meta {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    margin-top: 0.4rem;
  }
  .legend {
    font-size: 0.85rem;
    color: #666;
  }
</style>
