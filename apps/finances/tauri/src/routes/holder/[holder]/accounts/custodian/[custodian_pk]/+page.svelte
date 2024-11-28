<script lang="ts">
    import AccountsTable from "$lib/components/AccountsTable.svelte";

    /** @type {{ data: import('./$types').PageData }} */
    let { data } = $props();
  </script>

  <div class="mt-px space-y-4">

    {data.custodian}
    <img src="{data.base_media_url}{data.custodian.photo}" />

    <AccountsTable show_custodian={false} bind:accounts={data.accounts_for_custodian} />
  </div>
