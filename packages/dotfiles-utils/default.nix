{rustPlatform}:
rustPlatform.buildRustPackage {
  pname = "dotfiles-utils";
  version = "0.1.0";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;
}
