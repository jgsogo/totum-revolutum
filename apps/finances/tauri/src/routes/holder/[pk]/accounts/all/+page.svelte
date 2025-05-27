<script lang="ts">
  import AccountsTable from '$lib/components/AccountsTable.svelte';

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();
</script>

Holder detail card for: {data.holder_context.holder().name()}
<AccountsTable show_holders={false} accounts={data.holder_accounts} app_state={data.app_state}/>
