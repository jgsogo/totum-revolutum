<script lang="ts">
  import '../app.css';
  import Navbar from '$lib/components/Navbar.svelte';
  import AccountsMenu from '$lib/components/AccountsMenu.svelte';
  import type { AppState, MainContext } from '../../models/src-js';
  import { PUBLIC_VERSION } from '$env/static/public';

  let { data, children } = $props();
  let app_state: AppState = data.app_state;
  let main_context: MainContext = data.main_context;

  let drawerHidden = $state(false);
</script>

<header class="fixed top-0 z-40 mx-auto w-full flex-none border-b border-gray-200 bg-white dark:border-gray-600 dark:bg-gray-800">
  <Navbar bind:drawerHidden home_href="/" holder={app_state.holder()} version={PUBLIC_VERSION}/>
</header>

<div class="overflow-hidden lg:flex">
  <AccountsMenu bind:drawerHidden accounts={main_context.accounts()} holders={main_context.holders()} {app_state} />
  <div class="relative h-full w-full overflow-y-auto pt-[70px] lg:ml-64">
    {@render children()}
  </div>
</div>
