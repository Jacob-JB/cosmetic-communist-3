<script lang="ts">
    import { onMount } from "svelte";
    import { use_db } from "$lib/database";

    type Cosmetic = {
        id: RecordId;
        name: string;
    };

    let cosmetics: Cosmetic[] = $state([]);
    let filter = $state("");
    let selected: int = $state(null);

    onMount(async () => {
        const db = await use_db();
        const response = await db.query("SELECT id, name FROM cosmetic;");

        response[0]?.forEach((record) => {
            cosmetics.push({
                id: record.id,
                name: record.name,
            });
        });
    });
</script>

<div class="m-4 flex flex-col">
    <div class="m-2 flex flex-col">
        {#if selected != null}
            <p>Found: {cosmetics[selected].name}</p>
            <button
                type="button"
                class="rounded-sm bg-green-600 px-2 py-1 text-xs font-semibold text-white shadow-xs hover:bg-green-500"
                onclick={async () => {
                    const db = await use_db();
                    await db.query(
                        `INSERT INTO cosmeticFound { foundUser: $auth, foundCosmetic: ${cosmetics[selected].id} };`,
                    );
                    selected = null;
                    filter = "";
                }}>Create Event</button
            >
        {:else}
            <p>Select a cosmetic to ping people who need it</p>
        {/if}
    </div>

    <div class="m-2 flex flex-col">
        <input type="text" placeholder="Filter" bind:value={filter} />
        {#each cosmetics as cosmetic, i}
            {#if cosmetic.name.toLowerCase().includes(filter.toLowerCase())}
                <button
                    type="button"
                    class={selected == i
                        ? "bg-gray-400"
                        : "bg-gray-200 hover:bg-gray-300"}
                    onclick={() => {
                        selected = i;
                    }}
                >
                    {cosmetic.name}
                </button>
            {/if}
        {/each}
    </div>
</div>
