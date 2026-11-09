{
  description = "A Cargo workspace built one Nix derivation per crate on the build base.";

  inputs.base.url = "github:LiGoldragon/build-base";

  # Cargo.nix is generated from Cargo.lock by the build base's crate2nix and
  # committed, so evaluation imports no derivation:
  #   nix run github:LiGoldragon/build-base#crate2nix -- generate
  outputs =
    { self, base }:
    base.lib.workspaceOutputs {
      cargoNix = ./Cargo.nix;

      # clippy over every target of each member under its default features,
      # the workspace's [lints] applied, warnings refused; rustdoc over each
      # library, warnings refused.
      extraChecks =
        { clippy, doc, ... }:
        {
          clippy-word = clippy "example-word" [ "default" "json" ] [ "-D" "warnings" ];
          clippy-number = clippy "example-number" [ "default" ] [ "-D" "warnings" ];
          clippy-sentence = clippy "example-sentence" [ "default" ] [ "-D" "warnings" ];
          doc-word = doc "example-word" [ "default" "json" ] [ "-D" "warnings" ];
          doc-number = doc "example-number" [ "default" ] [ "-D" "warnings" ];
          doc-sentence = doc "example-sentence" [ "default" ] [ "-D" "warnings" ];
        };
    };
}
