{ system
, lib
, pkgs
, naersk
, fenix
}:

let
  toolchain = with fenix.packages.${system};
    combine [
      latest.rustc
      latest.cargo
    ];
  naersk-lib = naersk.lib.${system}.override {
    cargo = toolchain;
    rustc = toolchain;
  };
in
naersk-lib.buildPackage {
  pname = "zen";
  root = ./.;
  buildInputs = with pkgs; [
        pkgconfig
          
        xorg.libX11
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
  ];
  meta = with lib; {
    description = "Zen - Game Engine";
  };
}
