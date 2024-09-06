{
  lib,
  config,
  dream2nix,
  ...
}: {
  imports = [
    dream2nix.modules.dream2nix.rust-cargo-lock
    dream2nix.modules.dream2nix.rust-crane
  ];

  deps = {nixpkgs, ...}: {
    inherit (nixpkgs) pkg-config openssl;
  };

  name = lib.mkForce "mistralai-client";
  version = lib.mkForce "0.14.0";

  env = {
    "PKG_CONFIG_PATH" = "${config.deps.openssl.dev}/lib/pkgconfig";
  };

  # options defined on top-level will be applied to the main derivation (the derivation that is exposed)
  mkDerivation = {
    # define the source root that contains the package we want to build.
    propagatedBuildInputs = [
      config.deps.openssl.dev
    ];
    nativeBuildInputs = [
      config.deps.pkg-config
      config.deps.openssl
    ];
    src = ./.;
  };

  rust-crane = {
    buildProfile = "dev";
    buildFlags = ["--verbose"];
    runTests = false;
    depsDrv = {
  mkDerivation = {
    propagatedBuildInputs = [
      config.deps.openssl.dev
    ];
    nativeBuildInputs = [
      config.deps.pkg-config
    ];
  };
      # options defined here will be applied to the dependencies derivation
    };
  };
}
