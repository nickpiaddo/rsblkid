let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-23.11";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in
pkgs.testers.runNixOSTest {
  name = "rsblkid";
  nodes.vm = {config, pkgs, ...}: {
    # QEMU config
    virtualisation = {
      memorySize = 2048; #MiB
      diskSize = 8192; #MiB
      sharedDirectories = {
         rsblkid = { source = "${./.}"; target = "/repos/rsblkid"; };
      };
    };

    environment = {
      sessionVariables = {
            # Rust source path
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

            # Required by `libblkid-sys`
            LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.libclang ];

            # Required by `pkg-config` to discover the locations of `libblkid` and `libmount`
            PKG_CONFIG_PATH = "${pkgs.util-linux.dev}/lib/pkgconfig";

            # Inspired by: "C header includes in NixOS"
            # https://discourse.nixos.org/t/c-header-includes-in-nixos/17410
            # Solves the root cause of error messages emitted when trying to
            # compile rsblkid-sys from inside a VM.
            # --- stderr
            # src/wrapper.h:1:10: fatal error: 'blkid/blkid.h' file not found
            C_INCLUDE_PATH="${pkgs.util-linux.dev}/include";
      };
    };

    # For interactive builds
    nix = {
    # Install Nix Flakes
    package = pkgs.nixFlakes;
    extraOptions = ''
      experimental-features = nix-command flakes
      '';
    };

    # Root user
    users.users.root = {
      password = "";
      # Default packages.
      packages = with pkgs; [
        # For interactive builds
        bat
        git
        htop
        vim

        # For Rust
        cargo
        cargo-nextest
        cargo-tarpaulin
        rustc
        rust-analyzer
        rustfmt
        rustPackages.clippy
        clang
        libclang.lib
        pkg-config
        util-linux.dev
      ];
    };

    services.getty.autologinUser = "root";

    system.stateVersion = "23.11";
  };

  testScript = ''
    vm.wait_for_unit("multi-user.target")

    # Copy repos
    vm.succeed("rsync -a /repos/rsblkid     /root --exclude .git --exclude .direnv --exclude target --exclude result --exclude web-snapshots --exclude area51 --exclude test-microvm")

    # Run tests
    #vm.succeed("cd /root/rsblkid; cargo nextest run; cargo test --doc")
    vm.succeed("cd /root/rsblkid && cargo nextest run")
    vm.succeed("cd /root/rsblkid && cargo test --doc")
  '';
}
