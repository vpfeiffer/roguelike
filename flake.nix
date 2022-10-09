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
      in
      {
        # `nix build`
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          postFixUp = ''
            wrapProgram $out/bin/rogue-like-tutorial \
              --prefix LIBGL_DRIVERS_PATH ":" "${pkgs.mesa.drivers}/lib/dri" \
              --prefix LD_LIBRARY_PATH ":" "${pkgs.mesa.drivers}/lib"

          '';
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ 
            vulkan-loader
            wayland
            wayland-protocols
            xorg.libX11 
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            libglvnd
            libGL
            libGLU
          ];
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
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
