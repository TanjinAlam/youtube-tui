# Install Packages

```
create .env & update your Google Youtube API KEY
cargo run
```

# Go to the executable file directory

```
cd target/debug
```

# Run to find songs & play

```
./youtube-tui --help        // show the help
./youtube-tui -s "neshar bojha"      // show the help

It will appear something like this

"Popeye(Bangladesh) - Neshar Bojha Lyrics Video" ---- ID "O3SyEDO0K9U"
"Neshar Bojha - Popeye Bangladesh" ---- ID "MmIrIiYPTxQ"
"Popeye Bangladesh | নেশ\u{9be}র বোঝ\u{9be} | Neshar Bojha Lofi Music |" ---- ID "zJwSus_ZqW0"
"🎶 Neshar Bojha- Lyrics || Aaj ami sob harano (আজ আমি সব হ\u{9be}র\u{9be}নো) || Popeye || Lyrical Music By Farzan" ---- ID "3GkA8vR19Oo"
"Aaj ami sob harano ( আজ আমি সব হ\u{9be}র\u{9be}নো ) Neshar Bojha Lyrics (নেশ\u{9be}র বোঝ\u{9be}) Popeye | Copy Unlimited" ---- ID "myWgnBVxHFA"

# Get the ID of the song
./youtube-tui -p "O3SyEDO0K9U"      // show the help
```
