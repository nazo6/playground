import * as v from "https://deno.land/x/valibot@v0.18.0/mod.ts";
import { db } from "./db.ts";

const yahooReverseGeocodingResponseSchema = v.object({
  ResultInfo: v.object({
    Count: v.number(),
    Total: v.number(),
    Start: v.number(),
    Status: v.number(),
    Latency: v.number(),
    Description: v.string(),
  }),
  Feature: v.array(
    v.object({
      Property: v.object({
        Address: v.string(),
        Building: v.optional(v.array(
          v.object({
            Name: v.string(),
          }),
        )),
      }),
    }),
  ),
});
async function yahooReverseGeocodingInner(
  { lat, lon, appid }: {
    lat: number | string;
    lon: number | string;
    appid: string;
  },
) {
  const query = new URLSearchParams({
    lat: lat.toString(),
    lon: lon.toString(),
    appid,
    output: "json",
  });
  const res = await (await fetch(
    "https://map.yahooapis.jp/geoapi/V1/reverseGeoCoder?" + query,
  )).json();

  const result = v.safeParse(yahooReverseGeocodingResponseSchema, res);

  if (!result.success) {
    throw new Error(JSON.stringify(result.issues, null, 2));
  }

  return result.output;
}

export async function yahooReverseGeocoding(
  { lat, lon, appid }: {
    lat: number | string;
    lon: number | string;
    appid: string;
  },
) {
  const cache =
    (await db.get<v.Output<typeof yahooReverseGeocodingResponseSchema>>([
      "cache",
      "yahooReverseGeocoding",
      lat,
      lon,
    ])).value;
  if (cache) {
    return cache;
  }
  const data = await yahooReverseGeocodingInner({ lat, lon, appid });
  await db.set(
    ["cache", "yahooReverseGeocoding", lat, lon],
    data,
  );
  return data;
}
