{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-24.11";
    parts.url = "github:hercules-ci/flake-parts";
    naersk.url = "github:nix-community/naersk";
    rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs@{ nixpkgs, parts, rust, naersk, ... }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];

      perSystem = { system, ... }:
        let
          inherit (nixpkgs) lib;
          overlays = [ (import rust) ];
          pkgs = import nixpkgs { inherit system overlays; };
          toolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile
            ./rust-toolchain.toml;
          naersk-lib = pkgs.callPackage naersk {
            cargo = toolchain;
            rustc = toolchain;
          };

          bevyDeps = with pkgs;
            [
              libgcc
              pkg-config
              alsa-lib-with-plugins
              libudev-zero
              libxkbcommon
              wayland
              vulkan-loader
            ] ++ (with xorg; [ libX11 libXcursor libXi ]);
        in {
          devShells.default = pkgs.mkShell {
            LD_LIBRARY_PATH = lib.makeLibraryPath bevyDeps;
            packages = bevyDeps
              ++ (with pkgs; [ toolchain just bacon nil nixfmt-classic taplo ]);
          };
          packages.default = naersk-lib.buildPackage { src = ./.; };
        };
    };
}
