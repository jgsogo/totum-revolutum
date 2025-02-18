<script lang="ts">
  let { data, children } = $props();
</script>

<div class="mt-px space-y-4">
  Custodian detail card for: {data.custodian.name()}

  {@render children()}
</div>
