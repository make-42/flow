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
        rev = "d169333e5768c4dd4dd5ea8c0922c0c702c97d93";
        hash = "sha256-EdD1WsXHkfce/c2J+qBmMYlEyPhaH5W+yMWedJfk4Ts=";
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
