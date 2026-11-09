# rust-nix-example

A Cargo workspace built one Nix derivation per crate on
[build-base](https://github.com/LiGoldragon/build-base): crate2nix's committed
`Cargo.nix`, nixpkgs `buildRustCrate`, no import-from-derivation.

- `example-word` — a leaf; serde derive, thiserror, an optional `json` feature.
- `example-number` — a leaf independent of the others.
- `example-sentence` — a library and command over both, and over
  `example-signal`.
- `signal/` — `example-signal`, reached by the workspace as a git dependency
  the way `signal-flow` is reached by `flow`. It names `example-word` by this
  repository's git URL; the workspace's `[patch]` resolves that edge to its own
  path crate, as flow does for `flow-ethos`.

`nix build` builds every member; `nix flake check` runs every member's test
binaries and fails if `Cargo.nix` is not what the base's crate2nix generates
from this source. After editing `Cargo.toml` or `Cargo.lock`, regenerate `Cargo.nix`:
`nix run github:LiGoldragon/build-base#crate2nix -- generate`.
