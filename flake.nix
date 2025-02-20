{
  description = "My flake with dream2nix packages";

  inputs = {
rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    self,
    rust-overlay,
    nixpkgs,
    ...
  }: let
    system = "x86_64-linux";
      pkgs = import nixpkgs {
          inherit system;
        overlays = [ (import rust-overlay) ];
        };
  in {
        devShells.${system}.default = pkgs.mkShell {
          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";
        "PKG_CONFIG_PATH" = "${pkgs.openssl.dev}/lib/pkgconfig";

          # Extra inputs
          buildInputs = [
          ];

          # Extra inputs can be added here
          nativeBuildInputs = with pkgs; [
            cargo
            cargo-edit
            rustfmt
            rustc
            nixpkgs-fmt
            pkg-config
          ];
        };
  };
}
