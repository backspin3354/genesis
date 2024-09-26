{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    parts.url = "github:hercules-ci/flake-parts";
    parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    crane.url = "github:ipetkov/crane";
    
    rust.url = "github:oxalica/rust-overlay";
    rust.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ {
    nixpkgs,
    parts,
    crane,
    rust,
    ...
  }:
  parts.lib.mkFlake { inherit inputs; } {
    systems = [
      "x86_64-linux"
    ];

    perSystem = {
      system,
      ...
    }: let 
      pkgs = nixpkgs.legacyPackages.${system}.extend rust.overlays.default;

      toolchain = pkgs.rust-bin.stable.latest.minimal;      
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

      devToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" "rust-analyzer" ];
      };
      devCraneLib = (crane.mkLib pkgs).overrideToolchain devToolchain;

      buildDeps = with pkgs; [  
        clang
        mold
      ];

      runtimeDeps = with pkgs; [
        # TODO
      ];
    
      craneArgs = {
        src = craneLib.cleanCargoSource ./.;
        strictDeps = true;

        nativeBuildInputs = buildDeps;          
        buildInputs = runtimeDeps;
      };

      package = craneLib.buildPackage (craneArgs // {
        cargoArtifacts = craneLib.buildDepsOnly craneArgs;
      });
    in {
      packages.default = package;
      
      devShells.default = devCraneLib.devShell {
        packages = [
          # TODO
        ] ++ buildDeps;
      };
    };
  };
}
