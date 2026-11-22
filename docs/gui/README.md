# prybar-gui

Desktop interface for prybar, built with GPUI. It calls `prybar-service` like
the CLI and the MCP server do.

## Run

```sh
cargo run -p prybar-gui --release -- --sample
```

- `--sample` writes a generated model and VPK to the temp folder and opens them.
- `--screen <name>` opens on a screen by its bar label (`model`, `package`,
  `decompile`, `compile`, `publish`, `settings`, `"3d preview"`, `textures`).
- `--size <w>x<h>` opens the window at that size in points (for example
  `900x600`), to check layouts at a fixed size.
- Other arguments are files to open, including a model inside a package:
  `pak_dir.vpk:models/props/crate.mdl`. Files can also be dropped on the window.

The app is dark by default. The theme choice is saved in `gui.txt` next to the
games config (`PRYBAR_CONFIG` moves both).

## Keys

`cmd` on macOS, `ctrl` elsewhere.

| Key | Action |
|---|---|
| cmd-o | Open files |
| cmd-k, cmd-shift-p | Command palette |
| cmd-f | Search |
| cmd-shift-l | Light or dark theme |
| esc, cmd-[ | Back |
| tab / shift-tab | Move focus |
| cmd-q | Quit |

In the 3D preview, once it has focus (click it):

| Key | Action |
|---|---|
| f | Frame the model |
| r | Reset the camera |
| shift-f | Full size preview |
| w | Wireframe |
| b | Bones |
| space | Play or pause the sequence |

## Mouse and trackpad

- Left drag orbits; the camera keeps gliding briefly after release.
- Right, middle or shift drag pans. Two-finger trackpad scroll pans.
- Wheel or pinch zooms toward the cursor; cmd or ctrl with two fingers
  zooms too.
- Double-click a point on the model to orbit around it.
- In the texture preview, drag or two fingers pan, wheel or pinch zooms
  toward the cursor, double-click or Fit shows the whole texture.

## What needs a game install

Nothing to browse: models, packages, decompile, the 3D preview and the
texture preview work on any files. Textures are found next to the model,
in the same package, or in the games set up in Settings, so models from a
game show their textures once that game is detected. Compile needs a game
with its studiomdl (and Wine off Windows). Publish needs Steam.

## Performance

The 3D preview renders on the GPU (wgpu) off the UI thread. A 8744
triangle character renders in about 1.7 ms per frame at 2400x1500 on an
Apple Silicon Mac, well over 60 fps; the frame time is shown in the
corner of the preview. Measure on your own model with
`PRYBAR_BENCH_MDL=<model> cargo test -p prybar-gui --release bench -- --ignored --nocapture`.

## What works

Open and drop files, recent files, model inspector (version, bones,
meshes, sequences, materials, flex, body parts, companions found or missing),
package listing with filter and extract, decompile to a folder with split QC,
game list in settings and compile, live 3D preview with sequences, skins,
body groups and LODs, texture preview with mips and alpha.

Compile runs the picked game's studiomdl (through Wine or `PRYBAR_WINE` off
Windows) with the log streamed line by line, cancel and copy. Settings
detects games and edits a game's folder and studiomdl.

The Steam badge at the right of the top bar shows your Steam picture and
name with a green dot when publishing works and a red one when it does not;
its tooltip says why (not running, not logged in, game not owned, no Steam
support in this build) and a click checks again. Every button, toggle and
control explains itself in a tooltip after a short pause.

Publish repeats what blocks Steam, if anything, with Retry, lists your Garry's Mod items (a click fills the id and
title to update one), takes an addon folder (its `addon.json` type and tags
and every file the game refuses are listed as you type) or a `.gma`, and
fits a png/jpg preview to 512x512 under 1 MB (a GIF goes as is and must be
under 1 MB). Empty tags take `Addon`, the type and the tags from
`addon.json`. Steam's stages and bytes show during upload; Steam has no
cancel for a submitted update. Release builds include Steam support; from
source: `cargo run -p prybar-gui --features workshop` and copy
`libsteam_api.dylib` (from `target/debug/build/steamworks-sys-*/out/`) next
to the binary.

### Manual Workshop test (owner only)

Tests only use a mocked Steam. One real upload checks the whole path:

1. Start Steam, log in with the account that owns Garry's Mod.
2. Build and run with Steam support (see above); open Publish. The line
   The badge must show your name with a green dot; Publish lists your items.
3. Make a test folder: `addon.json` with
   `{"title": "prybar test", "type": "tool", "tags": ["fun"]}` and
   `lua/autorun/prybar_test.lua` with `print("prybar")`.
4. Item id empty, title `prybar test`, visibility `Hidden`, content the
   folder, a preview png of any size. Dry run, then Publish.
5. Expect the stages, then "Published item <id>" and Open in Steam. On the
   page: private, preview 512x512, tags Addon / tool / fun.
6. Update: click the item in the list, change note `second`, Publish. The
   page shows the change note.
7. Delete the item on its Steam page.


## Updates and privacy

The app checks for updates at launch and installs them on restart (Settings >
Updates; `PRYBAR_NO_UPDATE_CHECK=1` turns the check off). Anonymous launch and
crash analytics are on by default with a first-launch opt-out; everything sent
is listed in Settings > Privacy. See [PRIVACY.md](../../PRIVACY.md).
