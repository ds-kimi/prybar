# Format sources

Prybar is a clean-room implementation: every parser and writer is our own code, written from public documentation, never ported from another tool. Real game files are read only from the local install and never committed.

| Format | Public sources used |
|---|---|
| MDL / VVD / VTX / PHY / ANI (v44-49, v52/53) | Valve Developer Community wiki (MDL, VVD, VTX, PHY pages); Source SDK 2013 headers `studio.h`, `optimize.h`, `phyfile.h` |
| MDL legacy Source (2531, 31, 32, 36, 37) and GoldSrc (v10, v04/06/14) | VDC wiki, Half-Life SDK `studio.h`; see `docs/formats/mdl-legacy.md`, `docs/formats/mdl-goldsrc.md` |
| QC / SMD / VTA / VRD output | VDC wiki pages on QC commands and the SMD/VTA formats |
| VPK v1/v2 | VDC wiki VPK page |
| GMA | Facepunch/Garry's Mod wiki GMA description |
| APK, HFS | Observed layout; HFS filename key table taken as format data, provenance in `docs/formats/hfs.md` |
| Steam Workshop | Steamworks SDK documentation; behavior of existing publishers (gmpublisher, gmad) observed, no code taken: `docs/formats/workshop.md` |

Prybar reads a Crowbar clone locally only to learn expected output and version quirks. Its design differs: one version-neutral IR, shared traits per version family, parallel batch work.
