{
    description = "A personal utility tool for my custom DE.";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system:
        let
            pkgs = import nixpkgs { inherit system; };
            cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        in {
            # nix build or nix run functionality
            packages.default = pkgs.rustPlatform.buildRustPackage {
                pname = "utiltool";
                version = cargoToml.package.version;

                src = pkgs.lib.cleanSource ./.;

                cargoLock.lockFile = ./Cargo.lock;

                nativeBuildInputs = [ pkgs.installShellFiles ];
                
                # Install shell completions (elv is also generated, but breaks installShellCompletion when trying to install it)
                postInstall = ''
                    installShellCompletion target/*/release/build/*/out/utiltool.{bash,fish}
                    installShellCompletion --zsh target/*/release/build/*/out/_utiltool
                    installManPage target/*/release/build/*/out/man/*
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
