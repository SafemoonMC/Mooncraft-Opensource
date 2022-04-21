# -----------------------------------------------------------------------------------------------
# This file applies rules to chat messages and includes rules from global.rs.
#
# For help, see https://github.com/kangarko/ChatControl-Red/wiki/Rules
# -----------------------------------------------------------------------------------------------

@import global

# -----------------------------------------------------------------------------------------------
# Anti spam.
# -----------------------------------------------------------------------------------------------

# Limiting repeating characters to four repeats maximum.
#match (.)(?=([a-z])\1{3})
#then replace

# Prevent special, unicode characters. Those are misused to bypass filters.
# Please keep in mind that non-English languages use them, so only uncomment this on English servers.
match [^\u0000-\u007F]+
then warn &cUnicode characters are prohibited.
then deny

# Prevent hacked clients spamming messages with UUIDs in them: https://i.imgur.com/zuaROdw.png
match (\[|)\b[0-9a-f]{8}(\b-|)[0-9a-f]{4}(-|)[0-9a-f]{4}(-|)[0-9a-f]{4}(-\b|)[0-9a-f]{12}(\]|)
then warn &cPlease do not type UUIDs into chat.
then deny

# -----------------------------------------------------------------------------------------------
# Forwarding messages to other channels.
# -----------------------------------------------------------------------------------------------

# Send messages starting with "!" to staff channel
match ^(\!)(.*)
then command channel send staff $2
then deny
dont verbose

# Send messages starting with "~" to admin channel
match ^(\~)(.*)
then command channel send admin $2
then deny
dont verbose



#match \bdis\b
#then replace this

#match \bwanna\b
#then replace want to

#match \bgonna\b
#then replace going to

#match \bu\b
#then replace you

#match \bdia\b
#then replace diamond

#match \bgg\b
#then replace Good Game!

#match \bnp\!\b
#then replace No Problem!

#match \bomg\b
#then replace My golly, this is qwite the shock OwO

#match \bwtf\b
#then replace This princessy-wessy is qwite confuzzled

#match ^(hello|hi|sup)\b
#then replace Greetings from afar fellow pwincesses, this pwincess is looking for some love and compassion

#match ^no
#then replace noh :3

#match ^yes$
#then replace ow yeah :3

#match \bcya\b
#then replace bon voyage, me princess

#match \boml\b
#then replace >.< i iz confuzzled i need some hewp appwreciating dis fwilter

#match \bcannon\b
#then replace shooty-wooty

# -----------------------------------------------------------------------------------------------
# Educate players.
# -----------------------------------------------------------------------------------------------

match ^how (do|can) (I|you) (build|claim)(| a) (land|residence)
then warn {prefix_info}Use &9/help land &7to learn about claiming land!
then deny

match \bl+\s*a+\s*g+
then warn &cIf you believe you have lag, check your internet connection. Your current ping within the server is &e{player_ping}ms
then deny

# -----------------------------------------------------------------------------------------------
# Prevent begging for ranks.
# -----------------------------------------------------------------------------------------------

match (can|may|would you like if) i (have|be|become|get|has) (op|admin|mod|builder)
then warn &cIf you would wish to apply use the following form: &b&nhttps://mooncraft.gg/forms.html
then deny

match [cC]an I [aA]pply
then warn &cIf you would wish to apply use the following form: &b&nhttps://mooncraft.gg/forms.html
then deny

match ([hH]ow|[wW]here) [cC]an [iI] [aA]pply
then warn &cIf you would wish to apply use the following form: &b&nhttps://mooncraft.gg/forms.html
then deny

match ([hH]ow) [cC]an I ([aA]pply) (to be|for) ([sS]taff|[oO]p|[oO]ps|[oO]perators|[aA]dmins|[mM]ods|[mM]od|[bB]uilders|[aA]dmin|[nN]ew people|[tT]eam)
then warn &cIf you would wish to apply use the following form: &b&nhttps://mooncraft.gg/forms.html
then deny

match [aA]re [aA]pplications|[aA]pps|[sS]taff [aA]pplications|[sS]taff apps [oO]pen
then warn &cIf you would wish to apply use the following form: &b&nhttps://mooncraft.gg/forms.html
then deny

# -----------------------------------------------------------------------------------------------
# Prevent people saying bad things about your server.
# -----------------------------------------------------------------------------------------------

match [tT]his [Ss]erver (is (bad|crappy|shitty)|suck|sucks|ass)
name server hate
then notify chatcontrol.notify.rulesalert &8[&7ID {rule_name}&8] &7{player}: &f{original_message}
then rewrite I love this server!|Who wants to play with me?|Thanks for being so nice guys!|I love my cats!|I love my dogs!
then console warn {player} Inappropriate Chat Behavior

match [Ss]erver (is (bad|crappy|shitty)|suck|sucks|ass)
name server hate
then notify chatcontrol.notify.rulesalert &8[&7ID {rule_name}&8] &7{player}: &f{original_message}
then rewrite I love this server!|Who wants to play with me?|Thanks for being so nice guys!|I love my cats!|I love my dogs!
then console warn {player} Inappropriate Chat Behavior

# -----------------------------------------------------------------------------------------------
# Simple chat bots.
# You can then use the {data_name} variable anywhere else, also in PlaceholderAPI!
# -----------------------------------------------------------------------------------------------

# You can create simple helping bots to answer frequently asked questions.
# This will simply listen to the question below and then send player formats/sethome.yml message
#match ^how (do|can|to)(| I| you) (set|create|place)(| a) home
#then warn sethome
#then deny

# Or you can create advanced bots saving and showing data (data is saved permanently)
# See https://github.com/kangarko/ChatControl-Red/wiki/Rules for a tutorial
#match ^(\@bot name)$
#ignore key name
#then warn &8[&dBot&8] &7Please enter your name.
#then deny

#match ^(\@bot name)$
#require key name
#then warn &8[&dBot&8] &7Your name is: {data_name}
#then deny

#match ^(\@bot name) null$
#save key name
#then warn &8[&dBot&8] &7Removed your name.
#then deny

#match ^(\@bot name)(.*)
#save key name "$2".trim()
#then warn &8[&dBot&8] &7Saved your name as: {data_name}.
#then deny

# -----------------------------------------------------------------------------------------------
# Special filter to colorize any commands appearing in the chat. This will turn "Type /help"
# into "Type &c/help" and then intelligently revert back to whatever color the message had
# previously.
#
# PLEASE NOTE that the [#flpc-X] variables are the only two special ones you can use without
# any modifications. If you have Strip_Colors on true in Rules in settings.yml, using any other
# color in the message will get it removed because the match removes colors as per this setting.
# -----------------------------------------------------------------------------------------------

#match (,|\s)\/(\w+)
#then replace [#flpc-i]&c$1/$2[#flpc-1]

# -----------------------------------------------------------------------------------------------
# Smileys (some turds call them emojis)
#
# Notice this may or may not work on your system. Ensure you save the file in UTF-8 encoding,
# and if it still does not work, do not report this, ask your server administrator/hosting.
# -----------------------------------------------------------------------------------------------

match :-\(
then replace ☹

match (:\))|(;\))
then replace ㋡

match \:star\:
then replace ★

#match \:shrug\:
#then replace ¯\\_(ツ)_/¯

#match \:flip\:
#then replace (╯°□°）╯︵ ┻━┻

#match \:wtf\:
#then replace ⚆_⚆

#match \:derp\:
#then replace (◑‿◐)

#match \:love\:
#then replace (✿ ♥‿♥)

#match \:sad\:
#then replace (ಥ﹏ಥ)

#match \:finger\:
#then replace ╭∩╮ ( •_• ) ╭∩╮

#match \:peace\:
#then replace ✌(-‿-)✌

#match \:face\:
#then replace (ᵔᴥᵔ)

#match \:dog\:
#then replace (◕ᴥ◕ʋ)

#match \:bat\:
#then replace /\|\ ^._.^ /\|\

#match \:gun\:
#then replace ︻╦╤─

#match \:butterfly\:
#then replace ƸӜƷ

#match \:tick\:
#then replace &a✓&r

#match \:creep\:
#then replace ԅ(≖‿≖ԅ)

#match \:dead\:
#then replace x⸑x

#match \:fu\:
#then replace ┌П┐(ಠ_ಠ)

#match \:haha\:
#then replace ٩(^‿^)۶

#match \:magicflip\:
#then replace (/¯◡ ‿ ◡)/¯ ~ ┻━┻

#match \:meep\:
#then replace \(°^°)/

#match \:meh\:
#then replace ಠ_ಠ

#match \:no\:
#then replace →_←

#match \:nyan\:
#then replace ~=[,,_,,]:3

#match \:omg\:
#then replace ◕_◕

#match \:rainbowcat\:
#then replace (=^･ｪ･^=))ﾉ彡☆

#match \:cat\:
#then replace ฅ^•ﻌ•^ฅ

#match \:shy\:
#then replace =^_^=

#match \:smirk\:
#then replace ¬‿¬

#match \:unflip\:
#then replace ┬──┬ ノ(ò_óノ)

match \:up\:
then replace ↑

#match \:whistle\:
#then replace (っ^з^)♪♬

#match \:wut\:
#then replace ⊙ω⊙

#match \:yay\:
#then replace \( ﾟヮﾟ)/

#match \:rip\:
#then replace rest in spaghetti never forgetti
