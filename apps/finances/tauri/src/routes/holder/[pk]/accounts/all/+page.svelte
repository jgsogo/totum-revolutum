<script lang="ts">
  import AccountsTable from "$lib/components/AccountsTable.svelte";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();
</script>

<div class="mt-px space-y-4">
  <AccountsTable show_holders={false} accounts={data.holder_accounts} holder={data.app_state.holder()}/>
</div>
