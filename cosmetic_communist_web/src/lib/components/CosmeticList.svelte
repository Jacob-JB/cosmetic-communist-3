<script lang="ts">
    import { onMount } from "svelte";
    import { use_db } from "$lib/database";

    type Cosmetic = {
        id: RecordId;
        name: string
        need: boolean;
    };

    const cosmetics: Cosmetic[] = $state([]);

    function setNeeded(id: RecordId, need: boolean) {
        cosmetics.find(e => e.id.id == id.id).need = need;
    }

    onMount(async () => {
        const db = await use_db();
        const response = await db.query(
            "SELECT id, name FROM cosmetic; SELECT out FROM needs WHERE in.id = $auth; LIVE SELECT out FROM needs WHERE in = $auth;",
        );

        response[0]?.forEach(record => {
            cosmetics.push({
                id: record.id,
                name: record.name,
            });
        });

        response[1]?.forEach(record => {
            setNeeded(record.out, true);
        });

        await db.subscribeLive(response[2]?, (action, data) => {
            if (action == "CREATE") {
                setNeeded(data.out, true);
            }
            if (action == "DELETE") {
                setNeeded(data.out, false);
            }
        });
    })
</script>

<table class="m-4">
    <tbody>
        {#each cosmetics as cosmetic}
            <tr>
                <td class="p-1">
                    <p>{cosmetic.name}</p>
                </td>

                <td class="p-1">
                    <button
                        type="button"
                        class="rounded-sm {cosmetic.need
                            ? 'bg-indigo-600'
                            : 'bg-indigo-400'} px-2 py-1 text-xs font-semibold text-white shadow-xs hover:bg-indigo-500"
                        onclick={async () => {
                            const db = await use_db();
                            if (cosmetic.need) {
                                await db.query(
                                    `DELETE $auth->needs WHERE out = ${cosmetic.id}`,
                                );
                            } else {
                                await db.query(
                                    `RELATE $auth->needs->${cosmetic.id}`,
                                );
                            }
                        }}
                    >
                        {cosmetic.need ? "Needed" : "Unlocked"}
                    </button>
                </td>
            </tr>
        {/each}
    </tbody>
</table>
