# Round-trip report

199 / 200 models pass (99.5%).
Sample: 50 models with a `.vvd` per package (stride over the sorted list),
from `garrysmod_dir.vpk`, `hl2_misc_dir.vpk`, `content_hl2_dir.vpk`,
`content_cstrike_dir.vpk`. Command: `scripts/roundtrip/roundtrip.sh 50`.
Run 2026-11-22 on Windows 11 with GMod's studiomdl.

Fixed in the first pass (studiomdl rejected the decompiled QC):
- `$animation`, `$sequence`, `$model`, `flexfile`, `$bodygroup`, `$lod`,
  `$jigglebone` (+ sub-blocks), `$weightlist`, `$collisionmodel` /
  `$collisionjoints`: opening brace moved onto the command line
  (`bad command {` / `no animations found`). `$shadowlod` keeps it on the
  next line, which its parser requires.
- `predelta` no longer written inside `$animation` (`Unknown animation
  option 'predelta'`); the sequence still carries it.

Fixed in the second pass:
- HL2 faces (`eli`, `female_01`, `female_05`, `female_03_bloody`): lid
  frames become `eyelid` lines (lowerer key tagged -11, raiser 11,
  neutral = frame 0) bound to named eyeballs (`eye_right` / `eye_left`:
  the MDL stores no eyeball names); rule-only descs (`right_open`, ...)
  are declared with `localvar`; `<name>R+<name>L` stereo frames are
  written as `flexpair` like `<name>L+<name>R`.
- `$concave` hangs (carousel, coreball, flatcar) and `fanhousing`
  (`IVP Failed`): the physics SMD used face normals, so studiomdl welded
  nothing and saw 130+ loose parts. Normals are now averaged per vertex
  within each convex piece; hulls stay whole and apart.
- Vertex metric: counted per LOD over the vertices the VTX draws. Old
  builds keep unreferenced vertices (`lampfixture01a`: 12) and separate
  LOD copies (`eli`: equal per LOD, 7501 vs 6851 summed raw), which no
  decompile can carry over.

Remaining failure:
- `shadertest/vertexlitenvmap`: the shipped VVD holds 19 vertices that
  are exact duplicates (position, normal, UV, bone) of others; current
  studiomdl welds them. Not a decompiler loss.

| Model | Stage | Differences |
|---|---|---|
| `models/shadertest/vertexlitenvmap` | compare | vertices: 289 -> 270 |
