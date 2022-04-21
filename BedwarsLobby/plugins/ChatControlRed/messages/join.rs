# ---------------------------------------------------------------------------------
# Join messages. Uses the same syntax as files in rules/ folder but operators
# are slightly different. For documentation and a quick tutorial, see:
# https://github.com/kangarko/ChatControl-Red/wiki/messages
# ---------------------------------------------------------------------------------

group administrator
require sender perm joinmessage.administrator
message:
- none

group all
require sender perm joinmessage.disabled
message:
- none

group diamondhands
require sender perm joinmessage.diamondhands
message:
- &b✦&3✦&b✦ &bDiamondhands &8| &b{player} &ehas joined the game! &b✦&3✦&b✦

group trader
require sender perm joinmessage.trader
message:
- &b✦&3✦ &aTrader &8| &a{player} &ehas joined the game! &3✦&b✦

group investor
require sender perm joinmessage.investor
message:
- &b✦ &6Investor &8| &6{player} &ehas joined the game! &b✦

group holder
require sender perm joinmessage.holder
message:
- &2Holder &8| &2{player} &ehas joined the game!

group paperhands
require sender perm joinmessage.paperhands
message:
- &fPaperhands &8| &f{player} &ehas joined the game!
