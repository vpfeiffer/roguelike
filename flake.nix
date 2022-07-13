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
          buildInputs = with pkgs; [ xorg.libX11 ];
        };

        # `nix run`
        defaultApp = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };

        # `nix develop`
        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy rust-analyzer ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
