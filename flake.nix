{
  description = "A simple terminal file viewer written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      forAllSystems = nixpkgs.lib.genAttrs systems;

    in {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "pretty_files";
            version = "3.0.0";

            src = pkgs.lib.cleanSource ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            meta = {
              description = "A simple terminal file viewer with syntax highlighting and binary viewing support";
              homepage = "https://github.com/yok1rai/pretty_files";
              license = pkgs.lib.licenses.gpl3Plus;
              mainProgram = "pf";
              platforms = pkgs.lib.platforms.linux;
            };
          };
        });
    };
}
