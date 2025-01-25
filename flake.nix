{
    description = "A personal utility tool for my custom DE.";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system:
        let
            pkgs = import nixpkgs { inherit system; };
        in {
            # nix build or nix run functionality
            packages.default = pkgs.rustPlatform.buildRustPackage {
                pname = "utiltool";
                version = "0.1.0";

                src = pkgs.lib.cleanSource ./.;

                cargoLock.lockFile = ./Cargo.lock;

                nativeBuildInputs = [ pkgs.installShellFiles ];
                
                # Install shell completions
                postInstall = ''
                    installShellCompletion target/*/release/build/*/out/utiltool.{bash,fish,elv}
                '';
            };

            # nix develop functionality
            devShell = pkgs.mkShell {
                buildInputs = [
                    pkgs.rustc
                    pkgs.cargo
                    pkgs.rust-analyzer
                ];
                RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            };
        }
    );
}
