<script lang="ts">
  import AccountsTable from '$lib/components/AccountsTable.svelte';

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();
</script>

Category detail card for: {data.category}
<AccountsTable show_category={false} accounts={data.category_accounts} app_state={data.app_state}/>
