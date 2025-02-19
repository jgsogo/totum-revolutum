<script lang="ts">
  import "../app.postcss";
  import Navbar from "$lib/components/Navbar.svelte";
  import AccountsMenu from "$lib/components/AccountsMenu.svelte";
  import type { AppState, MainContext } from "../../models/src-js";
  import { do_backup } from "$lib/commands";
  import { Button } from "flowbite-svelte";

  let { data, children } = $props();
  let app_state: AppState = data.app_state;
  let main_context: MainContext = data.main_context;

  let drawerHidden = $state(false);

  let backup_info: string = $state("not run yet");
  const run_backup = async () => {
    backup_info = await do_backup();
  };
</script>

<header
  class="fixed top-0 z-40 mx-auto w-full flex-none border-b border-gray-200 bg-white dark:border-gray-600 dark:bg-gray-800"
>
  <Navbar bind:drawerHidden home_href="/" />
</header>
<div class="overflow-hidden lg:flex">
  <AccountsMenu bind:drawerHidden accounts={main_context.accounts()} holders={main_context.holders()} {app_state} />

  <div class="relative h-full w-full overflow-y-auto lg:ml-64 pt-[70px]">
    <main class="p-4">
      <Button onclick={run_backup}>Run backup</Button>
      {backup_info}

      {@render children()}
    </main>
  </div>
</div>
