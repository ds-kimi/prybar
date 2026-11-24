# GUI, CLI and MCP parity

Everything the desktop app does can be done from the CLI (`prybar --json ...`) and the MCP server (`prybar-mcp`). All three call `prybar-service`, so the same input gives the same result. Writes that others see or that change the user's settings need `confirm: true` in MCP.

| Feature | GUI | CLI | MCP |
|---|---|---|---|
| Model summary (version, bones, meshes, sequences, materials, companions) | Model screen | `inspect <model>` | `inspect_model` |
| Decompile to QC/SMD/VTA/VRD (tree, includes) | Decompile screen (file, package, or a picked/dropped folder; switches for subfolders, tree, includes, split QC; item list, cancel) | `decompile <inputs> -o <dir> [--flat] [--no-includes]` | `decompile_model` (`flat`, `with_includes`) |
| Browse a package (search) | Package screen | `list <pkg> --filter <wildcard>` | `list_package` |
| Extract a package | Package screen › Extract | `unpack <pkg> -o <dir>` | `unpack_package` |
| Build a VPK or GMA | Package screen | `pack <dir> -o <out>` | `pack_package` |
| Compile with the log | Compile screen (streamed) | `compile <qc> -g <game>` (log in the report) | `compile_model` `qc` (log in the result) |
| Batch compile a folder | Compile screen › Folder… or drop a folder (item list, cancel, Open output) | `compile <dir\|*.qc>... -g <game> [-r] [-j N]` | `compile_model` `inputs`, `recursive`, `jobs` |
| Open in hlmv | Model screen | `view <mdl> -g <game>` | `view_model` |
| Texture preview (VTF decode, mip level) | Texture screen | `texture <vtf> -o <png> [--level]`, `texture <model> --material <m> -o <png>` | `decode_texture` |
| 3D model preview | Model screen viewport | none: GPU-only view; `inspect` gives the data it draws | none, same reason |
| Games: list, detect, save | Settings › Games | `games [--detect] [--save]` | `games_list`, `config_get` |
| Games: add, edit, remove one | Settings › Games | edit the config file (`PRYBAR_CONFIG`) | `config_set` (no tool paths, on purpose) |
| Steam status and signed-in account | Publish avatar badge | `publish --status <game>` | `workshop_status` |
| Workshop games supported and detected | Publish game chips | `publish --games` | `publish_workshop {"list_games": true}` |
| My items: search, sort, filter, tags | Publish › My items | `workshop items <game> [-s] [--sort] [--reverse] [--visibility] [--tag]` | `workshop_items` |
| Item stats, chart points, version history | Item details panel | `workshop item <game> <id>` | `workshop_item` |
| Create an item | Create popup | `publish <spec.json>` (no `item_id`) | `publish_workshop` + `confirm` |
| Edit title, description, tags, visibility | Edit popup | `publish <spec.json>` with `item_id` and only the changed fields | `publish_workshop` + `confirm` |
| Update content from a folder | Edit popup › folder | spec `content` = folder or `.gma` | same |
| Preview image check and crop | Image previewer | `workshop preview <image>` (the crop happens on publish) | `check_preview` |
| BBCode description check and render | Description editor + live preview | `workshop bbcode <file\|->` (plain text render) | `check_bbcode` |
| Dry run before upload | Create/edit popup checks | `publish --dry-run` | `publish_workshop {"dry_run": true}` |
| Update check | Update button | `version --check` | `check_update` |
| Install an update | Restart to update | `self-update` | none: the server never replaces itself; tell the user to run `prybar self-update` |
| Analytics on/off, sent-data log, clear log | Settings › Privacy | `privacy [--on\|--off] [--clear-log]` | `privacy` (+ `confirm` to change) |

Formats used everywhere:

- Descriptions are Steam BBCode: `[h1] [h2] [h3] [b] [i] [u] [strike] [spoiler] [noparse] [hr] [url=…] [img] [list] [olist] [*] [quote=…] [code] [table] [tr] [th] [td]`, at most 7999 characters. Unknown or unclosed tags show as text.
- Titles at most 128 characters, at most 32 tags of 255 characters. Garry's Mod tags: `Addon`, `gamemode`, `map`, `weapon`, `vehicle`, `npc`, `entity`, `tool`, `effects`, `model`, `servercontent`, `fun`, `roleplay`, `scenic`, `movie`, `realism`, `cartoon`, `water`, `comic`, `build`. TF2: `Maps`.
- Visibility: `public`, `friendsonly`, `private` (default), `unlisted`.
- Preview: PNG, JPEG or GIF under 32 MB; still images are center-cropped to 512x512; a GIF must be under 1 MB.
