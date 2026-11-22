# GoldSrc MDL (v10, v14, v04/v06)

Sources: public Half-Life SDK `studio.h` (struct layouts), Crowbar ref
(read only, for v14 and the strip/fan winding it writes to SMD).

## v10 (decoded)

- Header: 61 words (244 bytes), all offsets absolute.
- Bones 112 B (32-byte name, parent, flags, 6 controller slots,
  value[6] = pos + euler radians, scale[6]). Attachments 88 B (origin
  only kept), hitboxes 32 B in one `default` set.
- Textures 80 B; pixels (w*h palette indices) then 768-byte RGB palette.
  Sizes above 4096 are rejected. `T.mdl` supplies textures/skins when
  the model has none (`Companions::textures`).
- Body parts 76 B, models 112 B, meshes 20 B. Vertices/normals are in
  bone space and are moved to model space with the bind pose. Triangle
  commands: signed i16 count (<0 fan, >0 strip, 0 end), corners
  `[vert, norm, s, t]`; UV = s/width, t/height. Strip winding
  (k, k+2, k+1) on even k, fan (0, k+2, k+1), as studiomdl expects.
- Sequences 176 B, events 76 B (frame turned into cycle). One animation
  per blend; per-bone `mstudioanim_t` of six u16 offsets into RLE
  `mstudioanimvalue_t` runs. Group `n > 0` reads `Companions::groups[n-1]`
  (`NN.mdl`); without it the animation keeps metadata and no frames.
- Not decoded: bone controllers, transitions, sounds (listed in notes).

## v14

Same header head, bones, sequences, attachments and hitboxes as v10;
mesh and texture structs differ and are not decoded (note in IR).

## v04 / v06

Alpha formats with unrelated layouts; `parse` returns `Unsupported`.
