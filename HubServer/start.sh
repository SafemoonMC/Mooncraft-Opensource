#!/bin/sh

identifier=$(echo $P_SERVER_UUID | cut -d'-' -f 1)

curl "https://pterodactyl.mooncraft.dev/api/client/servers/$identifier/power" \
  -H 'Accept: application/json' \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer z8egv2rfNbhfbDnZX8jMFakP0UIdYNdoeYJ07eJb3cjLtOXE' \
  -X POST \
  -d '{"signal": "restart"}'