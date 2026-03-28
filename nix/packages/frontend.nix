{
  buildNpmPackage,
  wasm,
  ...
}:
buildNpmPackage {
  pname = "audio-experiments";
  version = "1.0.0";

  src = ../frontend;

  postUnpack = ''
    mkdir -p ./wasm/pkg
    cp -r ${wasm}/* ./wasm/pkg/
  '';

  npmDepsHash = "sha256-1YGoicXdYKq+p3Gn3ym8DOCmmCvw9NihPA2bzcL3YsI=";

}
