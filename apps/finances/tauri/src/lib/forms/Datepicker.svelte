<script lang="ts">
  import { Datepicker } from "flowbite-svelte";

  let {
    value = $bindable(),
    inline,
    required,
  }: {
    value: Date | undefined;
    required: boolean;
    inline: boolean;
  } = $props();

  let dateFormat: Intl.DateTimeFormatOptions = {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  };
</script>

<!--
@component
Just a regular DatePicker with opinionated DateTime formating
-->

<Datepicker {required} {inline} bind:value {dateFormat} />
