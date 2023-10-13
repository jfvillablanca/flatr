{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    devenv.url = "github:cachix/devenv";
  };

  outputs = { self, nixpkgs, devenv, flake-utils, ... } @ inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        pname = "flatr";
        version = "0.1.0";
      in
      {
        devShells.default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              languages.rust.enable = true;
              packages = with pkgs; [
                cargo-watch
              ];
            }
          ];
        };
        packages.default = pkgs.rustPlatform.buildRustPackage {
          inherit pname version;
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      });
}
