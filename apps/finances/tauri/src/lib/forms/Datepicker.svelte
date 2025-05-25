<script lang="ts">
  import { Datepicker } from "flowbite-svelte";

  let {
    value = $bindable(),
    required,
    inline = false,
    locale = "default",
  }: {
    value: Date | null;
    required: boolean;
    inline?: boolean;
    locale?: string;
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

<Datepicker {required} {inline} autohide={false} bind:value {dateFormat} {locale}/>
