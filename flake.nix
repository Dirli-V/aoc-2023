{
  description = "Flake for aoc-2023";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    nix-filter.url = "github:numtide/nix-filter";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    nix-filter,
    crane,
    fenix,
  }:
    flake-utils.lib.eachSystem ["x86_64-linux" "aarch64-linux"] (system: let
      pkgs = import nixpkgs {inherit system;};
      craneLib = crane.lib.${system}.overrideToolchain fenix.packages.${system}.stable.toolchain;

      pkgDef = {
        src = nix-filter.lib.filter {
          root = ./.;
          include = [
            ./src
            ./Cargo.toml
            ./Cargo.lock
          ];
        };
        nativeBuildInputs = with pkgs; [pkg-config autoPatchelfHook];
        buildInputs = [];
        runtimeDependencies = [];
      };

      cargoArtifacts = craneLib.buildDepsOnly pkgDef;
      aoc2023 = craneLib.buildPackage (pkgDef
        // {
          inherit cargoArtifacts;
        });
    in {
      checks = {
        inherit aoc2023;
      };

      packages.default = aoc2023;

      apps.default = flake-utils.lib.mkApp {
        drv = aoc2023;
      };

      devShells.default = pkgs.mkShell rec {
        inputsFrom = builtins.attrValues self.checks.${system};
        LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath (builtins.concatMap (d: d.runtimeDependencies) inputsFrom);
      };
    });

  nixConfig = {
    extra-trusted-substituters = ["https://nix-community.cachix.org"];
    extra-trusted-public-keys = ["nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="];
  };
}
