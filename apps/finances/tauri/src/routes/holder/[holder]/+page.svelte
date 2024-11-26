<script lang="ts">
  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();
  import { Alert } from "flowbite-svelte";
</script>

<Alert>
  <span class="font-medium">Info alert!</span>
  We are seing holder {data.holder}!
</Alert>
