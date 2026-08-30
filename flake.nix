{
  description = "Simple flake for rust"; # Will probably evolve soon

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs = {nixpkgs, ...}: let
    # here is a lesson in nix
    # Basically we make a function forAllSystems(function)
    # lib.genAttrs takes a list of items and maps them to something
    # imagine it like genAttrs(systems []string) []string
    # lib.systems.flakeExposed gives a list of common systems nix targets ([x86_64-linux, aarch64-darwin,...])
    # Then for each we generate pkgs for each system (ex: nixpkgs.legacyPackages.x86_64-linux)
    # This eliminates the need to import flake-utils, reducing a dependency, ok?
    forAllSystems = function:
      nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
        system: function nixpkgs.legacyPackages.${system}
      );
  in {
    packages = forAllSystems (pkgs: {
      default = import ./nix/shell.nix {inherit pkgs;};
    });
  };
}
