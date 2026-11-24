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

2531 meshes are decoded (`legacy/geometry/`): each model stores its own
vertices after the 128-byte name (type, radius, mesh count/offset, vertex
count/offset, tangent offset, vertex format, position offset and scale).
Meshes are 60 B (material, model offset, vertex count, first vertex).
Vertex formats:

| Format | Size | Layout |
|---|---|---|
| 0 skinned | 44 B | 3 weight bytes, influence selector (count = sel % 5, min 1; 4th weight = 255 - others), 4 x i16 bones, position, normal, uv floats |
| 1 unskinned | 12 B | u16 position x3 (scale + offset), u16 byte offset into the fine normal table, u16 uv / 65535 |
| 2 compressed | 8 B | u8 position x3 (/ 255, scale + offset), u8 index into the coarse normal table, u16 uv / 65535 |

The normal tables are rebuilt from their rule: rings of unit vectors from
-Z to +Z, ring k of n holding 4 * floor(n / 2 * sin(k pi / n) + 0.9)
steps (n = 64, 5314 entries, 12-byte stride; n = 12, 202 entries).
Triangles come from VTX v107 (`.dx80.vtx`): i16 counts, 8 B mesh, 20 B
strip group, 12 B vertex (3 bone ids, mesh vertex id at byte 10), 16 B
strip; flag 0x10 marks static-prop groups whose vertices are bare u16 ids.

2531 facial data (`legacy/face/`): header ints 1-2 are the `$phonemefilter`
floats; flex descs, controllers and rules use the v44 records at int slots
29, 31 and 33. Mesh words 4-5 are the flex count and mesh-relative index,
words 6-7 the material type and parameter. A flex is 32 B (desc, 4 target
floats, i16 count, offset, i16 type); type 0 deltas are 20 B (u16 index,
float position, u16 normal direction + u8 length), type 1 deltas 8 B
(u16 index, u16 position and normal directions, u8 lengths). Directions
index the fine normal table; lengths are /255 x 8 (position) or x 2
(normal). Eyeballs are 140 B at model words 16-17 with no material: eye i
takes the material of the mesh with type 1 and parameter i. The QC gets
`$phonemefilter`, `eyeball` lines and the VTA like any v44 model.

Not decoded, listed in `ModelIr::notes`: meshes for v27-v37 (vertex data
lives in the MDL in old layouts), animations, sequences, flex controllers (except 2531), pose
parameters, procedural bone rules, and attachments for v27-v31.
