<script lang="ts">
  import { Avatar } from 'flowbite-svelte';

  let {
    photo_url,
    name,
    ...props
  }: {
    photo_url: string | undefined;
    name: string;
  } = $props();

  const initials = (name: string): string => {
    let words = name.split(/\s/);
    if (words.length == 1) {
      return name.substring(0, 3);
    } else {
      let acronym = words.map((word) => word.replace('(', '')).reduce((response, word) => (response += word.slice(0, 1)), '');
      return acronym.substring(0, 3);
    }
  };
</script>

{#if photo_url}
  <Avatar title={name} src={photo_url} {...props}>{name}</Avatar>
{:else}
  <Avatar title={name} {...props}>{initials(name).toUpperCase()}</Avatar>
{/if}
