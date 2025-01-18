{
  description = "rust-wasm-pitfalls";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        f = with fenix.packages.${system};
          combine [
            latest.toolchain
            targets.wasm32-unknown-unknown.latest.rust-std
          ];
      in {
        devShells.default = pkgs.mkShell {
          name = "rust-wasm-pitfalls";

          packages = with pkgs; [ f deno wasm-pack ];
        };
      });
}
