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
- Other arguments are files to open. Files can also be dropped on the window.

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

## Real vs placeholder

Real: open and drop files, recent files, model inspector (version, bones,
meshes, sequences, materials, flex, body parts, companions found or missing),
package listing with filter and extract, decompile to a folder with split QC,
game list in settings and compile.

Compile runs the picked game's studiomdl (through Wine or `PRYBAR_WINE` off
Windows) with the log streamed line by line, cancel and copy. Settings
detects games and edits a game's folder and studiomdl. Publish validates
with Dry run; real uploads need a build with the Steamworks SDK:
`cargo run -p prybar-gui --features workshop`, Steam running and logged in.
Without them the screen says which one is missing.

