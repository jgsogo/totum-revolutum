<script lang="ts">
  import { Avatar, Button, Card, Img, Modal } from "flowbite-svelte";
  import type { Account } from "$lib/models/Account";
  import type { Custodian } from "$lib/models/Custodian";
  import type { Snapshot } from "$lib/models/Snapshot";
  import { CameraPhotoOutline, ArrowDownToBracketOutline, ArrowUpFromBracketOutline } from "flowbite-svelte-icons";
  import SnapshotForm from "$lib/forms/SnapshotForm/SnapshotForm.svelte";
  import { goToTransactionCreate } from "$lib/utils";
  import type { Holder } from "$lib/models/Holder";
  import { create_snapshot } from "$lib/commands";
  import { invalidate } from "$app/navigation";
  import { NewSnapshot } from "$lib/forms/SnapshotForm/NewSnapshot.svelte";

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
  let newSnapshot = $state(new NewSnapshot(account));
  const on_snapshot = async (e: MouseEvent) => {
    e.preventDefault();
    if (newSnapshot.isValid()) {
      await create_snapshot(newSnapshot);
      // TODO: Handle error if it fails to create the snapshot
      await invalidate("invalidate:account");
      snapshotModal = false;
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
  <form>
    <SnapshotForm bind:snapshot={newSnapshot} />
    <Button disabled={newSnapshot.isValid() ? false : true} onclick={on_snapshot} type="submit" class="w-full, mt-4">
      Submit
    </Button>
  </form>
</Modal>
