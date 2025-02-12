<script lang="ts">
    import CosmeticList from "$lib/components/CosmeticList.svelte";
    import FoundCosmetics from "$lib/components/FoundCosmetics.svelte";
    import FoundSomething from "$lib/components/FoundSomething.svelte";
    import DiscordAccount from "$lib/components/DiscordAccount.svelte";
</script>

<div class="flex flex-col">
    <DiscordAccount />
    <div class="flex flex-row">
        <CosmeticList />
        <FoundCosmetics />
        <FoundSomething />
    </div>
</div>
