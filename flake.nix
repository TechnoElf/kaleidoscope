{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs-mozilla.url = "github:mozilla/nixpkgs-mozilla";
  };

  outputs = { self, nixpkgs, flake-utils, nixpkgs-mozilla, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ nixpkgs-mozilla.overlays.rust ];
      };
    in with pkgs; {
      devShell = mkShell rec {
        buildInputs = [
          (rustChannelOfTargets "nightly" "2023-03-07" [ "x86_64-unknown-linux-gnu" ])
          cmake
          pkg-config
          fontconfig
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libX11
          libxkbcommon
          libGL
        ];

        LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
      };

      packages.default = mkDerivation {};
    }
  );
}
