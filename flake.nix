{
  description = "A safe-Rust wrapper around the `util-linux/libblkid` C library";

  inputs = {
    # Nixpkgs / NixOS version to use.
    nixpkgs.url = "nixpkgs/nixos-23.11";

    # Set of functions to make flake nix packages simpler to set up without
    # external dependencies.
    utils.url = "github:numtide/flake-utils";

    # Nix library for building Rust projects
    naersk.url = "github:nix-community/naersk/master";

    # Backward compatibility for people without flakes enabled.
    # https://github.com/edolstra/flake-compat
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
  };

  outputs = { self, nixpkgs, utils, naersk, flake-compat }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        packages.default = naersk-lib.buildPackage ./.;

        # Development environment
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            # Diagrams
            d2

            # Markdown
            glow
            pandoc
            lynx
            w3m

            # Rust
            cargo
            cargo-audit
            # Use
            # `nix shell github:oxalica/rust-overlay#rust-nightly`
            # to have a temporary shell to use `cargo expand --lib  | bat -p -l rust` to see TypeBuilder imlementation
            cargo-expand
            cargo-flamegraph
            cargo-modules
            cargo-nextest
            cargo-rr
            cargo-tarpaulin
            cargo-vet
            cargo-valgrind
            cargo-workspaces
            lldb
            pkg-config
            rustc
            rust-analyzer
            rustfmt
            rustPackages.clippy
            valgrind

            # QEMU
            #qemu
            #OVMF

            # microvm.nix
            rlwrap
            socat

            # For code linting and formatting
            nodejs_20
            marksman
            pre-commit
            ruby
            shellcheck
            shfmt

            # Required by `libblkid-sys`
            clang
            libclang.lib
            util-linux.dev
          ];

          # Rust source path
          RUST_SRC_PATH = rustPlatform.rustLibSrc;

          # Required by `rsblkid-sys`
          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.libclang ];

          # Inspired by: "C header includes in NixOS"
          # https://discourse.nixos.org/t/c-header-includes-in-nixos/17410
          # Solves the root cause of error messages emitted when trying to
          # compile rsblkid-sys from inside a VM.
          # --- stderr
          # src/wrapper.h:1:10: fatal error: 'blkid/blkid.h' file not found
          C_INCLUDE_PATH="${util-linux.dev}/include";
        };
      });
}
