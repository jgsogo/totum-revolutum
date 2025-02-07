<script lang="ts">
  import { Button, Card, Img, Modal } from "flowbite-svelte";

  import { CameraPhotoOutline, ArrowDownToBracketOutline, ArrowUpFromBracketOutline } from "flowbite-svelte-icons";
  import SnapshotForm from "$lib/forms/SnapshotForm/SnapshotForm.svelte";
  import { goToTransactionCreate } from "$lib/utils";
  import { create_snapshot } from "$lib/commands";

  import { NewSnapshot } from "$lib/forms/SnapshotForm/NewSnapshot.svelte";
  import { type Holder, Account } from "../../../models/src-js";
  import Avatar from "./Avatar.svelte";

  let { holder, account, base_media_url }: { holder: Holder; account: Account; base_media_url: string } = $props();

  let snapshotModal: boolean = $state(false);
  let newSnapshot = $state(new NewSnapshot(account, account.last_snapshot()));
  const on_snapshot = async (e: MouseEvent) => {
    e.preventDefault();
    if (newSnapshot.isValid()) {
      await create_snapshot(newSnapshot.toMessage());
      // TODO: Handle error if it fails to create the snapshot
      snapshotModal = false;
    }
  };
</script>

<Card size="xl">
  <ul class="-m-3 divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
    <li class="py-3 sm:py-3.5">
      <div class="flex items-center justify-between">
        <div class="flex min-w-0 items-center">
          <Avatar photo_url={base_media_url + account.custodian().photo()} name={account.custodian().name()} />

          <div class="ml-3">
            <p class="truncate font-medium text-gray-900 dark:text-white">
              {account.name()}
            </p>
            <span class="text-gray-500 text-sm">{account.identifier()}</span>
            <p>
              <span class="flex text-xs mr-2">
                <button class="flex hover:underline mr-2" onclick={() => (snapshotModal = true)}>
                  <CameraPhotoOutline class="w-4 h-4 mr-1" />
                  Snapshot
                </button>
                <button
                  class="flex hover:underline mr-2"
                  onclick={() => goToTransactionCreate(holder, account, undefined, account)}
                >
                  <ArrowDownToBracketOutline class="w-4 h-4 mr-1" />
                  Income
                </button>
                <button
                  class="flex hover:underline mr-2"
                  onclick={() => goToTransactionCreate(holder, account, account, undefined)}
                >
                  <ArrowUpFromBracketOutline class="w-4 h-4 mr-1" />
                  Expense
                </button>
              </span>
            </p>
          </div>
        </div>
        {#if account.last_snapshot()}
          <div class="truncate inline-flex items-center text-base font-semibold text-gray-900 dark:text-white">
            {account.last_snapshot()!.amount().amount()}
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
