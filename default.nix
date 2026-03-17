{ lib,
  rustPlatform,
  pkg-config,
  makeWrapper,
  git,
  openssh,
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
    makeWrapper
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

  postInstall = ''
    wrapProgram $out/bin/nixos-config-puller \
      --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath buildInputs} \
      --prefix PATH : ${lib.makeBinPath [git openssh]}
  '';

  meta = with lib; {
    description = "NixOS config puller";
    platforms = platforms.linux;
  };
}
