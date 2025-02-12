import { redirect } from "@sveltejs/kit";
import { CLIENT_ID, CLIENT_SECRET } from "$env/static/private";

export async function GET({ url, cookies }) {
  try {
    const code = url.searchParams.get("code");

    if (!code) {
      return new Response(
        JSON.stringify({ error: "Missing 'code' parameter" }),
        {
          status: 400,
          headers: { "Content-Type": "application/json" },
        },
      );
    }

    const clientId = CLIENT_ID; // Replace with env variable
    const clientSecret = CLIENT_SECRET; // Replace with env variable
    const redirectUri = "http://localhost:5173/discord"; // Ensure it's configured securely

    // Use URLSearchParams for encoding
    const formData = new URLSearchParams();
    formData.append("grant_type", "authorization_code");
    formData.append("code", code);
    formData.append("redirect_uri", redirectUri);

    // Make request to Discord OAuth API
    const response = await fetch("https://discord.com/api/v10/oauth2/token", {
      method: "POST",
      headers: {
        Authorization: `Basic ${btoa(`${clientId}:${clientSecret}`)}`,
        "Content-Type": "application/x-www-form-urlencoded",
      },
      body: formData.toString(),
    });

    if (!response.ok) {
      console.error("Discord API error:", await response.text());
      return new Response(
        JSON.stringify({ error: "Failed to exchange authorization code" }),
        {
          status: response.status,
          headers: { "Content-Type": "application/json" },
        },
      );
    }

    // Parse JSON response safely
    const tokenData = await response.json();

    if (!tokenData.access_token || tokenData.scope !== "identify") {
      console.error("Invalid token data:", tokenData);
      return new Response(JSON.stringify({ error: "Invalid OAuth scope" }), {
        status: 403,
        headers: { "Content-Type": "application/json" },
      });
    }

    // Securely set the authentication cookie
    cookies.set("discord-token", tokenData.access_token, {
      path: "/",
      httpOnly: false,
      secure: true,
      sameSite: "strict",
      maxAge: tokenData.expires_in,
    });

    return new Response(null, {
      status: 302,
      headers: {
        Location: "/", // Redirects to the main dashboard
      },
    });
  } catch (error) {
    console.error("OAuth handler error:", error);
    return new Response(JSON.stringify({ error: "Internal Server Error" }), {
      status: 500,
      headers: { "Content-Type": "application/json" },
    });
  }

  // Redirect to the dashboard after authentication
  throw redirect(302, "/");
}
