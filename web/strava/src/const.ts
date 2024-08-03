import { load } from "https://deno.land/std@0.213.0/dotenv/mod.ts";

await load({
  envPath: "../.env",
  export: true,
});

const client_id = Deno.env.get("CLIENT_ID")!;
const redirect_url = Deno.env.get("REDIRECT_URL")!;
const scope = Deno.env.get("SCOPE")!;
const client_secret = Deno.env.get("CLIENT_SECRET")!;
const yahoo_appid = Deno.env.get("YAHOO_APPID")!;

if (!client_id || !redirect_url || !scope || !client_secret || !yahoo_appid) {
  throw new Error("Missing environment variables");
}

export { client_id, client_secret, redirect_url, scope, yahoo_appid };
