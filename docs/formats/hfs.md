# HFS (Vindictus packages)

prybar reads, lists and extracts HFS packages. It does not write them.

## Layout

HFS is a ZIP archive with `HF` in place of `PK` in every signature:

| Record | Signature | Fields used |
|---|---|---|
| Local header (30 bytes) | `0x04034648` | name length @26, extra length @28; data follows |
| Central record (46 bytes) | `0x02014648` | stored size @20, size @24, name length @28, extra @30, comment @32, local offset @42 |
| End of directory (22 bytes) | `0x06054648` | entry count @10, directory offset @16 |

The end record is searched backwards over the last 22 + 65535 bytes, as
in ZIP. Local header signatures are not checked because real packages
are not consistent about them.

## Names

Every name byte is XORed with `NAME_KEY[(file_offset + i) % 4096]`, where
`file_offset` is the absolute position of the name in the package.

## `.comp` entries

Entries whose name ends in `.comp` hold data in an undocumented compressed
form. prybar lists them; `read`/`extract_one` return `Error::Unsupported`
and `extract_all` skips them.

## Provenance

No public specification of HFS exists. The layout above was confirmed by
reading Crowbar's HFS reader (`Crowbar/Core/GamePackage/HfsFile/HfsFile.vb`).
The 4096-byte filename key in `crates/prybar-hfs/src/format/key_table.rs`
is copied from that file as format constant data, with the owner's
approval (2026-11-20). All prybar code is original.
