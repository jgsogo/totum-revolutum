<script lang="ts">
  import { Spring } from 'svelte/motion'; // Svelte 5 runes friendly

  let { cell_id, size }: { cell_id: number; size: number } = $props();

  const margin = size * 0.02; // small outer padding
  const innerSize = size - margin * 2;
  const cellSize = innerSize / 3;
  const xColor: string = '#d9534f';
  const oColor: string = '#2b9df4';
  const strokeWidth: number = 8;

  const scale = new Spring(0, { stiffness: 0.25, damping: 0.7 });
  // svelte-ignore non_reactive_update
  let mark: string = $state('');

  export function updateCell(new_mark: 'X' | 'O' | '') {
    console.log(`[frontend] updateCell(new_mark='${new_mark}') in cell_id='${cell_id}' (size='${size}')`);
    const prev_mark = mark;
    mark = new_mark;
    if (prev_mark === '' && (new_mark === 'X' || new_mark === 'O')) {
      // animate: spring from 0 -> 1
      scale.set(0);
      // small timeout to allow spring value reset to take effect
      setTimeout(() => scale.set(1), 10);
    } else {
      // changed or removed: just set to final
      scale.set(new_mark ? 1 : 0);
    }
  }

  // Coordinates for drawing helpers
  function cellCenter(i: number) {
    const r = Math.floor(i / 3);
    const c = i % 3;
    const cx = margin + c * cellSize + cellSize / 2;
    const cy = margin + r * cellSize + cellSize / 2;
    return { cx, cy };
  }

  // draw X as 2 lines
  function XPaths(i: number, scale: number) {
    const { cx, cy } = cellCenter(i);
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
  function OAttrs(i: number, scale: number) {
    const { cx, cy } = cellCenter(i);
    const r = cellSize * 0.28 * scale;
    return { cx, cy, r };
  }
</script>

{#if mark === 'X'}
  <g>
    <line
      x1={XPaths(cell_id, scale.current).x1}
      y1={XPaths(cell_id, scale.current).y1}
      x2={XPaths(cell_id, scale.current).x2}
      y2={XPaths(cell_id, scale.current).y2}
      stroke={xColor}
      stroke-width={strokeWidth * 0.9}
      stroke-linecap="round"
      stroke-linejoin="round"
      opacity={0.98}
    />
    <line
      x1={XPaths(cell_id, scale.current).x3}
      y1={XPaths(cell_id, scale.current).y3}
      x2={XPaths(cell_id, scale.current).x4}
      y2={XPaths(cell_id, scale.current).y4}
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
      cx={OAttrs(cell_id, scale.current).cx}
      cy={OAttrs(cell_id, scale.current).cy}
      r={OAttrs(cell_id, scale.current).r}
      stroke={oColor}
      stroke-width={strokeWidth * 0.9}
      fill="none"
      stroke-linecap="round"
      opacity={0.98}
    />
  </g>
{/if}
