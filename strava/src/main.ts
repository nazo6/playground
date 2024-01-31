import "https://deno.land/std@0.213.0/dotenv/load.ts";

import { Strava, SummaryActivity } from "npm:strava";
// @deno-types="npm:@types/bluebird"
import Promise from "npm:bluebird";
import { authFlow } from "./auth.ts";
import { yahooReverseGeocoding } from "./reverse-geocoding.ts";
import { db } from "./db.ts";
import {
  client_id,
  client_secret,
  redirect_url,
  scope,
  yahoo_appid,
} from "./const.ts";

await Deno.mkdir("../db", { recursive: true });
await Deno.mkdir("../data", { recursive: true });

let refresh_token = (await db.get<string>(["auth", "refresh_token"])).value;
if (!refresh_token) {
  const token = await authFlow({
    client_id,
    redirect_url,
    scope,
    client_secret,
  });
  await db.set(["auth", "refresh_token"], token.refresh_token);
  refresh_token = token.refresh_token;
}

const strava = new Strava({
  client_id,
  client_secret,
  refresh_token,
  on_token_refresh: (response) => {
    console.log("Access token: ", response.access_token);
    db.set(["auth", "refresh_token"], response.refresh_token);
  },
});

const activities: SummaryActivity[] = [];
let page = 1;
while (true) {
  const a = await strava.activities.getLoggedInAthleteActivities({ page });
  if (a.length == 0) {
    break;
  }
  console.log(`Page ${page}: ${a.length} activities`);
  activities.push(...a);
  page++;
}

const rideData: {
  startAddress: string;
  endAddress: string;
  startLat: [number, number];
  endLat: [number, number];
  distance: number;
  moving_time: number;
  elapsed_time: number;
}[] = [];

await Promise.map(
  activities,
  async (activity) => {
    if (activity.type == "Ride") {
      if (
        // @ts-ignore this is correct
        activity.start_latlng.length == 0 || activity.end_latlng.length == 0
      ) {
        console.log(
          "No latlng: https://www.strava.com/activities/" + activity.id +
            "/edit_map_visibility",
        );
        return;
      }
      const startAddress = await yahooReverseGeocoding({
        appid: yahoo_appid,
        lat: activity.start_latlng[0],
        lon: activity.start_latlng[1],
      });
      const endAddress = await yahooReverseGeocoding({
        appid: yahoo_appid,
        lat: activity.end_latlng[0],
        lon: activity.end_latlng[1],
      });
      rideData.push({
        startAddress: startAddress.Feature[0].Property.Address,
        endAddress: endAddress.Feature[0].Property.Address,
        startLat: activity.start_latlng,
        endLat: activity.end_latlng,
        distance: activity.distance,
        moving_time: activity.moving_time,
        elapsed_time: activity.elapsed_time,
      });
    }
  },
  { concurrency: 5 },
);

await Deno.writeTextFile(
  "../data/data.json",
  JSON.stringify(rideData, null, 2),
);
