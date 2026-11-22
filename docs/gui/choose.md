# Choosing a GUI direction

`prybar-gui` ships three full designs over the same state and actions, so the
comparison is about design only. Pick one; later work continues in that
direction and the other two get deleted.

## Run

```sh
cargo run -p prybar-gui --release -- --sample
```

`--sample` opens two synthetic files (a model and a package) so every screen
has data. Drop real `.mdl`, `.vpk` or `.gma` files on the window, or pass them
as arguments. `--design workbench|canvas|console` skips the chooser.

## Keys

`cmd` on macOS, `ctrl` elsewhere.

| Key | Action |
|---|---|
| cmd-1 / cmd-2 / cmd-3 | Workbench / Canvas / Console, live |
| cmd-0 | Back to the chooser |
| cmd-o | Open files |
| cmd-k, cmd-shift-p | Command palette (Console: command bar) |
| cmd-f | Search |
| cmd-shift-l | Light / dark theme (Console stays dark) |
| esc, cmd-[ | Back |
| tab / shift-tab | Move focus |
| cmd-q | Quit |

## The three designs

- **Workbench**: dense pro tool. Screen list and open files on the left, tabs
  in the middle, inspector on the right, status bar with key hints. Most data
  on screen at once.
- **Canvas**: friendly visual flow. Top nav, a step path (drop, inspect,
  decompile, done), file cards, big stat cards, chips and toasts. Warm light
  theme, springy motion.
- **Console**: keyboard first. One command bar with fuzzy commands and typed
  paths, sliding panes, monospace output and key hints in the footer.

## Real vs placeholder

Real (through `prybar-service`): open and drop files, recent files, model
inspector (version, bones, meshes, sequences, materials, flex, body parts,
companions found or missing), package listing with search and extract,
decompile to a folder with split-QC, game list in settings and compile.

Placeholder (animated, labelled regions): 3D model preview, texture preview,
compile log stream, workshop upload progress. The publish form is laid out
but does not upload.

Screenshots: `docs/evidence/gui-F/` (chooser, workbench, canvas, console).
