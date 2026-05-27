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

        # Build toolchain
        nativeBuildInputs = with pkgs; [
          cargo
          gcc
          cmake
          pkg-config
        ];

        # Dependencies/Libraries
        buildInputs = with pkgs; [
          # Wayland
          glfw3
          wayland

          # Xorg stuff that's required for some reason
          libX11
          libxrandr
          libxinerama
          libxcursor
          libxi
        ];
      };
    });
  };
}