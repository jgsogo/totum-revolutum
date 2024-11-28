<script lang="ts">
    import AccountsTable from "$lib/components/AccountsTable.svelte";

    /** @type {{ data: import('./$types').PageData }} */
    let { data } = $props();
  </script>

  <div class="mt-px space-y-4">
    <AccountsTable bind:accounts={data.savings_accounts_list} />
  </div>
