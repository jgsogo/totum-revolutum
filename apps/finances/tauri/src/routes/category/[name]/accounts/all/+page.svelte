<script lang="ts">
  import AccountsTable from "$lib/components/AccountsTable.svelte";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();
</script>

<div class="mt-px space-y-4">
  <AccountsTable show_category={false} accounts={data.category_accounts} />
</div>
