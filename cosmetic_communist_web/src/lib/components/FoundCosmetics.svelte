<script lang="ts">
    import { onMount } from "svelte";
    import { use_db } from "$lib/database.ts";
    import { type RecordId } from "surrealdb";

    type CosmeticFound = {
        id: RecordId<"cosmeticFound">;
        cosmetic: string;
    };

    const foundCosmetics: CosmeticFound[] = $state([]);

    function addFoundCosmetic(record) {
        foundCosmetics.push({
            id: record.id,
            cosmetic: record.foundCosmetic.id,
        });
    }

    function removeFoundCosmetic(id) {
        const index = foundCosmetics.findIndex(e => e.id.id == id.id);
        foundCosmetics.splice(index, 1);
    }

    onMount(async () => {
        const db = await use_db();
        const response = await db.query(
            "SELECT * FROM cosmeticFound WHERE foundUser = $auth; LIVE SELECT * FROM cosmeticFound WHERE foundUser = $auth;",
        );

        response[0]?.forEach(addFoundCosmetic);

        await db.subscribeLive(response[1]?, (action, record) => {
            if (action == "CREATE") {
                addFoundCosmetic(record);
            }
            if (action == "DELETE") {
                removeFoundCosmetic(record.id);
            }
        });
    });
</script>

<ul class="m-4">
    {#each foundCosmetics as foundCosmetic}
        <li>
            <div class="m-2 flex flex-col">
                <p>{foundCosmetic.cosmetic}</p>
                <button
                    type="button"
                    class="rounded-sm bg-red-600 px-2 py-1 text-xs font-semibold text-white shadow-xs hover:bg-red-500"
                    onclick={async () => {
                        const db = await use_db();
                        await db.query(`DELETE ${foundCosmetic.id}`);
                    }}>Cancel</button
                >
            </div>
        </li>
    {/each}
</ul>
