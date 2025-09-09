<script lang="ts">
  import { page } from '$app/state';
  import { DarkMode, NavBrand, Navbar, Search } from 'flowbite-svelte';
  import type { Holder } from '../../../models/src-js';

  let {
    drawerHidden = $bindable(),
    home_href = '/',
    holder,
    version,
  }: {
    drawerHidden: boolean;
    home_href: string;
    holder?: Holder;
    version: string;
  } = $props();

  let activeUrl = $derived(page.url.pathname);
</script>

<Navbar class="mx-10 sm:mx-0">
  <NavBrand href="/" class="mx-10">
    <img src="/favicon.png" class="me-2.5 h-6 sm:h-8" alt="Flowbite Logo" />
    <span class="ml-px self-center text-xl font-semibold whitespace-nowrap sm:text-2xl dark:text-white"> Finances </span>
    <span class="ml-px self-center text-xs font-semibold whitespace-nowrap sm:text-xs dark:text-white"> {version}</span>
  </NavBrand>

  <div class="hidden lg:block lg:ps-3">
    <form>
      <Search size="md" class="mt-1 w-96 border focus:outline-none" />
    </form>
  </div>

  <div class="ms-auto flex items-center text-gray-500 sm:order-2 dark:text-gray-300">
    <DarkMode />
  </div>
</Navbar>
