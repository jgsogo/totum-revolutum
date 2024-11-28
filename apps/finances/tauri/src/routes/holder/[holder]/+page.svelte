<script lang="ts">
  import AccountsTable from "$lib/components/AccountsTable.svelte";

  /** @type {{ data: import('./$types').PageData }} */
  let { data } = $props();
</script>

<main class="p-4">
  <div class="mt-px space-y-4">
    <AccountsTable bind:accounts={data.all_accounts_list} />
  </div>
</main>
