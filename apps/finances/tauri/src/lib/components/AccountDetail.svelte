<script lang="ts">
  import { Avatar, Card, Img, Modal } from "flowbite-svelte";
  import type { Account } from "$lib/models/Account";
  import type { Custodian } from "$lib/models/Custodian";
  import type { Snapshot } from "$lib/models/Snapshot";
  import { CameraPhotoOutline, ArrowDownToBracketOutline, ArrowUpFromBracketOutline } from "flowbite-svelte-icons";
  import SnapshotForm from "$lib/forms/SnapshotForm.svelte";
  import { goToTransactionCreate } from "$lib/utils";
  import type { Holder } from "$lib/models/Holder";
  import { create_snapshot } from "$lib/commands";
  import { invalidate } from "$app/navigation";

  let {
    holder = $bindable(),
    account = $bindable(),
    base_media_url,
    last_snapshot = $bindable(),
  }: { holder: Holder; account: Account; base_media_url: string; last_snapshot?: Snapshot } = $props();

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

  let snapshotModal: boolean = $state(false);
  const on_snapshot = async (date_value: Date, amount: number, quantity?: number, unit_value?: number) =>  {
      await create_snapshot(account, date_value, amount, quantity, unit_value);
      await invalidate("invalidate:account");
      snapshotModal = false;
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
            <p>
              <span class="flex text-xs mr-2">
                <button class="flex hover:underline mr-2" onclick={() => (snapshotModal = true)}>
                  <CameraPhotoOutline class="w-4 h-4 mr-1" />
                  Snapshot
                </button>
                <button
                  class="flex hover:underline mr-2"
                  onclick={() => goToTransactionCreate(holder, undefined, account)}
                >
                  <ArrowDownToBracketOutline class="w-4 h-4 mr-1" />
                  Income
                </button>
                <button
                  class="flex hover:underline mr-2"
                  onclick={() => goToTransactionCreate(holder, account, undefined)}
                >
                  <ArrowUpFromBracketOutline class="w-4 h-4 mr-1" />
                  Expense
                </button>
              </span>
            </p>
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

<Modal bind:open={snapshotModal} size="xs" class="w-full h-full" autoclose={false}>
  <SnapshotForm {account} {last_snapshot} {on_snapshot} />
</Modal>
