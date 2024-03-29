{
  description = "Bitbucket merging assistant";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs-unstable, flake-utils, ... }:
    let
      version = "0.2.1";
      supportedSystems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
      forAllSystems = nixpkgs-unstable.lib.genAttrs supportedSystems;

      nixpkgsFor = forAllSystems (system: import nixpkgs-unstable { inherit system; });

    in
    {
      packages = forAllSystems (system:
        let pkgs = nixpkgsFor.${system};
        in
        {
          toke-rs = pkgs.rustPlatform.buildRustPackage {
            pname = "toke-rs";
            inherit version;
            src = ./.;
            cargoSha256 = "sha256-20pL74xQ2vSSdy6VicbthINf7i+8sK7QgpF6zBC1XiA=";

            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl.dev ];

            meta = with pkgs.lib; {
              description = "Never let your vault token expire";
              license = licenses.mit;
            };
          };
        });

      defaultPackage = forAllSystems (system: self.packages.${system}.toke-rs);

      defaultApp = forAllSystems (system: {
        type = "app";
        program = "${self.packages.${system}.toke-rs}/bin/toket";
      });

      devShell = forAllSystems (system:
        let
          system = "x86_64-linux";
          pkgs = nixpkgsFor.${system};
        in
        pkgs.mkShell {
          packages = with pkgs; [
            rust-analyzer
            cargo
            rustup
            bashInteractive
            openssl.dev
            pkg-config
            python39Packages.grip
            gdb
          ];
        });
    };
}
