<script lang="ts">
    import { onMount } from "svelte";
    import { use_db } from "$lib/database.ts";

    type Cosmetic = {
        id: string;
        need: boolean;
    };

    const cosmetics: Cosmetic[] = $state([]);

    function setNeeded(id: string, need: boolean) {
        cosmetics.find((e) => e.id == id).need = need;
    }

    onMount(async () => {
        const db = await use_db();
        const response = await db.query(
            "SELECT * FROM cosmetic; SELECT out FROM needs WHERE in.id = $auth; LIVE SELECT out FROM needs WHERE in = $auth;",
        );

        response[0]?.forEach(record => {
            cosmetics.push({
                id: record.id.id,
            });
        });

        response[1]?.forEach(record => {
            setNeeded(record.out.id, true);
        });

        await db.subscribeLive(response[2]?, (action, data) => {
            if (action == "CREATE") {
                setNeeded(data.out.id, true);
            }
            if (action == "DELETE") {
                setNeeded(data.out.id, false);
            }
        });
    })
</script>

<table class="m-4">
    <tbody>
        {#each cosmetics as cosmetic}
            <tr>
                <td class="p-1">
                    <p>{cosmetic.id}</p>
                </td>

                <td class="p-1">
                    <button
                        type="button"
                        class="rounded-sm {cosmetic.need
                            ? 'bg-indigo-600'
                            : 'bg-indigo-400'} px-2 py-1 text-xs font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                        onclick={async () => {
                            const db = await use_db();
                            if (cosmetic.need) {
                                await db.query(
                                    `DELETE $auth->needs WHERE out = cosmetic:\`${cosmetic.id}\``,
                                );
                            } else {
                                await db.query(
                                    `RELATE $auth->needs->cosmetic:\`${cosmetic.id}\``,
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
