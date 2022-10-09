{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };


  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        libPath = with pkgs; lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];
      in
      {
        # `nix build`
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ 
            vulkan-loader
            wayland
            wayland-protocols
            xorg.libX11 
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libxcb
            libglvnd
            libGL
            libGLU
          ];
          postInstall = ''
            wrapProgram $out/bin/rogue-like-tutorial \
              --prefix LD_LIBRARY_PATH ":" "${libPath}"

          '';
        };

        # `nix run`
        defaultApp = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };

        # `nix develop`
        devShell = with pkgs; mkShell {
          buildInputs = [ 
            cargo 
            rustc 
            rustfmt 
            pre-commit 
            rustPackages.clippy 
            rust-analyzer
            tokei
            xorg.libxcb
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = libPath;
        };
      });
}
