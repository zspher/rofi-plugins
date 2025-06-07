{
  rustPlatform,
  lockFile,
  just,
  rofi,
  pkg-config,
  glib,
  cairo,
  pango,
  name,
}:
let
  cargoToml = builtins.fromTOML (builtins.readFile ../${name}/Cargo.toml);
  pname = cargoToml.package.name;
  version = cargoToml.package.version;
in
rustPlatform.buildRustPackage {
  inherit pname version;
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
  ];

  env = {
    PKGNAME = pname;
    PLUGINS_DIR = "/lib/rofi";
    PKGDIR = placeholder "out";
  };
}
