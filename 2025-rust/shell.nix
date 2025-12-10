let
  rust_overlay = import (builtins.fetchTarball https://github.com/oxalica/rust-overlay/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "rust_overlay_shell";
    buildInputs = [ nix-ld pkg-config llvmPackages_19.clang-unwrapped z3 ];
    shellHook = ''
      export LIBCLANG_PATH="/run/current-system/sw/share/nix-ld/lib"
      export BINDGEN_EXTRA_CLANG_ARGS="-I${pkgs.llvmPackages_19.clang-unwrapped.lib}/lib/clang/19/include -I${pkgs.z3.dev}/include"
    '';
  }
