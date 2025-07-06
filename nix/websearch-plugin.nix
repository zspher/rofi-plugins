{
  rustPlatform,
  lockFile,
  just,
  rofi,
  pkg-config,
  glib,
  cairo,
  pango,
  curl,
}:
let
  pname = "rofi-websearch";
  cargoToml = builtins.fromTOML (builtins.readFile ../${pname}/Cargo.toml);
  version = cargoToml.package.version;
in
rustPlatform.buildRustPackage {
  inherit version pname;
  src = ../.;
  cargoLock.lockFile = lockFile;

  nativeBuildInputs = [
    rustPlatform.cargoSetupHook
    just
    rofi
    pkg-config
  ];

  strictDeps = true;

  buildInputs = [
    glib
    cairo
    pango
    curl
  ];

  env = {
    PKGNAME = pname;
    PLUGINS_DIR = "/lib/rofi";
    PKGDIR = placeholder "out";
  };
}
