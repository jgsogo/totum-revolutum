<script lang="ts">
  import "../../../app.postcss";
  import Navbar from "$lib/components/Navbar.svelte";
  import SidebarMenu from "$lib/components/SidebarMenu/SidebarMenu.svelte";
  import type { AppState, Holder } from "../../../../models/src-js";

  let { data, children } = $props();
  let app_state: AppState = data.app_state;
  let all_holders: Holder[] = data.main_context.holders;
  let active_holder: Holder = data.holder_context.holder();
  let menu = data.menu;

  let drawerHidden = $state(false);
</script>

<header
  class="fixed top-0 z-40 mx-auto w-full flex-none border-b border-gray-200 bg-white dark:border-gray-600 dark:bg-gray-800"
>
  <Navbar bind:drawerHidden {all_holders} {active_holder} home_href="/holder/{active_holder.pk}" {app_state} />
</header>
<div class="overflow-hidden lg:flex">
  <SidebarMenu bind:drawerHidden {menu} {app_state} />

  <div class="relative h-full w-full overflow-y-auto lg:ml-64 pt-[70px]">
    <main class="p-4">
      {@render children()}
    </main>
  </div>
</div>
