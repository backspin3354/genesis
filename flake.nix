{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    parts.url = "github:hercules-ci/flake-parts";
    parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    crane.url = "github:ipetkov/crane";
    
    rust.url = "github:oxalica/rust-overlay";
    rust.inputs.nixpkgs.follows = "nixpkgs";

    filter.url = "github:numtide/nix-filter";
  };

  outputs = inputs @ {
    nixpkgs,
    parts,
    crane,
    rust,
    filter,
    ...
  }:
  parts.lib.mkFlake { inherit inputs; } {
    systems = [
      "x86_64-linux"
    ];

    perSystem = {
      system,
      lib,
      ...
    }: let 
      pkgs = nixpkgs.legacyPackages.${system}.extend rust.overlays.default;

      toolchain = pkgs.rust-bin.stable.latest.minimal;      
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
      
      runtimeDependencies = with pkgs; [
        wayland
        libxkbcommon
        libglvnd # GL
        vulkan-loader # Vulkan
      ];

      craneArgs = {
        src = filter.lib.filter {
          root = ./.;
          include = [
            "./.cargo"
            "./src"
            "./Cargo.toml"
            "./Cargo.lock"
          ];
        };
        strictDeps = true;

        # Dependencies used during build
        nativeBuildInputs = with pkgs; [
          pkg-config
          autoPatchelfHook
          clang
          mold
        ];

        # Dependencies used during build AND during runtime
        buildInputs = with pkgs; [
          stdenv.cc.cc.lib # libgcc_s.so.1
        ];
      
        # Dependencies used during runtime
        inherit runtimeDependencies;
      };

      package = craneLib.buildPackage (craneArgs // {
        cargoArtifacts = craneLib.buildDepsOnly craneArgs;
      });
    in {
      packages.default = package;
            
      devShells.default = let 
        toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
      in craneLib.devShell {
        LD_LIBRARY_PATH = "${lib.makeLibraryPath runtimeDependencies}";
        inputsFrom = [ package ];
      };
    };
  };
}
