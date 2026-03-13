{
  description = "nixos-config-puller";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default;

        nativeBuildInputs = with pkgs; [
          pkg-config
          rustToolchain
        ];

        buildInputs = with pkgs; [
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
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "nixos-config-puller";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit nativeBuildInputs buildInputs;

          # Keep build output clean; binaries link to runtime libs below.
          doCheck = false;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };

        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.xorg.libX11
            pkgs.xorg.libxcb
            pkgs.vulkan-loader
            pkgs.fontconfig
          ];

          shellHook = ''
            echo "Rust toolchain: $(${rustToolchain}/bin/rustc --version)"
            echo "You can now run: cargo build"
          '';
        };

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}