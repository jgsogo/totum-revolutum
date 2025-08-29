<script lang="ts">
  import { Spring } from 'svelte/motion';

  type TicTacToeMarkProps = {
    cell_id: number;
    margin: number;
    cellSize: number;
    strokeWidth: number;
    cellClick: () => void;
  };

  let { cell_id, margin, cellSize, strokeWidth, cellClick }: TicTacToeMarkProps = $props();

  const xColor: string = '#d9534f';
  const oColor: string = '#2b9df4';
  const row = cell_id % 3;
  const column = Math.floor(cell_id / 3);
  const cx = margin + column * cellSize + cellSize / 2;
  const cy = margin + row * cellSize + cellSize / 2;

  const scale = new Spring(0, { stiffness: 0.25, damping: 0.7 });
  let mark: string = $state('');

  export function updateCell(new_mark: 'X' | 'O' | '') {
    console.log(`[frontend] updateCell(new_mark='${new_mark}') in cell_id='${cell_id}'`);
    const prev_mark = mark;
    mark = new_mark;
    if (prev_mark === '' && (new_mark === 'X' || new_mark === 'O')) {
      // animate: spring from 0 -> 1
      scale.set(0, { instant: true });
      scale.set(1);
    } else {
      // changed or removed: just set to final
      scale.set(new_mark ? 1 : 0, { instant: true });
    }
  }

  // draw X as 2 lines
  function XPaths(scale: number) {
    const pad = cellSize * 0.22;
    const x1 = cx - pad * scale,
      y1 = cy - pad * scale;
    const x2 = cx + pad * scale,
      y2 = cy + pad * scale;
    const x3 = cx - pad * scale,
      y3 = cy + pad * scale;
    const x4 = cx + pad * scale,
      y4 = cy - pad * scale;
    return { x1, y1, x2, y2, x3, y3, x4, y4 };
  }

  // draw O as circle
  function OAttrs(scale: number) {
    const r = cellSize * 0.28 * scale;
    return { cx, cy, r };
  }
</script>

<rect
  x={margin + row * cellSize}
  y={margin + column * cellSize}
  width={cellSize}
  height={cellSize}
  fill="transparent"
  onclick={cellClick}
/>

{#if mark === 'X'}
  <g>
    <line
      x1={XPaths(scale.current).x1}
      y1={XPaths(scale.current).y1}
      x2={XPaths(scale.current).x2}
      y2={XPaths(scale.current).y2}
      stroke={xColor}
      stroke-width={strokeWidth * 0.9}
      stroke-linecap="round"
      stroke-linejoin="round"
      opacity={0.98}
    />
    <line
      x1={XPaths(scale.current).x3}
      y1={XPaths(scale.current).y3}
      x2={XPaths(scale.current).x4}
      y2={XPaths(scale.current).y4}
      stroke={xColor}
      stroke-width={strokeWidth * 0.9}
      stroke-linecap="round"
      stroke-linejoin="round"
      opacity={0.98}
    />
  </g>
{:else if mark === 'O'}
  <g>
    <circle
      cx={OAttrs(scale.current).cx}
      cy={OAttrs(scale.current).cy}
      r={OAttrs(scale.current).r}
      stroke={oColor}
      stroke-width={strokeWidth * 0.9}
      fill="none"
      stroke-linecap="round"
      opacity={0.98}
    />
  </g>
{/if}
