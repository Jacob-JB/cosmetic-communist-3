import { browser } from "$app/environment";
import Surreal from "surrealdb";

const browser_user_db = browser ? init_surreal_client() : null;

export async function use_db(): Promise<Surreal> {
  return await browser_user_db!;
}

export async function init_surreal_client(): Promise<Surreal> {
  const db = new Surreal();

  // https://stackoverflow.com/questions/5639346/what-is-the-shortest-function-for-reading-a-cookie-by-name-in-javascript
  const getCookieValue = (name: String) =>
    document.cookie.match("(^|;)\\s*" + name + "\\s*=\\s*([^;]+)")?.pop() || "";

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
