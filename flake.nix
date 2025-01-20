{
  description = "A personal utility tool for my custom DE.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs"; # Use the latest nixpkgs
    flake-utils.url = "github:numtide/flake-utils"; # Utility library for flakes
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs { inherit system; };
  in {
    # nix build or nix run functionality
    packages.default = pkgs.rustPlatform.buildRustPackage {
      pname = "utiltool";
      version = "0.1.0";

      # Path to the Cargo project
      src = ./.;

      # Optional: Overrides for specific dependencies
      cargoPatches = [ ];

      # Additional build inputs
      nativeBuildInputs = with pkgs; [ ];
    };

    # nix develop functionality
    devShell = pkgs.mkShell {
        buildInputs = [
            pkgs.rustc        # Rust compiler
            pkgs.cargo        # Rust package manager
            pkgs.rust-analyzer # Rust language server for IDE support
        ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
  });
}
