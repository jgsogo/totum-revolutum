<script lang="ts">
  import { page } from '$app/stores';
  import { player_id } from '$lib/player_id';
</script>

<h1>Admin view</h1>
<hr/>

<p>Player ID: {JSON.stringify(player_id)}</p>
<h1>Room: {$page.params.roomId}</h1>
