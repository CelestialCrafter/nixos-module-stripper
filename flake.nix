{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    with nixpkgs;
    let
      nixos-module-stripper =
        pkgs:
        pkgs.rustPlatform.buildRustPackage (finalAttrs: {
          pname = "nixos-module-stripper";
          version = "0.1.0";

          src = lib.cleanSource ./.;

          cargoHash = "sha256-jvF63U+qTIt+I4LC89kOcVuDe2exgyV8IafmC0WwCWQ=";

          meta = {
            description = "strips nixos modules of their `config` attribute";
            homepage = "https://github.com/CelestialCrafter/nixos-module-stripper";
            license = lib.licenses.mpl20;
            maintainers = [ "CelestialCrafter" ];
          };
        });
    in
    {
      packages = lib.genAttrs [ "aarch64-linux" "x86_64-linux" ] (system: {
        default = nixos-module-stripper legacyPackages.${system};
      });

      devShells.x86_64-linux.default =
        with legacyPackages.x86_64-linux;
        mkShell {
          packages = [
            cargo
            rustc
            clang
          ];
        };
    };
}
