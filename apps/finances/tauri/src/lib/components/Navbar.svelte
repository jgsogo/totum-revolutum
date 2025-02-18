<script lang="ts">
  import HolderSelector from "./HolderSelector.svelte";
  import { DarkMode, Dropdown, DropdownItem, NavBrand, NavHamburger, NavLi, NavUl, Navbar, Search } from "flowbite-svelte";
  import { invalidateAll } from "$app/navigation";
  import { ChevronDownOutline, RefreshOutline } from "flowbite-svelte-icons";
  import type { AppState, Holder } from "../../../models/src-js/index";

  let {
    fluid = true,
    drawerHidden = $bindable(),
    list = false,
    // all_holders,
    // active_holder,
    home_href = "/",
    app_state,
  }: {
    fluid: boolean;
    drawerHidden: boolean;
    list: boolean;
    // all_holders: Holder[];
    // active_holder: Holder;
    home_href: string;
    app_state: AppState;
  } = $props();
</script>

<Navbar {fluid} class="text-black" color="default" let:NavContainer>
  <NavHamburger onClick={() => (drawerHidden = !drawerHidden)} class="m-0 me-3 md:block lg:hidden" />

  <NavBrand href={home_href} class={list ? "w-40" : "lg:w-60"}>
    <img src="/favicon.png" class="me-2.5 h-6 sm:h-8" alt="Flowbite Logo" />
    <span class="ml-px self-center whitespace-nowrap text-xl font-semibold dark:text-white sm:text-2xl"> Finances </span>
  </NavBrand>

  <div class="hidden lg:block lg:ps-3">
    {#if list}
      <NavUl class="ml-2" activeUrl="/" activeClass="text-primary-600 dark:text-primary-500">
        <NavLi href="/">Home</NavLi>
        <NavLi href="#top">Messages</NavLi>
        <NavLi href="#top">Profile</NavLi>
        <NavLi href="#top">Settings</NavLi>
        <NavLi class="cursor-pointer">
          Dropdown
          <ChevronDownOutline class="ms-0 inline" />
        </NavLi>
        <Dropdown class="z-20 w-44">
          <DropdownItem href="#top">Item 1</DropdownItem>
          <DropdownItem href="#top">Item 2</DropdownItem>
          <DropdownItem href="#top">Item 3</DropdownItem>
        </Dropdown>
      </NavUl>
    {:else}
      <form>
        <Search size="md" class="mt-1 w-96 border focus:outline-none" />
      </form>
    {/if}
  </div>

  <div class="ms-auto flex items-center text-gray-500 dark:text-gray-400 sm:order-2">
    <button
      onclick={invalidateAll}
      class="ms-3 dark:ring-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none rounded-lg p-2.5"
    >
      <RefreshOutline />
    </button>

    <DarkMode />
    <!-- <HolderSelector {active_holder} {all_holders} {app_state} /> -->
  </div>
</Navbar>
