# Unsafe inventory

Every crate inherits `unsafe_code = "forbid"` from the workspace
(`[workspace.lints.rust]` in `Cargo.toml`), so any `unsafe` block fails the
build. To add one, lift the lint for that crate only, justify the block in a
`// SAFETY:` comment, and list it here.

| Crate | File | Why | Invariant |
|---|---|---|---|
| (none) | | | |

The fuzz harness (`fuzz/`) is outside the workspace and uses no `unsafe`.
