<script lang="ts">
    import CosmeticList from "$lib/components/CosmeticList.svelte";
    import FoundCosmetics from "$lib/components/FoundCosmetics.svelte";
    import FoundSomething from "$lib/components/FoundSomething.svelte";
</script>

<div class="flex flex-row">
    <CosmeticList />
    <FoundCosmetics />
    <FoundSomething />
</div>
