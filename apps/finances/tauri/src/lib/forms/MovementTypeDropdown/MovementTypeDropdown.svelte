<script lang="ts">
  import type { MovementType } from "../../../../models/src-js";
  import MultilevelDropdown from "../MultilevelDropdown.svelte";

  let {
    movementtype = $bindable(),
    all_movementtypes,
  }: {
    movementtype: MovementType | undefined;
    all_movementtypes: MovementType[]; // TODO: Document that all movement types should have breadcrumbs already resolved!
  } = $props();

  const breadcrumbs_group_size = 2;

  const groupBy = (it: MovementType) => it.breadcrumb()!.slice(0, breadcrumbs_group_size).toString();

  const labelInGroup = (it: MovementType) => it.breadcrumb()!.slice(breadcrumbs_group_size).toString();

  const equalItems = (lhs: MovementType, rhs: MovementType) => {
    return lhs.pk() === rhs.pk();
  };

  const labelForItem = (it: MovementType) => it.breadcrumb()!.toString();
</script>

<!-- Common fields -->
<MultilevelDropdown bind:value={movementtype} items={all_movementtypes} {labelForItem} {equalItems} {groupBy} {labelInGroup} on:change />
