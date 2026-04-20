{
  buildNpmPackage,
  wasm,
  ...
}:
buildNpmPackage {
  pname = "audio-experiments";
  version = "1.0.0";

  src = ../../frontend;

  preConfigure = ''
    mkdir -p ../wasm/pkg
    cp -r ${wasm}/* ../wasm/pkg/
  '';

  installPhase = ''
    mkdir -p $out
    cp -r ./build/* $out/
  '';

  npmDepsHash = "sha256-h7VGXwNT8YRD/W5obaec+x6NaP1FAF/tR/li+kGga10=";

}
