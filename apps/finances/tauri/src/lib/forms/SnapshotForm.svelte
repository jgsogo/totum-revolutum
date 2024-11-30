<script lang="ts">
  import { Button, Input, Label, ButtonGroup, InputAddon, Datepicker } from "flowbite-svelte";

  let { ccy, on_snapshot }: { ccy: string, on_snapshot: (amount: number, date_value: Date) => void | string } = $props();

  let snapshotDate: Date = $state(new Date());
  let snapshotAmount: number | null = $state(null);

  const submitSnapshot = () => {
    // TODO: Validation:
    //  * Date lower or equal than today
    //  * Amount makes sense
    if (snapshotAmount ) {
        let err = on_snapshot(snapshotAmount, snapshotDate);
        // TODO: Manage error
    }
  };
</script>

<form class="flex flex-col space-y-6" action="#">
  <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Add snapshot</h3>
  <Label class="space-y-2">
    <span>Date value: {snapshotDate.toLocaleDateString()}</span>
    <Datepicker required inline bind:value={snapshotDate}/>
  </Label>
  <Label class="space-y-2">
    <span>Amount</span>
    <ButtonGroup class="w-full">
      <InputAddon>{ccy}</InputAddon>
      <Input required id="snapshot-amount" placeholder="1234,56" bind:value={snapshotAmount} />
    </ButtonGroup>
  </Label>
  <Button type="submit" onclick={submitSnapshot} class="w-full1">Submit</Button>
</form>
