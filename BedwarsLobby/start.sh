#!/bin/sh

identifier=$(echo $P_SERVER_UUID | cut -d'-' -f 1)

curl "https://pterodactyl.mooncraft.dev/api/client/servers/$identifier/power" \
  -H 'Accept: application/json' \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer z8egv2rxfNbhfbDnZX8jMFaykP0UIdYNdoxeYJ07eJb3cjLtOxXE' \
  -X POST \
  -d '{"signal": "restart"}'