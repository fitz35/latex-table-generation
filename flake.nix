{
  description = "cve-detection";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        

        devShell = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            clang
            llvmPackages.bintools
            rustup

            wasm-pack

            # webpack for dev server
            nodePackages.webpack
            nodePackages.webpack-cli
            nodePackages.webpack-dev-server
            nodePackages.copy-webpack-plugin

            nodejs
            # plotter
            pkg-config 
            fontconfig
            freetype
            openssl.dev

            
          ];

          RUSTC_VERSION = pkgs.lib.readFile ./rust-toolchain;
          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];

          postShellHook = ''
            unset SOURCE_DATE_EPOCH
            export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
            export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
            export RUSTFLAGS='--cfg=web_sys_unstable_apis'
          '';

          RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
            # add libraries here
            
          ]);

          BINDGEN_EXTRA_CLANG_ARGS = 
            (builtins.map (a: ''-I"${a}/include"'') [
              pkgs.pkg-config 
              pkgs.openssl.dev
              pkgs.glibc.dev 
              pkgs.fontconfig.dev
              pkgs.freetype.dev
            ])
            ++ [
              ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
              ''-I"${pkgs.glib.dev}/include/glib-2.0"''
              ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
            ];
        };
      }
    );
}
