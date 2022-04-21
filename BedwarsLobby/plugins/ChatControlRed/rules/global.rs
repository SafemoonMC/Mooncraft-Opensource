match [0-9]{1,3}(\.|dot|\(dot\)|-|;|:|,|(\W|\d|_)*\s)+[0-9]{1,3}(\.|dot|\(dot\)|-|;|:|,|(\W|\d|_)*\s)+[0-9]{1,3}(\.|dot|\(dot\)|-|;|:|,|(\W|\d|_)*\s)+[0-9]{1,3}
name ip
before replace dot|\[|\]|\{|\}|\(|\) with .
before replace [\(\[\]\)]|\s*
then warn &cDo not advertise other servers, websites or resources in chat!
then points advert 2
then deny

match [a-zA-Z0-9\-\.]+\s?(\.|dot|\(dot\)|-|;|:|,)\s?(c(| +)o(| +)m|o(| +)r(| +)g|n(| +)e(| +)t|c(| +)z|c(| +)o|u(| +)k|s(| +)k|b(| +)i(| +)z|m(| +)o(| +)b(| +)i|x(| +)x(| +)x|e(| +)u|m(| +)e|i(| +)o)\b
#ignore string mooncraft.gg|play.mooncraft.gg|safemoon.net
name url
before replace dot|\[|\]|\{|\}|\(|\) with .
before replace [\(\[\]\)]
then warn &cDo not advertise other servers, websites or resources in chat!
then points advert 2
then deny

match \u534D|\u5350
then kick &cDisallowed characters in the chat
then notify chatcontrol.notify.rulesalert &8[&7Swastika&8] &7{player}: &f{original_message}
then deny
then points swear 7

# Bitch filter - commented out just to show you example of usage. It is filtered in more advanced way below.
#match \bb(i|!)tch
#name swear
#then warn &cWatch your language please.
#then points swear 1

# Blocks 'fuck'
match \b(f+(\W|\d|_)*u+(\W|\d|_)*c+(\W|\d|_)*k+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'nigg'
match \b(n+(\W|\d|_)*i+(\W|\d|_)*g+(\W|\d|_)*g+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'niga'
match \b(n+(\W|\d|_)*i+(\W|\d|_)*g+(\W|\d|_)*a+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'horny'
match \b(h+(\W|\d|_)*o+(\W|\d|_)*r+(\W|\d|_)*n+(\W|\d|_)*y+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'hentai'
match \b(h+(\W|\d|_)*e+(\W|\d|_)*n+(\W|\d|_)*t+(\W|\d|_)*a+(\W|\d|_)*i+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'hitler'
match \b(h+(\W|\d|_)*i+(\W|\d|_)*t+(\W|\d|_)*l+(\W|\d|_)*e+(\W|\d|_)*r+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'slut'
match \b(s+(\W|\d|_)*l+(\W|\d|_)*u+(\W|\d|_)*t+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'nazi'
match \b(n+(\W|\d|_)*a+(\W|\d|_)*z+(\W|\d|_)*i+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'slag'
match \b(s+(\W|\d|_)*l+(\W|\d|_)*a+(\W|\d|_)*g+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'boob'
match \b(b+(\W|\d|_)*o+(\W|\d|_)*o+(\W|\d|_)*b+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'pussy'
match \b(p+(\W|\d|_)*u+(\W|\d|_)*s+(\W|\d|_)*s+(\W|\d|_)*y+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'vagina'
match \b(v+(\W|\d|_)*a+(\W|\d|_)*g+(\W|\d|_)*i+(\W|\d|_)*n+(\W|\d|_)*a+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'penis'
match \b(p+(\W|\d|_)*e+(\W|\d|_)*n+(\W|\d|_)*i+(\W|\d|_)*s+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'bugger'
match \b(b+(\W|\d|_)*u+(\W|\d|_)*g+(\W|\d|_)*g+(\W|\d|_)*e+(\W|\d|_)*r+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'bastard'
match \b(b+(\W|\d|_)*a+(\W|\d|_)*s+(\W|\d|_)*t+(\W|\d|_)*a+(\W|\d|_)*r+(\W|\d|_)*d+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'cunt'
match \b(c+(\W|\d|_)*u+(\W|\d|_)*n+(\W|\d|_)*t+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'jerk'
match \b(j+(\W|\d|_)*e+(\W|\d|_)*r+(\W|\d|_)*k+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'wanker'
match \b(w+(\W|\d|_)*a+(\W|\d|_)*n+(\W|\d|_)*k+(\W|\d|_)*e+(\W|\d|_)*r+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'tosser'
match \b(t+(\W|\d|_)*o+(\W|\d|_)*s+(\W|\d|_)*s+(\W|\d|_)*e+(\W|\d|_)*r+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'shit'
match \b(s+(\W|\d|_)*h+(\W|\d|_)*i+(\W|\d|_)*t+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'rape'
match \b(r+(\W|\d|_)*a+(\W|\d|_)*p+(\W|\d|_)*e+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'rapist'
match \b(r+(\W|\d|_)*a+(\W|\d|_)*p+(\W|\d|_)*i+(\W|\d|_)*s+(\W|\d|_)*t+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'dick'
match \b(d+(\W|\d|_)*i+(\W|\d|_)*c+(\W|\d|_)*k+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'cock'
match \b(c+(\W|\d|_)*o+(\W|\d|_)*c+(\W|\d|_)*k+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'whore'
match \b(w+(\W|\d|_)*h+(\W|\d|_)*o+(\W|\d|_)*r+(\W|\d|_)*e+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'bitch'
match \b(b+(\W|\d|_)*i+(\W|\d|_)*t+(\W|\d|_)*c+(\W|\d|_)*h+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'asshole'
match \b(a+(\W|\d|_)*s+(\W|\d|_)*s+(\W|\d|_)*h+(\W|\d|_)*o+(\W|\d|_)*l+(\W|\d|_)*e+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'twat'
match \b(t+(\W|\d|_)*w+(\W|\d|_)*a+(\W|\d|_)*t+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'titt'
match \b(t+(\W|\d|_)*i+(\W|\d|_)*t+(\W|\d|_)*(t|s)+(\W|\d|_)*)
ignore string title
then deny
then points swear 1

# Blocks 'piss'
match \b(p+(\W|\d|_)*i+(\W|\d|_)*s+(\W|\d|_)*s+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'gay'
match \b(g+(\W|\d|_)*a+(\W|\d|_)*y+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'homo'
match \b(h+(\W|\d|_)*o+(\W|\d|_)*m+(\W|\d|_)*o+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'homosexual'
match \b(h+(\W|\d|_)*o+(\W|\d|_)*m+(\W|\d|_)*o+(\W|\d|_)*s+(\W|\d|_)*e+(\W|\d|_)*x+(\W|\d|_)*u+(\W|\d|_)*a+(\W|\d|_)*l+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'intercourse'
match \b(i+(\W|\d|_)*n+(\W|\d|_)*t+(\W|\d|_)*e+(\W|\d|_)*r+(\W|\d|_)*c+(\W|\d|_)*o+(\W|\d|_)*u+(\W|\d|_)*r+(\W|\d|_)*s+(\W|\d|_)*e+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'cum'
match \b(c+(\W|\d|_)*u+(\W|\d|_)*m+(\W|\d|_)*) 
then deny
then points swear 1

# Blocks 'sperm'
match \b(s+(\W|\d|_)*p+(\W|\d|_)*e+(\W|\d|_)*r+(\W|\d|_)*m+(\W|\d|_)*) 
then deny
then points swear 1

# Blocks 'spunk'
match \b(s+(\W|\d|_)*p+(\W|\d|_)*u+(\W|\d|_)*n+(\W|\d|_)*k+(\W|\d|_)*) 
then deny
then points swear 1

# Blocks 'testicle'
match \b(t+(\W|\d|_)*e+(\W|\d|_)*s+(\W|\d|_)*t+(\W|\d|_)*i+(\W|\d|_)*c+(\W|\d|_)*l+(\W|\d|_)*e+(\W|\d|_)*) 
then deny
then points swear 1

# Blocks 'milf'
match \b(m+(\W|\d|_)*i+(\W|\d|_)*l+(\W|\d|_)*f+(\W|\d|_)*) 
then deny
then points swear 1

# Blocks 'retard'
match \b(r+(\W|\d|_)*e+(\W|\d|_)*t+(\W|\d|_)*a+(\W|\d|_)*r+(\W|\d|_)*d+(\W|\d|_)*) 
then deny
then points swear 1

# Blocks 'anus'
match \b(a+(\W|\d|_)*n+(\W|\d|_)*u+(\W|\d|_)*s+(\W|\d|_)*) 
then deny
then points swear 1

# Blocks 'dafuq'
match \b(d+(\W|\d|_)*a+(\W|\d|_)*f+(\W|\d|_)*u+(\W|\d|_)*q+(\W|\d|_)*) 
then deny
then points swear 1

#Blocks 'prick'
match \b(p+(\W|\d|_)*r+(\W|\d|_)*i+(\W|\d|_)*c+(\W|\d|_)*k+(\W|\d|_)*)
then deny
then points swear 1

#Blocks 'douche'
match \b(d+(\W|\d|_)*o+(\W|\d|_)*u+(\W|\d|_)*c+(\W|\d|_)*h+(\W|\d|_)*e+(\W|\d|_)*)
then deny
then points swear 1

# Blocks 'prostitute'
match \b(p+(\W|\d|_)*r+(\W|\d|_)*o+(\W|\d|_)*s+(\W|\d|_)*t+(\W|\d|_)*i+(\W|\d|_)*t+(\W|\d|_)*u+(\W|\d|_)*t+(\W|\d|_)*e+(\W|\d|_)*) 
then deny
then points swear 1

# Blocks 'ffs'
match \b(f+(\W|\d|_)*f+(\W|\d|_)*s+(\W|\d|_)*) 
then deny
then points swear 1

# Blocks 'pedo'
match \b(p+(\W|\d|_)*e+(\W|\d|_)*d+(\W|\d|_)*o+(\W|\d|_)*) 
then deny
then points swear 1
