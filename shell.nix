{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    wayland
    wayland.dev
    wayland-protocols
    libxkbcommon
    libxkbcommon.dev
    xorg.libX11
    xorg.libxcb
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    vulkan-loader
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.wayland pkgs.libxkbcommon pkgs.xorg.libX11 pkgs.xorg.libxcb pkgs.vulkan-loader ];
}
