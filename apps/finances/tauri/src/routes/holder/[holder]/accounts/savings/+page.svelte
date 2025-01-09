<script lang="ts">
    import AccountsTable from "$lib/components/AccountsTable.svelte";

    /** @type {{ data: import('./$types').PageData }} */
    let { data } = $props();
  </script>

  <div class="mt-px space-y-4">
    <AccountsTable holder={data.holder_context.holder()} bind:accounts={data.all_savings_accounts_for_holder}/>
  </div>
