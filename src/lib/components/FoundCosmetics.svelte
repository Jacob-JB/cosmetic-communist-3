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

<ul>
    {#each foundCosmetics as foundCosmetic}
        <li>
            <div class="flex flex-col">
                <p>{foundCosmetic.cosmetic}</p>
                <button
                    type="button"
                    onclick={async () => {
                        const db = await use_db();
                        await db.query(`DELETE ${foundCosmetic.id}`);
                    }}>Cancel</button
                >
            </div>
        </li>
    {/each}
</ul>
