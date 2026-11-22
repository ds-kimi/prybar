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
detects games and edits a game's folder and studiomdl. Publish validates
with Dry run; real uploads need a build with the Steamworks SDK:
`cargo run -p prybar-gui --features workshop`, Steam running and logged in.
Without them the screen says which one is missing.

