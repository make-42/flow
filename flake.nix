{
  description = "Flow - An inline network speed stats util";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
    };
  in {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage rec {
      pname = "flow";
      version = "0-unstable-2025-06-22";

      src = pkgs.fetchFromGitHub {
        owner = "make-42";
        repo = "flow";
        rev = "c0fb308ff664be25f55e385c142b6d9c89ed4156";
        hash = "sha256-G/U5coANc/cNNZF6ru7EfYvOFj+47PljBa7w2D6hI+g=";
      };

      cargoHash = "sha256-+A5JK0SAhLf1g4wwPNH5j2wqZWXHqU9kwE6ddtG0q7k=";

      meta = {
        description = "Easy inline network speed stats";
        homepage = "https://github.com/make-42/flow";
        license = pkgs.lib.licenses.mit;
        maintainers = with pkgs.lib.maintainers; [];
        mainProgram = "flow";
      };
    };

    defaultPackage.${system} = self.packages.${system}.default;
  };
}
