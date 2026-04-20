{
  description = "My first WASM project";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nix-darwin.url = "github:nix-darwin/nix-darwin";
  };

  outputs =
    {
      nixpkgs,
      ...
    }@inputs:
    let
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-darwin"
        ] (system: f (import nixpkgs { inherit system; }));
    in
    {

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            lld
            rustfmt
            nodejs
            wasm-pack
            wasm-bindgen-cli_0_2_114
            binaryen
            tree
          ];
        };
      });

      packages = forAllSystems (pkgs: rec {
        wasm = pkgs.callPackage ./nix/packages/wasm.nix { };

        frontend = pkgs.callPackage ./nix/packages/frontend.nix {
          inherit wasm;
        };
        default = frontend;

      });
    };
}
