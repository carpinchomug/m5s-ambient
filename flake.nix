{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgs = forAllSystems (system: nixpkgs.legacyPackages.${system});
    in
    {
      devShells = forAllSystems (system: with pkgs.${system};
        {
          default = mkShell {
            packages = [
              rustc
              cargo
              rust-analyzer
              pkg-config
              openssl
            ];

            shellHook = ''
              set -a
              source ./.env
              set +a
            '';
          };
        });
    };
}
