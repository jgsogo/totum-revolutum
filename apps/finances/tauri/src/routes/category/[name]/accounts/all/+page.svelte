<script lang="ts">
  import AccountsTable from '$lib/components/AccountsTable.svelte';

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();
</script>

Category detail card for: {data.category}
<AccountsTable show_category={false} accounts={data.category_accounts} holder={data.app_state.holder()} />
