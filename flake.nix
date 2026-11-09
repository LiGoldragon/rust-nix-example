{
  description = "A Cargo workspace built one Nix derivation per crate on the build base.";

  inputs.base.url = "github:LiGoldragon/build-base";

  # Cargo.nix is generated from Cargo.lock by the build base's crate2nix and
  # committed, so evaluation imports no derivation:
  #   nix run github:LiGoldragon/build-base#crate2nix -- generate
  outputs = { self, base }: base.lib.workspaceOutputs { cargoNix = ./Cargo.nix; };
}
