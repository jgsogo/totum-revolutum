<script lang="ts">
    let { data, children } = $props();
  </script>

  <div class="mt-px space-y-4">
    Category detail card for: {data.category}

    {@render children()}
  </div>
