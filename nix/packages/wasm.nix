{
  rustPlatform,
  pkgs,
  ...
}:
rustPlatform.buildRustPackage {
  name = "audio-experiments";
  src = ../../wasm;

  WASM_PACK_CACHE = "/tmp/wasm-pack-cache";
  CARGO_HOME = "/tmp/cargo-home";

  cargoLock = {
    lockFile = ../../wasm/Cargo.lock;
  };

  # buildInputs = with pkgs; [
  # ];

  nativeBuildInputs = with pkgs; [
    # wasm-bindgen-cli
    wasm-bindgen-cli_0_2_114
    binaryen
    tree
    wasm-pack
    lld
  ];

  # outputHashes = {
  #   "wasm-bindgen-0.2.114" = pkgs.lib.fakeHash;
  # };

  buildPhase = ''
    mkdir -p $WASM_PACK_CACHE $CARGO_HOME

    make release
  '';

  installPhase = ''
    mkdir -p $out
    cp -r ./pkg/* $out/
  '';
}
