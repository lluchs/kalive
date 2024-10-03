<p align="center"><img width="128" height="128" src="presentation/icon.svg"></p>
<h1 align="center">KA.live: Karlsruhe departures</h1>

A Garmin Connect IQ widget displaying (public transport) nearby stops and departures within the KVV area in Germany.

<!-- Get it on the [Connect IQ Store](https://apps.garmin.com/en-GB/apps/TODO). -->

## Features

- View nearby stops
- Save favorite stops and view anywhere
- View departures
  - Filter by platform
  - See situations

## APIs

KA.live uses KVV's TRIAS API (see below).

KA.live is not an official KVV app. KVV is not liable for any contents.

## Develop

Obtain access to a TRIAS API: [KVV Open Data](https://www.kvv.de/fahrplan/fahrplaene/open-data.html)

Set API gateway settings in your environment:
```
LISTEN_URL=localhost:3100
TRIAS_URL=<URL>
TRIAS_REF=<REF>
API_SECRET=<SECRET>
```

Build and run the API gateway:
```
cd kalive-api
cargo build --release
target/release/kalive-api
```

Place your API keys somewhere gitignored, such as `ServiceSecrets.mc`:

```
const API_SECRECT = "<SECRET>";
```

