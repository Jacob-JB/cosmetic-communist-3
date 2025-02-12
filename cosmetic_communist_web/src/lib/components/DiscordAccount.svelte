<script lang="ts">
    import { onMount } from "svelte";
    import { use_db } from "$lib/database";
    import { getCookieValue } from "$lib/cookie";

    onMount(async () => {
        // await fetch("https://discord.com/api/v10/oauth2/token", {
        //     method: "POST",
        //     headers: {
        //         Authorization: `Basic ${btoa(`${clientId}:${clientSecret}`)}`,
        //         "Content-Type": "application/x-www-form-urlencoded",
        //     },
        //     body: formData.toString(),
        // });

        let userId = 389923975536312320;
        let discordToken = getCookieValue("discord-token");

        let response = await fetch(
            `https://cdn.discordapp.com/user/${userId}`,
            {
                method: "GET",
                headers: {
                    Authorization: "Bearer " + discordToken,
                    "Content-Type": "application/json",
                },
            },
        );

        console.log(response);
    });
</script>
