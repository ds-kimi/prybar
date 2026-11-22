# Garry's Mod Workshop publishing

Behavior notes for `prybar-workshop`, learned by reading how existing
publishers (gmpublisher, gmad) behave. Facts only; no code was taken.

## Item content

- Steam uploads a folder. For Garry's Mod that folder holds exactly one
  `.gma`; prybar packs an addon folder (or copies a `.gma`) into a temp
  folder and points Steam at it.
- `addon.json` at the addon root: `title` (non-empty), `type` one of
  `servercontent gamemode map weapon vehicle npc entity tool effects model`,
  `tags` at most two of `fun roleplay scenic movie realism cartoon water
  comic build`, optional `ignore` globs and `description`.
- Workshop tags are `Addon`, the type, then the tags. The in-game browser
  filters on them.
- Files must match the game's whitelist (gmad's list: `lua/*.lua`,
  `materials/*.vmt`, `models/*.mdl`, `sound/*.wav`, ...). `*.sw.vtx`,
  `*.360.vtx` and `*.xbox.vtx` are dropped even though `models/*.vtx`
  matches. Paths are lowercased; `*` crosses `/`.

## Preview

- Shown as a 512x512 square; Steam refuses files of 1 MB or more.
- Still images that are not 512x512 are center-cropped and resized, then
  saved as JPEG. GIFs are sent as they are (resizing would drop the
  animation), so they must already be under 1 MB.

## Steam flow

- `CreateItem` (app 4000, community file type) returns the id and whether
  the user still has to accept the Workshop legal agreement; until then the
  item stays hidden.
- One `SubmitItemUpdate` carries title, description, tags, visibility,
  preview and content. Its progress goes through preparing config,
  preparing content, uploading content, uploading preview, committing
  changes, with byte counts while uploading. There is no cancel.
- "My items": a user query on the published list, newest update first,
  with preview URL and subscriber count per item.
- Detection: init fails when Steam is not running; then logged on, then
  app ownership. The SDK reads the app id from the `SteamAppId`
  environment variable, so no `steam_appid.txt` is written.
