<script lang="ts">
  import { Datepicker, type DateOrRange } from "flowbite-svelte";

  let {
    value = $bindable(),
    required,
    inline = false,
    locale = "default",
    onselect,
    onclear,
    onapply,
  }: {
    value: Date | undefined;
    required: boolean;
    inline?: boolean;
    locale?: string;
    onselect?: (x: DateOrRange) => void;
    onclear?: () => void;
    onapply?: (x: DateOrRange) => void;
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

<Datepicker {required} {inline} bind:value {dateFormat} {locale} {onselect} {onclear} {onapply}/>
