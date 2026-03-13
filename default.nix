{
  lib,
  rustPlatform,
  pkg-config,
  wayland,
  wayland-protocols,
  libxkbcommon,
  xorg,
  vulkan-loader,
  fontconfig
}:

rustPlatform.buildRustPackage rec {
  pname = "nixos-config-puller";
  version = "0.1.0";

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    wayland
    wayland-protocols
    libxkbcommon
    xorg.libX11
    xorg.libxcb
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    vulkan-loader
    fontconfig
  ];

  meta = with lib; {
    description = "NixOS config puller";
    platforms = platforms.linux;
  };
}
