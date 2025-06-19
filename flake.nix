{
    description = "A personal utility tool for my custom DE.";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs";
        flake-utils.url = "github:numtide/flake-utils";
        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs = {
                nixpkgs.follows = "nixpkgs";
            };
        };
    };

    outputs = { self, nixpkgs, flake-utils, rust-overlay }: flake-utils.lib.eachDefaultSystem (system:
        let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs { inherit system overlays; };
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
            devShells.default = pkgs.mkShell {
                buildInputs = with pkgs; [
                    (rust-bin.stable.latest.default.override {
                        extensions = [ "rust-src" ];
                    })
                ];

                shellHook = ''
                    PS1="''${PS1/\\n/\\n(devenv) }"
                '';
            };
        }
    ) // {
        homeManagerModules.default = system:
            let
                pkgs = import nixpkgs {
                    inherit system;
                    overlays = [ (import rust-overlay) ];
                };
            in { config, ... }@args:
            import ./nix/hm-module.nix ({
                inherit pkgs;
                utiltoolPkg = self.packages.${system}.default;
                lib = pkgs.lib;
            } // args);
    };
}
