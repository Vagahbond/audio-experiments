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
          ];
        };
      });

      # packages = forAllSystems (pkgs: {
      #   cli = pkgs.rustPlatform.buildRustPackage {
      #     name = "charpente";
      #     src = ./.;
      #     cargoLock = {
      #       lockFile = ./Cargo.lock;
      #     };
      #     buildInputs = with pkgs; [
      #       cargo
      #       rustc
      #       rustfmt
      #     ];
      #   };
      # });
    };
}
