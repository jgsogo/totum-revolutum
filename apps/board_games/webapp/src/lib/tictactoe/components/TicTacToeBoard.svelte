<script lang="ts">
  import TicTacToeMark from './TicTacToeMark.svelte';

  type TicTacToeBoardProps = {
    size?: number;
    strokeWidth?: number;
    gridColor?: string;
    onCellClick?: (cell_id: number) => void;
  };

  let { size = 360, strokeWidth = 8, gridColor = '#222', onCellClick = (_: number) => {} }: TicTacToeBoardProps = $props();

  const margin = size * 0.02; // small outer padding
  const innerSize = size - margin * 2;
  const cellSize = innerSize / 3;

  const cells = Array.from({ length: 9 }, (_, i: number) => {
    return { cell_id: i, instance: typeof TicTacToeMark, margin, cellSize, strokeWidth };
  });

  export function updateBoard(nextBoard: string[]) {
    console.log(`[frontend] updateBoard(nextBoard='${nextBoard}')`);
    nextBoard.forEach((nextElem, idx) => {
      let cell = cells[idx];
      cell.instance.updateCell(nextElem);
    });
  }

  let strike: { x1: number; y1: number; x2: number; y2: number } | undefined = $state();
  export function winningLine(cellStart: number, cellEnd: number) {
    console.log(`[frontend] winningLine(cellStart='${cellStart}', cellEnd='${cellEnd}')`);
    const row_ini = Math.floor(cellStart / 3);
    const column_ini = cellStart % 3;
    const cx_ini = margin + column_ini * cellSize + cellSize / 2;
    const cy_ini = margin + row_ini * cellSize + cellSize / 2;

    const row_end = Math.floor(cellEnd / 3);
    const column_end = cellEnd % 3;
    const cx_end = margin + column_end * cellSize + cellSize / 2;
    const cy_end = margin + row_end * cellSize + cellSize / 2;

    if (row_ini === row_end) {
      // row
      strike = {
        x1: cx_ini - cellSize / 2,
        y1: cy_ini,
        x2: cx_end + cellSize / 2,
        y2: cy_end,
      };
    } else if (column_ini === column_end) {
      // column
      strike = {
        x1: cx_ini,
        y1: cy_ini - cellSize / 2,
        x2: cx_end,
        y2: cy_end + cellSize / 2,
      };
    } else if (column_ini < column_end) {
      // diagonal
      strike = {
        x1: cx_ini - cellSize / 2,
        y1: cy_ini - cellSize / 2,
        x2: cx_end + cellSize / 2,
        y2: cy_end + cellSize / 2,
      };
    } else if (column_ini > column_end) {
      // diagonal
      strike = {
        x1: cx_ini + cellSize / 2,
        y1: cy_ini - cellSize / 2,
        x2: cx_end - cellSize / 2,
        y2: cy_end + cellSize / 2,
      };
    }
  }
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
      <TicTacToeMark
        cell_id={cell.cell_id}
        margin={cell.margin}
        cellSize={cell.cellSize}
        strokeWidth={cell.strokeWidth}
        cellClick={() => onCellClick(cell.cell_id)}
        bind:this={cell.instance}
      />
    {/each}

    <!-- Strike-through line -->
    {#if strike}
      <line x1={strike.x1} y1={strike.y1} x2={strike.x2} y2={strike.y2} stroke="green" stroke-width="6" stroke-linecap="round" />
    {/if}
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
