{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    naersk,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in rec {
        # nix build
        packages.genpasswd = naersk-lib.buildPackage {
          pname = "genpasswd";
          root = ./.;
        };
        packages.default = packages.genpasswd;

        # nix run
        apps.genpasswd = flake-utils.lib.mkApp {
          drv = packages.genpasswd;
        };
        apps.default = apps.genpasswd;

        # nix develop
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [cargo rustfmt clippy];
        };
      }
    );
}
