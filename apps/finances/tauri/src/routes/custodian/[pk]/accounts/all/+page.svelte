<script lang="ts">
  import AccountsTable from '$lib/components/AccountsTable.svelte';

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();
</script>

Custodian detail card for: {data.custodian.name()}
<AccountsTable show_custodian={false} accounts={data.custodian_accounts} holder={data.app_state.holder()} />
