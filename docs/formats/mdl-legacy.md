# MDL legacy Source (2531, v27-v37)

Parsed by `crates/prybar-mdl/src/legacy/`. Layouts were worked out from the
Crowbar reference (read only, not ported) and are unverified against real
files: none were available.

| Version | Header | Bone | Texture | Model | Hitbox | Attachments |
|---|---|---|---|---|---|---|
| 32-37 | name 64, 6 vectors, ints at 152 | 196 B (euler, qAlignment, quat after tail) | 32 B | 140 B | 68 B (named) | 60 B |
| 27-31 | as 37 plus an old hitbox pair after bone controllers (slots +2) | 148 B (no quat, no contents) | 32 B | 120 B | 32 B | not read |
| 2531 | name 128, 7 vectors (unknown 3rd), ints at 228 | 136 B (quat instead of euler) | 20 B | 224 B (name 128) | 32 B | 60 B |

Recovered: header vectors, flags, mass (not 2531), surface prop, bones,
attachments, hitbox sets, materials, skins, body part and model names.

Not decoded, listed in `ModelIr::notes`: meshes (vertex data lives in the
MDL in old layouts), animations, sequences, flex controllers, pose
parameters, procedural bone rules, and attachments for v27-v31.
