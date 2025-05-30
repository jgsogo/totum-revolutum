<script lang="ts">
  import { Card, Modal } from 'flowbite-svelte';

  import { CameraPhotoOutline, ArrowDownToBracketOutline, ArrowUpFromBracketOutline } from 'flowbite-svelte-icons';
  import SnapshotForm from '$lib/forms/SnapshotForm/SnapshotForm.svelte';
  import { goToTransactionCreate } from '$lib/utils';
  import { create_snapshot } from '$lib/commands';

  import { NewSnapshot } from '$lib/forms/SnapshotForm/NewSnapshot.svelte';
  import { Account, AppState, Movement } from '../../../models/src-js';
  import Avatar from './Avatar.svelte';
  import LastSnapshotMoneyString from './LastSnapshotMoneyString.svelte';

  let { app_state, account, movements }: { app_state: AppState; account: Account; movements?: Movement[] | undefined } = $props();

  let snapshotModal: boolean = $state(false);

  function on_new_snapshot(newSnapshot: NewSnapshot) {
    create_snapshot(newSnapshot.toMessage())
      .then(() => {
        console.log('New snapshot created');
        // TODO: Refresh movements!
      })
      .catch((error) => {
        console.error('Error:', error);
        // TODO: Show some kind of banner
      });
  }
</script>

<Card size="xl" class="p-4 sm:p-6">
  <ul class="-m-3 divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
    <li class="py-3 sm:py-3.5">
      <div class="flex items-center justify-between">
        <div class="flex min-w-0 items-center">
          <Avatar photo_url={ app_state.base_url() + app_state.base_media_url() + account.custodian().photo()} name={account.custodian().name()} />

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
                <button class="flex hover:underline mr-2" onclick={() => goToTransactionCreate(account, undefined, account)}>
                  <ArrowDownToBracketOutline class="w-4 h-4 mr-1" />
                  Income
                </button>
                <button class="flex hover:underline mr-2" onclick={() => goToTransactionCreate(account, account, undefined)}>
                  <ArrowUpFromBracketOutline class="w-4 h-4 mr-1" />
                  Expense
                </button>
              </span>
            </p>
          </div>
        </div>
        {#if account.last_snapshot()}
          <div class="truncate inline-flex items-center text-base font-semibold text-gray-900 dark:text-white">
            <LastSnapshotMoneyString {app_state} {account} {movements} tooltip={true} />
          </div>
        {/if}
      </div>
    </li>
  </ul>
</Card>

<Modal bind:open={snapshotModal} size="md">
  <SnapshotForm {account} handleSubmit={on_new_snapshot} method={null} />
</Modal>
