{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable-small";
    systems.url = "github:nix-systems/default";
  };
  outputs = inputs: let
    perSystem = inputs.nixpkgs.lib.genAttrs (import inputs.systems);
    systemPkgs = perSystem(system: import inputs.nixpkgs {
      inherit system;
    });
  in {
    devShells = perSystem(system: let
      pkgs = systemPkgs.${system};
    in {
      default = pkgs.mkShellNoCC {
        # Environment variables
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        RUSTFLAGS = "-lX11";

        # Build toolchain and dependencies/libraries
        nativeBuildInputs = with pkgs; [
          cargo
          gcc
          cmake
          pkg-config
        ];
        buildInputs = with pkgs; [
          glfw3
          wayland

          libX11
        ];
      };
    });
  };
}