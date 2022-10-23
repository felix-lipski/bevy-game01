{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-unstable;
    flake-utils.url = github:numtide/flake-utils;
  };

  outputs = { nixpkgs, self, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem
      (system:
      let 
        pkgs = import nixpkgs {
          config = { allowUnfree = true; };
          inherit system;
        };
      in {
        devShell =
          pkgs.mkShell rec {
            nativeBuildInputs = with pkgs; [
              pkgconfig
              llvmPackages.bintools # To use lld linker
            ];
            buildInputs = with pkgs; [
              udev alsaLib vulkan-loader
              xlibsWrapper xorg.libXcursor xorg.libXrandr xorg.libXi # To use x11 feature
              # libxkbcommon wayland # To use wayland feature
              vulkan-tools

              graphviz

            ];
            shellHook = ''
              '' ;
              # export LD_LIBRARY_PATH = ${pkgs.lib.makeLibraryPath buildInputs}
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          };
        }
      );
}
