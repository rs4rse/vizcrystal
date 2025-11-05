{
  description = "Bevy-based";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    oxalica.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      oxalica,
    }:
    with flake-utils.lib;
    eachSystem allSystems (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system}.extend oxalica.overlays.default;
      in
      rec {

        packages = {
          bevy-program =
            let
              rustPlatform = pkgs.makeRustPlatform {
                cargo = pkgs.rust-bin.stable.latest.minimal;
                rustc = pkgs.rust-bin.stable.latest.minimal;
              };
            in
            rustPlatform.buildRustPackage rec {
              name = "vizmat";
              src = self;
              nativeBuildInputs = with pkgs; [ pkg-config ];
              buildInputs = with pkgs; [
                alsa-lib.dev
                udev.dev
                xorg.libX11
                xorg.libXrandr
                xorg.libXcursor
                xorg.libxcb
                xorg.libXi
                wayland
                libxkbcommon
                libxkbcommon.dev
                vulkan-loader
                vulkan-tools
                glfw
              ];
              cargoLock = {
                lockFile = ./Cargo.lock;
              };
              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
            };
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            (rust-bin.stable."1.88.0".complete.override {
              targets = [
                "x86_64-unknown-linux-gnu"
                "wasm32-unknown-unknown"
              ];
            })
            nodejs_20
            trunk
            wasm-pack
            pkg-config
            alsa-lib.dev
            udev.dev
            xorg.libX11
            xorg.libXrandr
            xorg.libXcursor
            xorg.libxcb
            xorg.libXi
            wayland
            libxkbcommon
            libxkbcommon.dev
            vulkan-loader
            vulkan-tools
            glfw
          ];

        };

        defaultPackage = packages.bevy-program;
        formatter = pkgs.nixfmt;
      }
    );
}
