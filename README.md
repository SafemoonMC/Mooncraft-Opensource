# Mooncraft Game Servers
This repository contains all the configured servers that were running on the Mooncraft network.

Any plugin jars that were not made exclusively by us, we cannot directly distribute. However, we have left ALL our configuration files present so our settings & alterations can be used. These plugins can be found on SpigotMC or MCMarket marketplaces. All of the code for our custom plugins are available within these repositories. However, for our internally coded solutions, you will need to edit some code to fit your needs, and then compile the jars yourself. Since build scripts were using internal and private infrastructure, things may not compile as expected out of the box. Also, attention must be oriented **only** towards the pinned repositories, any other repository may be incomplete or obsolete: in other words, were unused.

## Requirements
Servers were running with Java 17, Airplane, and Waterfall for Minecraft 1.17.

Other technologies used:
- MariaDB;
- KeyDB.

## Extra Information
While the hub, creative, and prison game modes don't need specific explanation, BedWars may require some extra words.

### BedWars is a multi-server game mode
BedWars uses KeyDB and BungeeCord's Messaging Channel to handle matches and transfer data across the servers:
- BedwarsLobby, is the server where matchmaking functions happen and players gather;
- BedwarsGame, is the server where live matches happen and players actually play.