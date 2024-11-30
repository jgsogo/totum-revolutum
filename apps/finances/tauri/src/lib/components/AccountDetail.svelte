<script lang="ts">
  import { Avatar, Card, Img } from "flowbite-svelte";
  import type { Account } from "$lib/models/Account";
  import type { Custodian } from "$lib/models/Custodian";
  import type { Snapshot } from "$lib/models/Snapshot";
  import type { Movement } from "$lib/models/Movement";

  let {
    account = $bindable(),
    base_media_url,
    last_snapshot = $bindable(),
  }: { account: Account; base_media_url: string; last_snapshot?: Snapshot } = $props();

  const initials = (custodian: Custodian): string => {
    let words = custodian.name.split(/\s/);
    if (words.length == 1) {
      return custodian.name.substring(0, 3);
    } else {
      let acronym = words
        .map((word) => word.replace("(", ""))
        .reduce((response, word) => (response += word.slice(0, 1)), "");
      return acronym.substring(0, 3);
    }
  };
</script>

<Card size="xl">
  <ul class="-m-3 divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
    <li class="py-3 sm:py-3.5">
      <div class="flex items-center justify-between">
        <div class="flex min-w-0 items-center">
          {#if account.custodian.photo}
            <Img size="w-20" src="{base_media_url}{account.custodian.photo}" />
            <!-- <Avatar title={account.custodian.name} src="{base_media_url}{account.custodian.photo}"
                >{account.custodian.name}</Avatar
              > -->
          {:else}
            <Avatar size="lg" title={account.custodian.name}>{initials(account.custodian)}</Avatar>
          {/if}

          <div class="ml-3">
            <p class="truncate font-medium text-gray-900 dark:text-white">
              {account.name}
            </p>
            <span class="text-gray-500 text-sm">{account.identifier}</span>
          </div>
        </div>
        {#if last_snapshot}
          <div class="truncate inline-flex items-center text-base font-semibold text-gray-900 dark:text-white">
            {last_snapshot}
          </div>
        {/if}
      </div>
    </li>
  </ul>
</Card>
