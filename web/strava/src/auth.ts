export type Token = {
  // token_type: "Bearer",
  /// ex. 1706724967
  expires_at: number;
  /// ex. 21600
  expires_in: number;
  refresh_token: string;
  access_token: string;
  athlete: {
    id: number;
    username: string;
    resource_state: 2;
    firstname: string;
    lastname: string;
  };
};

export async function authFlow({
  client_id,
  redirect_url,
  scope,
  client_secret,
}: {
  client_id: string;
  redirect_url: string;
  scope: string;
  client_secret: string;
}): Promise<Token> {
  const auth_url =
    `https://www.strava.com/oauth/authorize?client_id=${client_id}&response_type=code&redirect_uri=${redirect_url}&scope=${scope}&approval_prompt=auto`;
  console.log("Go to " + auth_url);
  const token = await new Promise((resolve) => {
    Deno.serve({ port: 3080 }, async (req) => {
      const code = new URL(req.url, "http://localhost:3080").searchParams.get(
        "code",
      );
      if (code) {
        const token = await (await fetch("https://www.strava.com/oauth/token", {
          method: "POST",
          headers: {
            "Content-Type": "application/x-www-form-urlencoded",
          },
          body: new URLSearchParams({
            client_id,
            client_secret,
            code,
            grant_type: "authorization_code",
          }),
        })).json();
        resolve(token);
        return new Response(
          "You can close this window now. Code: " + code,
          { status: 200 },
        );
      }

      return new Response(
        "error",
        { status: 500 },
      );
    });
  });
  return token as Token;
}
