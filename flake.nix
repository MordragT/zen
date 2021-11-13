{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, utils, naersk, fenix }: let
    overlay = (final: prev: {
      zen = prev.callPackage (import ./default.nix) {
        inherit naersk fenix;
      };
    });
  in { overlay = overlay; } // utils.lib.eachDefaultSystem (system: let
    pkgs = nixpkgs.legacyPackages."${system}";
    naersk-lib = naersk.lib."${system}";
  in rec {
    # `nix build`
    packages.zen = import ./default.nix {
      inherit system;
      inherit (nixpkgs) lib;
      inherit pkgs;
      inherit naersk fenix;
    };
      
    defaultPackage = packages.zen;

    # `nix run`
    apps.zen = utils.lib.mkApp {
      drv = packages.zen;
    };
    defaultApp = apps.zen;

    # `nix develop`
    devShell = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        pkgconfig
        vulkan-tools
        vulkan-loader
        vulkan-headers
        vulkan-validation-layers
          
        xorg.libX11
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
      ];
    };
  });
}
