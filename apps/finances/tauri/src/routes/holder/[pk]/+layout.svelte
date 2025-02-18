<script lang="ts">
  let { data, children } = $props();
</script>

<div class="mt-px space-y-4">
  Holder detail card for: {data.holder_context.holder().name()}

  {@render children()}
</div>
