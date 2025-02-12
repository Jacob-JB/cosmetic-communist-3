import { browser } from "$app/environment";
import Surreal from "surrealdb";
import { getCookieValue } from "$lib/cookie";

const browser_user_db = browser ? init_surreal_client() : null;

export async function use_db(): Promise<Surreal> {
  return await browser_user_db!;
}

export async function init_surreal_client(): Promise<Surreal> {
  const db = new Surreal();

  let discordToken = getCookieValue("discord-token");

  await db.connect("ws://127.0.0.1:8000", {
    auth: {
      namespace: "cosmetics",
      database: "cosmetics",
      access: "account",
      variables: {
        discordToken,
      },
    },
  });

  return db;
}

// https://github.com/AlbertMarashi/surrealdb-svelte-auth-template/blob/main/src/lib/database.ts
//
//
