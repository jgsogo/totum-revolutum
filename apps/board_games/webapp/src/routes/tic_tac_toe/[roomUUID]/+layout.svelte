<script>
	import { page } from '$app/state';

	let { children } = $props();
	const roomUUID = page.params.roomUUID;
</script>

<h1>=== Tic tac toe ===</h1>
<h2>== Room UUID: {roomUUID} ==</h2>

{@render children()}
