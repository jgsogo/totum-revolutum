<script lang="ts">
    import { Avatar, Dropdown, DropdownDivider, DropdownHeader, DropdownItem, Spinner } from "flowbite-svelte";
    import { holders } from "$lib/commands";
    import type { Holder } from "$lib/models/Holder";

    const classified_holders = async (): Promise<{
      personas: Holder[];
      companies: Holder[];
    }> => {
      const all_holders = await holders();
      const personas = all_holders.filter((it) => !it.is_company);
      const companies = all_holders.filter((it) => it.is_company);
      return { personas: personas, companies: companies };
    };

    // let activeClass = 'text-green-500 dark:text-green-300 hover:text-green-700 dark:hover:text-green-500';
  </script>

  <button class="ms-3 rounded-full ring-gray-400 focus:ring-4 dark:ring-gray-600">
    <Avatar size="sm" src="https://flowbite-admin-dashboard.vercel.app/images/users/bonnie-green.png" tabindex={0} />
  </button>
  {#await classified_holders()}
    <Spinner />
  {:then all_holders}
    <Dropdown placement="bottom-end">
      {#each all_holders.personas as persona}
        <DropdownItem href="/holder/{persona.pk}">{@html persona}</DropdownItem>
      {/each}
      <DropdownDivider />
      {#each all_holders.companies as company}
        <DropdownItem href="/holder/{company.pk}">{@html company}</DropdownItem>
      {/each}
    </Dropdown>
  {:catch error}
    <p style="color: red">{error.message}</p>
  {/await}
