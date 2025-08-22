<script lang="ts">
    import { page } from '$app/state';

	let { children } = $props();
    const roomUUID = page.params.roomId;
</script>

<h2>== Room UUID: {roomUUID} ==</h2>

{@render children()}
