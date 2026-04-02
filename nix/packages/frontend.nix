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

  npmDepsHash = "sha256-CxJcHNy9cgFUZcvVwOAX6U13qdqPOX5bunzuf/2b8z0=";

}
