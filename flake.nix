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
        rev = "2fc983e772acdcc40b4e8e75d276568b69a09792";
        hash = "sha256-D3BobeuHgaSfGijuuHTJItMHeO/AydBFv0QzLfQrIdo=";
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
