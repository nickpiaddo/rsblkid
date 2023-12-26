# rsblkid

----

[![Crates.io Version](https://img.shields.io/crates/v/rsblkid?labelColor=%23222222&color=%23fdb42f)][1]
[![docs.rs](https://img.shields.io/docsrs/rsblkid?labelColor=%23222222&color=%2322a884)][2]
![Crates.io MSRV](https://img.shields.io/crates/msrv/rsblkid?labelColor=%23222222&color=%239c179e)
![Crates.io License](https://img.shields.io/crates/l/rsblkid?labelColor=%23222222&color=%230d0887)

The `rsblkid` library is a safe Rust wrapper around [`util-linux/libblkid`][3].

`rsblkid` can identify disks (block devices), the file systems they use to
store content, as well as extract additional information such as:

- File system labels,
- Volume names,
- Unique identifiers,
- Serial numbers,
- Device sizes,
- Minimum and optimal I/O sizes,
- etc.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rsblkid = "0.1.0"
```

Then install the system packages below before running `cargo build`:

- `util-linux`: to generate Rust bindings from `libblkid`'s header files.
- `libclang`: to satisfy the [dependency][4] of [`bindgen`][5] on `libclang`.
- `pkg-config`: to detect system libraries.

This crate requires `libblkid` version `2.39.2` or later.

Read the [installation instructions](#install-required-dependencies) below to
install the required dependencies on your system.

- [Documentation (docs.rs)][2]

## Example

## Install required dependencies

### Alpine Linux

As `root`, issue the following command:

```console
apk add util-linux-dev clang-libclang pkgconfig
```

### NixOS

Install the packages in a temporary environment with:

```console
nix-shell -p util-linux.dev libclang.lib pkg-config
```

or permanently with:

```console
nix-env -iA nixos.util-linux.dev nixos.libclang.lib nixos.pkg-config
```

## License

This project is licensed under either of:

- [Apache License, Version 2.0][6]
- [MIT License][7]

Files in the [third-party/][8] and [web-snapshots/][9] directories are subject
to their own licenses and/or copyrights.

SPDX-License-Identifier: Apache-2.0 OR MIT

Copyright (c) 2023 Nick Piaddo

[1]: https://crates.io/crates/rsblkid
[2]: https://docs.rs/rsblkid
[3]: https://github.com/util-linux/util-linux/tree/master
[4]: https://rust-lang.github.io/rust-bindgen/requirements.html#clang
[5]: https://crates.io/crates/bindgen
[6]: https://www.apache.org/licenses/LICENSE-2.0
[7]: https://opensource.org/licenses/MIT
[8]: ./third-party/
[9]: ./web-snapshots/
