<script lang="ts">
    let { account = $bindable() } = $props();

</script>

<h1>{account.pk} {account.name}</h1>
<p>And here I am</p>
