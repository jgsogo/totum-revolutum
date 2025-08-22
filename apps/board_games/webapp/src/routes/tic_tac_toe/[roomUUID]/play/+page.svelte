<script lang="ts">
    import { page } from '$app/state';
  import { stringify } from 'querystring';

</script>

<h3>= Play view =</h3>
<hr/>
Session-id: {page.data.session_id}
<hr/>
<hr/>
Participant: {JSON.stringify( page.data.participant)}
<hr/>
