<script lang="ts">
  import "../../../app.postcss";
  import Navbar from "$lib/components/Navbar.svelte";
  import SidebarMenu from "$lib/components/SidebarMenu/SidebarMenu.svelte";
  let { data, children } = $props();

  let drawerHidden = $state(false);
</script>

<header
  class="fixed top-0 z-40 mx-auto w-full flex-none border-b border-gray-200 bg-white dark:border-gray-600 dark:bg-gray-800"
>
  <Navbar
    bind:drawerHidden
    holders={data.holders}
    bind:active_holder={data.holder}
    home_href="/holder/{data.holder.pk}"
    base_media_url={data.base_media_url}
  />
</header>
<div class="overflow-hidden lg:flex">
  <SidebarMenu bind:drawerHidden menu={data.menu}/>

  <div class="relative h-full w-full overflow-y-auto lg:ml-64 pt-[70px]">
    <main class="p-4">
      {@render children()}
    </main>
  </div>
</div>
