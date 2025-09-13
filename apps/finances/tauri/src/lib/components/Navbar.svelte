<script lang="ts">
  import { DarkMode, NavBrand, Navbar} from 'flowbite-svelte';


  let {
    home_href = '/',
    version,
  }: {
    home_href: string;
    version: string;
  } = $props();

</script>

<Navbar class="mx-10 sm:mx-0">
  <NavBrand href={home_href} class="mx-10">
    <img src="/favicon.png" class="me-2.5 h-6 sm:h-8" alt="Flowbite Logo" />
    <span class="ml-px self-center text-xl font-semibold whitespace-nowrap sm:text-2xl dark:text-white"> Finances </span>
    <span class="ml-px self-center text-xs font-semibold whitespace-nowrap sm:text-xs dark:text-white"> {version}</span>
  </NavBrand>

  <div class="ms-auto flex items-center text-gray-500 sm:order-2 dark:text-gray-300">
    <DarkMode />
  </div>
</Navbar>
