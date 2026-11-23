# Workshop games

Which Source games prybar knows can take a Steam Workshop upload from a
third-party tool (ISteamUGC, the API the game's own uploader uses), and
what each one accepts. `prybar --json publish --games` prints the same
table plus the games installed on this machine; the GUI publish page
lists only those.

| Game | id | App id | Supported | Content | Notes |
|---|---|---|---|---|---|
| Garry's Mod | `garrysmod` | 4000 | yes | addon folder with `addon.json` (packed to one `.gma`) or a `.gma` | files outside the GMod whitelist are refused; empty tags come from `addon.json` (`Addon` + type + tags) |
| Team Fortress 2 | `tf2` | 440 | yes, maps | folder with at least one `.bsp` (in it or in `maps/`) | cosmetics and other items go through the in-game item importer, not supported |
| Left 4 Dead 2 | `l4d2` | 550 | no | | legacy single-file Workshop; use Valve's L4D2 Workshop Publisher |
| Portal 2 | `portal2` | 620 | no | | legacy Workshop; publish from the Puzzle Maker or the Portal 2 Authoring Tools |
| Counter-Strike: Source | `css` | 240 | no | | no Steam Workshop |
| Half-Life 2 | `hl2` | 220 | no | | no Steam Workshop |
| Source SDK Base 2013 MP | `sdk2013mp` | 243750 | no | | no Steam Workshop |

Rules shared by every game: title up to 128 characters, description and
change note up to 8000 (Steam BBCode), preview png/jpg (cropped to
512x512) or a GIF under 1 MB, the Steam account must own the game.

Sources: Steamworks documentation, "Steam Workshop" and "ISteamUGC
Interface" (partner.steamgames.com/doc/features/workshop,
partner.steamgames.com/doc/api/ISteamUGC); Valve Developer Community,
"Garry's Mod Workshop" / "gmpublish" (wiki.facepunch.com/gmod/Workshop_Addon_Creation),
"Team Fortress 2 Workshop map submission"
(developer.valvesoftware.com/wiki/Steam_Workshop_(TF2)), "Left 4 Dead 2
Workshop Publisher" and "Portal 2 Puzzle Maker" pages on the same wiki.
Detection reads the games config first, then `steamapps/common/<install>`
in every Steam library.
