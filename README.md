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

This crate requires `libblkid` version `2.39.2` or later.

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rsblkid = { version = "0.2.1", features = ["v2_39"] }
```

Then install the system packages below before running `cargo build`:

- `util-linux`: to generate Rust bindings from `libblkid`'s header files.
- `libclang`: to satisfy the [dependency][4] of [`bindgen`][5] on `libclang`.
- `pkg-config`: to detect system libraries.

Read the [installation instructions](#install-required-dependencies) below to
install the required dependencies on your system.

- [Documentation (docs.rs)][2]

## Example

Extract device metadata about `/dev/vda`.

```rust
use rsblkid::probe::{Probe, ScanResult};

fn main() -> rsblkid::Result<()> {
   let mut probe = Probe::builder()
       .scan_device("/dev/vda")
       // Superblocks scanning is active by default, setting this option to
       // `true` here is redundant.
       .scan_device_superblocks(true)
       // Activate partition search functions.
       .scan_device_partitions(true)
       // Search for partition entries ONLY in DOS or GPT partition tables
       .scan_partitions_for_partition_tables(Filter::In,
           vec![
               PartitionTableType::DOS,
               PartitionTableType::GPT,
           ])
       // Activate topology search functions.
       .scan_device_topology(true)
       .build()?;

    match probe.find_device_properties() {
       ScanResult::FoundProperties => {
         // Print collected file system properties
         for property in probe.iter_device_properties() {
             println!("{property}")
         }

         println!();

         // Print metadata about partition table entries
         // Header
         println!("Partition table");
         println!("{} {:>10} {:>10}  {:>10}\n----", "number", "start", "size", "part_type");

         for partition in probe.iter_partitions() {
             let number = partition.number();
             let start = partition.location_in_sectors();
             let size = partition.size_in_sectors();
             let part_type = partition.partition_type();

             // Row
             println!("#{}: {:>10} {:>10}  0x{:x}", number, start, size, part_type)
         }

         println!();

         // Print metadata about device topology
         let topology = probe.topology()?;

         let alignment_offset = topology.alignment_offset_in_bytes();
         let dax_support = if topology.supports_dax() { "yes" } else { "no" };
         let minimum_io_size = topology.minimum_io_size();
         let optimal_io_size = topology.optimal_io_size();
         let logical_sector_size = topology.logical_sector_size();
         let physical_sector_size = topology.physical_sector_size();


         println!("Alignment offset (bytes): {}", alignment_offset);
         println!("Direct Access support (DAX): {}", dax_support);
         println!("Minimum I/O size (bytes): {}", minimum_io_size);
         println!("Optimal I/O size (bytes): {}", optimal_io_size);
         println!("Logical sector size (bytes): {}", logical_sector_size);
         println!("Physical sector size (bytes): {}", physical_sector_size);
       }
       _ => eprintln!("could not find device properties"),
    }

    // Example output
    //
    // LABEL="nixos"
    // UUID="ac4f36bf-191b-4fb0-b808-6d7fc9fc88be"
    // BLOCK_SIZE="1024"
    // TYPE="ext4"
    //
    // Partition table
    // number    start      size  part_type
    // ----
    // #1:          34      2014        0x0
    // #2:        2048      2048        0x0
    // #3:        4096      2048        0x0
    // #4:        6144      2048        0x0
    // #5:        8192      2048        0x0
    //
    // Alignment offset (bytes): 0
    // Direct Access support (DAX): no
    // Minimum I/O size (bytes): 512
    // Optimal I/O size (bytes): 0
    // Logical sector size (bytes): 512
    // Physical sector size (bytes): 512

    Ok(())
}
```

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
