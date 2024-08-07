{
  description = "A Game Engine for 3D old school games. Supports Gothic 1 & 2 Game File Formats.";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    fenix,
  }:
    utils.lib.eachDefaultSystem
    (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [fenix.overlays.default];
      };
      toolchain = pkgs.fenix.complete;
      packages = with pkgs; [
        openssl
        pkg-config

        vulkan-tools
        vulkan-loader
        vulkan-headers
        vulkan-validation-layers
        alsa-lib
        udev

        xorg.libX11
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
      ];
    in {
      packages.default =
        (pkgs.makeRustPlatform {
          inherit (toolchain) cargo rustc;
        })
        .buildRustPackage {
          pname = "zen";
          version = "0.0.1";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          buildInputs = packages;

          meta = with pkgs.lib; {
            description = "A Game Engine for 3D old school games. Supports Gothic 1 & 2 Game File Formats.";
            homepage = "https://github.com/MordragT/zen";
            license = licenses.mit;
            maintainers = with maintainers; [mordrag];
            mainProgram = "zen";
          };
        };

      devShells.default = pkgs.mkShell {
        nativeBuildInputs =
          (with toolchain; [
            cargo
            rustc
            rust-src
            clippy
            rustfmt
          ])
          ++ packages;

        RUST_SRC_PATH = "${toolchain.rust-src}/lib/rustlib/src/rust/library";
        LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath packages}";
      };
    })
    // {
      overlays.default = this: pkgs: {
        zen = self.packages."${pkgs.system}".default;
      };
    };
}
