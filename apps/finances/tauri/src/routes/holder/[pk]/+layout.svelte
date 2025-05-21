<script lang="ts">
  import { Breadcrumb, BreadcrumbItem, Heading } from 'flowbite-svelte';
  let { data, children } = $props();
</script>

<main class="p-4">
  <div class="grid grid-cols-1 space-y-2 dark:bg-gray-900">
    <div class="col-span-full xl:mb-0">
      <Breadcrumb class="mb-6">
        <BreadcrumbItem home>Home</BreadcrumbItem>
        <BreadcrumbItem class="hover:text-primary-600 inline-flex items-center text-gray-700 dark:text-gray-300 dark:hover:text-white"
          >Holder</BreadcrumbItem
        >
        <BreadcrumbItem>{data.holder_context.holder().name()}</BreadcrumbItem>
      </Breadcrumb>

      <Heading tag="h1" class="text-xl font-semibold text-gray-900 sm:text-2xl dark:text-white"
        >Accounts for {data.holder_context.holder().name()}</Heading
      >
    </div>
    <div class="col-span-full space-y-4">
      {@render children()}
    </div>
  </div>
</main>
