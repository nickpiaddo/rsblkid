// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Table of Contents
//! 1. [Description](#description)
//! 2. [API structure](#api-structure)
//! 3. [From `libblkid` to `rsblkid`](#from-libblkid-to-rsblkid)
//!     1. [High-Level functions](#high-level-functions)
//!         1. [Tag and spec evaluation](#tag-and-spec-evaluation)
//!         2. [`Cache` basic routines](#cache-basic-routines)
//!         3. [Search and iterate over devices in the cache](#search-and-iterate-over-devices-in-the-cache)
//!     2. [Low-Level functions](#low-level-functions)
//!         1. [Library initialization](#library-initialization)
//!         2. [Low-level probing](#low-level-probing)
//!         3. [Low-level tags](#low-level-tags)
//!         4. [Superblocks probing](#superblocks-probing)
//!         5. [Partitions probing](#partitions-probing)
//!         6. [Topology information](#topology-information)
//!     3. [Common utils](#common-utils)
//!         1. [Encoding utils](#encoding-utils)
//!         2. [Miscellaneous utils](#miscellaneous-utils)
//!
//! ## Description
//!
//! The `rsblkid` library is a safe Rust wrapper around `util-linux/libblkid`.
//!
//! The `libblkid` library helps identify disks (block devices), the file systems they use to
//! store content, as well as extracting additional information such as:
//! - File system labels,
//! - Volume names,
//! - Unique identifiers,
//! - Serial numbers,
//! - etc.
//!
//! `rsblkid` presents the data it gathers as key/value pairs (tags), where the keys can be for
//! example  a device's `LABEL`, `UUID`, file system `TYPE`, etc. (see
//! [`FsProperty`](crate::probe::FsProperty) for the list of keys supported by `rsblkid`).
//!
//! ## API structure
//!
//! `rsblkid`'s API is roughly divided into two parts, a [high-level API](crate::cache) that keeps information
//! about block devices in a cache file, and a [low-level API](crate::probe) that offers more fine grained methods
//! to extract data about file systems, device partitions, and disk topology.
//!
//! Provided it has permission to read raw block devices, the high-level part of the library
//! checks that device information is always up-to-date before returning it to the user. The cache
//! file allows unprivileged users, i.e. anyone other than `root` or a member of the `disk` user
//! group, to locate devices by label or id.
//!
//! ```ignore
//! # use pretty_assertions::assert_eq;
//! use std::path::PathBuf;
//! use rsblkid::core::device::Tag;
//! use rsblkid::cache::Cache;
//!
//! fn main() -> rsblkid::Result<()> {
//!     let mut cache = Cache::builder()
//!         .discard_changes_on_drop()
//!         .build()?;
//!
//!     cache.probe_all_devices()?;
//!
//!     // Find the absolute path to the device with the UUID.
//!     let uuid: Tag = r#"UUID="ac4f36bf-191b-4fb0-b808-6d7fc9fc88be""#.parse()?;
//!     let actual = cache.find_canonical_device_name_from_tag(&uuid);
//!     let device_name = PathBuf::from("/dev/vda");
//!     let expected = Some(device_name);
//!
//!     assert_eq!(actual, expected);
//!
//!     Ok(())
//! }
//! ```
//! To determine the values of the `LABEL` or `UUID` tags of a block device, the high-level API supports two methods:
//!  - extracting data directly by scanning a block device,
//!  - or reading information from [udev](https://wiki.archlinux.org/title/Udev)'s
//! `/dev/disk/by-*` symlinks (method used by default).
//!
//! The [low-level API](crate::probe), on the other hand, always scans a block device directly. It offers more
//! fine-grained control over the data collected.
//!
//! ```ignore
//! use rsblkid::probe::{Probe, ScanResult};
//!
//! fn main() -> rsblkid::Result<()> {
//!     let mut probe = Probe::builder()
//!         .scan_device("/dev/vda")
//!         // Superblocks scanning is active by default, setting this option to `true` here
//!         // is redundant.
//!         .scan_device_superblocks(true)
//!         .build()?;
//!
//!     match probe.find_device_properties() {
//!         ScanResult::FoundProperties => {
//!             // Print collected file system properties
//!             for property in probe.iter_device_properties() {
//!                 println!("{property}")
//!             }
//!         }
//!         _ => eprintln!("could not find device properties"),
//!     }
//!
//!     // Example output
//!     //
//!     // LABEL="nixos"
//!     // UUID="ac4f36bf-191b-4fb0-b808-6d7fc9fc88be"
//!     // BLOCK_SIZE="1024"
//!     // TYPE="ext4"
//!
//!     Ok(())
//! }
//! ```
//!
//! ## From `libblkid` to `rsblkid`
//!
//! This section maps `libblkid` functions to `rsblkid` methods. It follows the same layout as
//! `libblkid`'s documentation. You can use it as a reference to ease the transition from one API
//! to the other.
//!
//! ### High-Level functions
//! #### Tag and spec evaluation
//!
//! | `libblkid`                            | `rsblkid`                                                                                                                                                                                                                |
//! | ------------------------------------- | ---------                                                                                                                                                                                                                |
//! | [`blkid_evaluate_tag`][1]             | [`Cache::find_device_name_from_tag`](crate::cache::Cache::find_device_name_from_tag)                                                                                                                                     |
//! | [`blkid_evaluate_spec`][2]            | [`Cache::find_canonical_device_name_from_tag`](crate::cache::Cache::find_canonical_device_name_from_tag) <br> [`Cache::find_canonical_device_name_from_path`](crate::cache::Cache::find_canonical_device_name_from_path) |
//!
//! [1]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Tags-and-Spec-evaluation.html#blkid-evaluate-tag
//! [2]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Tags-and-Spec-evaluation.html#blkid-evaluate-spec
//!
//! #### `Cache` basic routines
//!
//! | `libblkid`                                                | `rsblkid`                                                                                |
//! | --------------------------------------------------------- | ---------                                                                                |
//! | [`blkid_gc_cache`][3]                                     | [`Cache::garbage_collect`](crate::cache::Cache::garbage_collect)                         |
//! | [`blkid_get_cache`][4]                                    | [`Cache::builder`](crate::cache::Cache::builder)                                         |
//! | [`blkid_put_cache`][5]                                    | [`Cache`](crate::cache::Cache) is automatically deallocated when it goes out of scope.   |
//! | [`blkid_probe_all`][6]                                    | [`Cache::probe_all_devices`](crate::cache::Cache::probe_all_devices)                     |
//! | [`blkid_probe_all_removable`][7]                          | [`Cache::probe_all_removable_devices`](crate::cache::Cache::probe_all_removable_devices) |
//! | [`blkid_probe_all_new`][8]                                | [`Cache::probe_all_new_devices`](crate::cache::Cache::probe_all_new_devices)             |
//! | [`blkid_verify`][9]                                       | [`Cache::refresh_device_data`](crate::cache::Cache::refresh_device_data)                 |
//!
//! [3]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Cache.html#blkid-gc-cache
//! [4]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Cache.html#blkid-get-cache
//! [5]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Cache.html#blkid-put-cache
//! [6]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Cache.html#blkid-probe-all
//! [7]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Cache.html#blkid-probe-all-removable
//! [8]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Cache.html#blkid-probe-all-new
//! [9]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Cache.html#blkid-verify
//!
//! #### Search and iterate over devices in the cache
//!
//! | `libblkid`                      | `rsblkid`                                                                                                                                                                                                                                                                                                                          |
//! | ------------                    | ---------                                                                                                                                                                                                                                                                                                                          |
//! | [`blkid_dev_devname`][10]       | [`Device::name`](crate::cache::Device::name)                                                                                                                                                                                                                                                                                       |
//! | [`blkid_dev_has_tag`][11]       | [`Device::has_tag`](crate::cache::Device::has_tag) <br> [`Device::has_tag_named`](crate::cache::Device::has_tag_named)                                                                                                                                                                                                             |
//! | [`blkid_dev_iterate_begin`][12] | [`Cache::iter`](crate::cache::Cache::iter)                                                                                                                                                                                                                                                                                         |
//! | [`blkid_dev_iterate_end`][13]   | [`EntryIter`](crate::cache::EntryIter) is automatically deallocated when it goes out of scope.                                                                                                                                                                                                                                     |
//! | [`blkid_dev_next`][14]          | [`EntryIter::next`](crate::cache::EntryIter::next)                                                                                                                                                                                                                                                                                 |
//! | [`blkid_dev_set_search`][15]    | Not implemented yet.                                                                                                                                                                                                                                                                                                               |
//! | [`blkid_find_dev_with_tag`][16] | [`Cache::find_device_with_tag`](crate::cache::Cache::find_device_with_tag)                                                                                                                                                                                                                                                         |
//! | [`blkid_get_dev`][17]           | [`Cache::add_new_entry`](crate::cache::Cache::add_new_entry) <br> [`Cache::find_device_by_name`](crate::cache::Cache::find_device_by_name) <br> [`Cache::lookup_device_by_name`](crate::cache::Cache::lookup_device_by_name) <br> [`Cache::lookup_refreshed_device_by_name`](crate::cache::Cache::lookup_refreshed_device_by_name) |
//! | [`blkid_get_devname`][18]       | Not implemented. Use [`Cache::find_device_with_tag`](crate::cache::Cache::find_device_with_tag) instead.                                                                                                                                                                                                                           |
//! | [`blkid_get_tag_value`][19]     | [`Cache::tag_value_from_device`](crate::cache::Cache::tag_value_from_device)                                                                                                                                                                                                                                                       |
//! | [`blkid_tag_iterate_begin`][20] | [`Device::iter`](crate::cache::Device::iter)                                                                                                                                                                                                                                                                                       |
//! | [`blkid_tag_iterate_end`][21]   | [`TagIter`](crate::cache::TagIter) is automatically deallocated when it goes out of scope.                                                                                                                                                                                                                                         |
//! | [`blkid_tag_next`][22]          | [`TagIter::next`](crate::cache::TagIter::next)                                                                                                                                                                                                                                                                                     |
//!
//! [10]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Search-and-iterate.html#blkid-dev-devname
//! [11]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Search-and-iterate.html#blkid-dev-has-tag
//! [12]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Search-and-iterate.html#blkid-dev-iterate-begin
//! [13]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Search-and-iterate.html#blkid-dev-iterate-end
//! [14]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Search-and-iterate.html#blkid-dev-next
//! [15]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Search-and-iterate.html#blkid-dev-set-search
//! [16]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Search-and-iterate.html#blkid-find-dev-with-tag
//! [17]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Search-and-iterate.html#blkid-get-dev
//! [18]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Search-and-iterate.html#blkid-get-devname
//! [19]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Search-and-iterate.html#blkid-get-tag-value
//! [20]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Search-and-iterate.html#blkid-tag-iterate-begin
//! [21]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Search-and-iterate.html#blkid-tag-iterate-end
//! [22]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Search-and-iterate.html#blkid-tag-next
//!
//! ### Low-Level functions
//! #### Library initialization
//! | `libblkid`               | `rsblkid`                                                                                                      |
//! | ------------------       | ---------                                                                                                      |
//! | [`blkid_init_debug`][23] | [`init_default_debug`](crate::debug::init_default_debug)<br>[`init_full_debug`](crate::debug::init_full_debug) |
//!
//! [23]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Library-initialization.html#blkid-init-debug
//!
//! #### Low-level probing
//!
//! | `libblkid`                                | `rsblkid`                                                                                        |
//! | ----------------------------------------- | ---------                                                                                        |
//! | [`blkid_free_probe`][24]                  | [`Probe`](crate::probe::Probe) is automatically deallocated when it goes out of scope.           |
//! | [`blkid_new_probe`][25]                   | [`Probe::builder`](crate::probe::Probe::builder)                                                 |
//! | [`blkid_new_probe_from_filename`][26]     | [`ProbeBuilder::scan_device`](crate::probe::ProbeBuilder::scan_device)                           |
//! | [`blkid_probe_get_devno`][27]             | [`Probe::device_number`](crate::probe::Probe::device_number)                                     |
//! | [`blkid_probe_get_fd`][28]                | [`Probe::device_file`](crate::probe::Probe::device_file)                                         |
//! | [`blkid_probe_get_offset`][29]            | [`Probe::scanned_device_segment_location`](crate::probe::Probe::scanned_device_segment_location) |
//! | [`blkid_probe_get_sectors`][30]           | [`Probe::device_size_in_sectors`](crate::probe::Probe::device_size_in_sectors)                   |
//! | [`blkid_probe_get_sectorsize`][31]        | [`Probe::device_logical_sector_size`](crate::probe::Probe::device_logical_sector_size)           |
//! | [`blkid_probe_get_size`][32]              | [`Probe::scanned_device_segment_size`](crate::probe::Probe::scanned_device_segment_size)         |
//! | [`blkid_probe_get_wholedisk_devno`][33]   | [`Probe::device_whole_disk_number`](crate::probe::Probe::device_whole_disk_number)               |
//! | [`blkid_probe_hide_range`][34]            | [`Probe::device_skip_bytes`](crate::probe::Probe::device_skip_bytes)                             |
//! | [`blkid_probe_is_wholedisk`][35]          | [`Probe::is_device_whole_disk`](crate::probe::Probe::is_device_whole_disk)                       |
//! | [`blkid_probe_reset_buffers`][36]         | [`Probe::empty_buffers`](crate::probe::Probe::empty_buffers)                                     |
//! | [`blkid_probe_reset_hints`][37]           | [`Probe::discard_hints`](crate::probe::Probe::discard_hints)                                     |
//! | [`blkid_probe_set_device`][38]            | Not implemented.                                                                                 |
//! | [`blkid_probe_set_hint`][39]              | [`Probe::set_hint`](crate::probe::Probe::set_hint)                                               |
//! | [`blkid_probe_set_sectorsize`][40]        | [`ProbeBuilder::bytes_per_sector`](crate::probe::ProbeBuilder::bytes_per_sector)                 |
//! | [`blkid_probe_step_back`][41]             | [`Probe::backtrack`](crate::probe::Probe::backtrack)                                             |
//! | [`blkid_reset_probe`][42]                 | [`Probe::reset`](crate::probe::Probe::reset)                                                     |
//!
//! [24]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-free-probe
//! [25]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-new-probe
//! [26]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-new-probe-from-filename
//! [27]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-get-devno
//! [28]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-get-fd
//! [29]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-get-offset
//! [30]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-get-sectors
//! [31]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-get-sectorsize
//! [32]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-get-size
//! [33]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-get-wholedisk-devno
//! [34]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-hide-range
//! [35]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-is-wholedisk
//! [36]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-reset-buffers
//! [37]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-reset-hints
//! [38]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-set-device
//! [39]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-set-hint
//! [40]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-set-sectorsize
//! [41]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-probe-step-back
//! [42]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-probing.html#blkid-reset-probe
//!
//! #### Low-level tags
//!
//! | `libblkid`                       | `rsblkid`                                                                                                                                                                                    |
//! | ------------------               | ---------                                                                                                                                                                                    |
//! | [`blkid_do_fullprobe`][43]       | [`Probe::find_device_properties`](crate::probe::Probe::find_device_properties)                                                                                                               |
//! | [`blkid_do_wipe`][44]            | [`Probe::delete_properties_from_device`](crate::probe::Probe::delete_properties_from_device)<br>[`Probe::delete_properties_from_memory`](crate::probe::Probe::delete_properties_from_memory) |
//! | [`blkid_do_probe`][45]           | [`Probe::run_scan`](crate::probe::Probe::run_scan)                                                                                                                                           |
//! | [`blkid_do_safeprobe`][46]       | [`Probe::find_all_device_properties`](crate::probe::Probe::find_all_device_properties)                                                                                                       |
//! | [`blkid_probe_get_value`][47]    | [`Probe::nth_device_property`](crate::probe::Probe::nth_device_property)                                                                                                                     |
//! | [`blkid_probe_has_value`][48]    | [`Probe::device_property_has_value`](crate::probe::Probe::device_property_has_value)                                                                                                         |
//! | [`blkid_probe_lookup_value`][49] | [`Probe::lookup_device_property_value`](crate::probe::Probe::lookup_device_property_value)                                                                                                   |
//! | [`blkid_probe_numof_values`][50] | [`Probe::count_device_properties`](crate::probe::Probe::count_device_properties)                                                                                                             |
//!
//! [43]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-tags.html#blkid-do-fullprobe
//! [44]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-tags.html#blkid-do-wipe
//! [45]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-tags.html#blkid-do-probe
//! [46]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-tags.html#blkid-do-safeprobe
//! [47]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-tags.html#blkid-probe-get-value
//! [48]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-tags.html#blkid-probe-has-value
//! [49]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-tags.html#blkid-probe-lookup-value
//! [50]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Low-level-tags.html#blkid-probe-numof-values
//!
//! #### Superblocks probing
//!
//! | `libblkid`                                    | `rsblkid`                                                                                                                                                                                                                  |
//! | ------------------                            | ---------                                                                                                                                                                                                                  |
//! | [`blkid_probe_enable_superblocks`][51]        | [`ProbeBuilder::scan_device_superblocks`](crate::probe::ProbeBuilder::scan_device_superblocks)                                                                                                                             |
//! | [`blkid_known_fstype`][52]                    | Not implemented. [`FileSystem`](crate::core::partition::FileSystem) lists all supported file systems.                                                                                                                      |
//! | [`blkid_superblocks_get_name`][53]            | [`Probe::iter_supported_file_systems`](crate::probe::Probe::iter_supported_file_systems)                                                                                                                                   |
//! | [`blkid_probe_filter_superblocks_type`][54]   | [`ProbeBuilder::scan_superblocks_for_file_systems`](crate::probe::ProbeBuilder::scan_superblocks_for_file_systems)<br>[`Probe::scan_superblocks_for_file_systems`](crate::probe::Probe::scan_superblocks_for_file_systems) |
//! | [`blkid_probe_filter_superblocks_usage`][55]  | [`ProbeBuilder::scan_superblocks_with_usage_flags`](crate::probe::ProbeBuilder::scan_superblocks_with_usage_flags)<br>[`Probe::scan_superblocks_with_usage_flags`](crate::probe::Probe::scan_superblocks_with_usage_flags) |
//! | [`blkid_probe_invert_superblocks_filter`][56] | [`Probe::invert_superblocks_scanning_filter`](crate::probe::Probe::invert_superblocks_scanning_filter)                                                                                                                     |
//! | [`blkid_probe_reset_superblocks_filter`][57]  | [`Probe::reset_superblocks_scanning_filter`](crate::probe::Probe::reset_superblocks_scanning_filter)                                                                                                                       |
//! | [`blkid_probe_set_superblocks_flags`][58]     | [`Probe::collect_fs_properties`](crate::probe::Probe::collect_fs_properties)                                                                                                                                               |
//! | [`blkid_probe_reset_filter`][59]              | Deprecated.                                                                                                                                                                                                                |
//! | [`blkid_probe_filter_types`][60]              | Deprecated.                                                                                                                                                                                                                |
//! | [`blkid_probe_filter_usage`][61]              | Deprecated.                                                                                                                                                                                                                |
//! | [`blkid_probe_invert_filter`][62]             | Deprecated.                                                                                                                                                                                                                |
//! | [`blkid_probe_set_request`][63]               | Deprecated.                                                                                                                                                                                                                |
//!
//!
//! [51]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Superblocks-probing.html#blkid-probe-enable-superblocks
//! [52]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Superblocks-probing.html#blkid-known-fstype
//! [53]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Superblocks-probing.html#blkid-superblocks-get-name
//! [54]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Superblocks-probing.html#blkid-probe-filter-superblocks-type
//! [55]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Superblocks-probing.html#blkid-probe-filter-superblocks-usage
//! [56]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Superblocks-probing.html#blkid-probe-invert-superblocks-filter
//! [57]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Superblocks-probing.html#blkid-probe-reset-superblocks-filter
//! [58]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Superblocks-probing.html#blkid-probe-set-superblocks-flags
//! [59]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Superblocks-probing.html#blkid-probe-reset-filter
//! [60]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Superblocks-probing.html#blkid-probe-filter-types
//! [61]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Superblocks-probing.html#blkid-probe-filter-usage
//! [62]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Superblocks-probing.html#blkid-probe-invert-filter
//! [63]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Superblocks-probing.html#blkid-probe-set-request

//! #### Partitions probing
//!
//! | `libblkid`                                     | `rsblkid`                                                                                    |
//! | ------------------                             | ---------                                                                                    |
//! | [`blkid_probe_enable_partitions`][64]          |                                                                                              |
//! | [`blkid_probe_set_partitions_flags`][65]       |                                                                                              |
//! | [`blkid_probe_filter_partitions_type`][66]     |                                                                                              |
//! | [`blkid_probe_invert_partitions_filter`][67]   |                                                                                              |
//! | [`blkid_probe_reset_partitions_filter`][68]    |                                                                                              |
//! | [`blkid_known_pttype`][69]                     |                                                                                              |
//! | [`blkid_partitions_get_name`][70]              |                                                                                              |
//! | [`blkid_partition_get_name`][71]               | [`Partition::name`](crate::probe::Partition::name)                                           |
//! | [`blkid_partition_get_flags`][72]              | [`Partition::flags`](crate::probe::Partition::flags)                                         |
//! | [`blkid_partition_get_partno`][73]             | [`Partition::number`](crate::probe::Partition::number)                                       |
//! | [`blkid_partition_get_size`][74]               | [`Partition::size_in_sectors`](crate::probe::Partition::size_in_sectors)                     |
//! | [`blkid_partition_get_start`][75]              | [`Partition::location_in_sectors`](crate::probe::Partition::location_in_sectors)             |
//! | [`blkid_partition_get_table`][76]              | [`Partition::partition_table`](crate::probe::Partition::partition_table)                     |
//! | [`blkid_partition_get_type`][77]               | [`Partition::partition_type`](crate::probe::Partition::partition_type)                       |
//! | [`blkid_partition_get_type_string`][78]        | [`Partition::partition_type_string`](crate::probe::Partition::partition_type_string)         |
//! | [`blkid_partition_get_uuid`][79]               | [`Partition::uuid`](crate::probe::Partition::uuid)                                           |
//! | [`blkid_partition_is_extended`][80]            | [`Partition::is_extended`](crate::probe::Partition::is_extended)                             |
//! | [`blkid_partition_is_logical`][81]             | [`Partition::is_logical`](crate::probe::Partition::is_logical)                               |
//! | [`blkid_partition_is_primary`][82]             | [`Partition::is_primary`](crate::probe::Partition::is_primary)                               |
//! | [`blkid_partlist_get_partition`][83]           |                                                                                              |
//! | [`blkid_partlist_get_partition_by_partno`][84] |                                                                                              |
//! | [`blkid_partlist_numof_partitions`][85]        |                                                                                              |
//! | [`blkid_partlist_devno_to_partition`][86]      |                                                                                              |
//! | [`blkid_partlist_get_table`][87]               |                                                                                              |
//! | [`blkid_parttable_get_id`][88]                 | [`PartitionTable::id`](crate::probe::PartitionTable::id)                                     |
//! | [`blkid_parttable_get_offset`][89]             | [`PartitionTable::location_in_bytes`](crate::probe::PartitionTable::location_in_bytes)       |
//! | [`blkid_parttable_get_parent`][90]             | [`PartitionTable::parent`](crate::probe::PartitionTable::parent)                             |
//! | [`blkid_parttable_get_type`][91]               | [`PartitionTable::partition_table_type`](crate::probe::PartitionTable::partition_table_type) |
//! | [`blkid_probe_get_partitions`][92]             |                                                                                              |
//!
//!
//! [64]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-probe-enable-partitions
//! [65]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-probe-set-partitions-flags
//! [66]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-probe-filter-partitions-type
//! [67]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-probe-invert-partitions-filter
//! [68]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-probe-reset-partitions-filter
//! [69]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-known-pttype
//! [70]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partitions-get-name
//! [71]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partition-get-name
//! [72]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partition-get-flags
//! [73]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partition-get-partno
//! [74]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partition-get-size
//! [75]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partition-get-start
//! [76]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partition-get-table
//! [77]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partition-get-type
//! [78]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partition-get-type-string
//! [79]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partition-get-uuid
//! [80]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partition-is-extended
//! [81]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partition-is-logical
//! [82]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partition-is-primary
//! [83]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partlist-get-partition
//! [84]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partlist-get-partition-by-partno
//! [85]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partlist-numof-partitions
//! [86]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partlist-devno-to-partition
//! [87]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-partlist-get-table
//! [88]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-parttable-get-id
//! [89]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-parttable-get-offset
//! [90]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-parttable-get-parent
//! [91]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-parttable-get-type
//! [92]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Partitions-probing.html#blkid-probe-get-partitions
//!
//! #### Topology information
//!
//! | `libblkid`                                       | `rsblkid` |
//! | ------------------                               | --------- |
//! | [`blkid_probe_enable_topology`][93]              |           |
//! | [`blkid_probe_get_topology`][94]                 |           |
//! | [`blkid_topology_get_alignment_offset`][95]      |           |
//! | [`blkid_topology_get_dax`][96]                   |           |
//! | [`blkid_topology_get_logical_sector_size`][97]   |           |
//! | [`blkid_topology_get_minimum_io_size`][98]       |           |
//! | [`blkid_topology_get_optimal_io_size`][99]       |           |
//! | [`blkid_topology_get_physical_sector_size`][100] |           |
//!
//!
//! [93]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Topology-information.html#blkid-probe-enable-topology
//! [94]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Topology-information.html#blkid-probe-get-topology
//! [95]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Topology-information.html#blkid-topology-get-alignment-offset
//! [96]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Topology-information.html#blkid-topology-get-dax
//! [97]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Topology-information.html#blkid-topology-get-logical-sector-size
//! [98]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Topology-information.html#blkid-topology-get-minimum-io-size
//! [99]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Topology-information.html#blkid-topology-get-optimal-io-size
//! [100]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Topology-information.html#blkid-topology-get-physical-sector-size
//!
//! ### Common Utils
//! #### Encoding utils
//!
//! | `libblkid`                   | `rsblkid`                               |
//! | ------------------           | ---------                               |
//! | [`blkid_encode_string`][101] | [`core::utils::encode::encode_string`]  |
//! | [`blkid_safe_string`][102]   | [`core::utils::encode::to_safe_string`] |
//!
//! [101]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Encoding-utils.html#blkid-encode-string
//! [102]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Encoding-utils.html#blkid-safe-string
//!
//! #### Miscellaneous utils
//!
//! | `libblkid`                          | `rsblkid` |
//! | ------------------                  | --------- |
//! | [`blkid_devno_to_devname`][103]     |           |
//! | [`blkid_devno_to_wholedisk`][104]   |           |
//! | [`blkid_get_dev_size`][105]         |           |
//! | [`blkid_get_library_version`][106]  |           |
//! | [`blkid_parse_tag_string`][107]     |           |
//! | [`blkid_parse_version_string`][108] |           |
//! | [`blkid_send_uevent`][109]          |           |
//!
//!
//! [103]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Miscellaneous-utils.html#blkid-devno-to-devname
//! [104]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Miscellaneous-utils.html#blkid-devno-to-wholedisk
//! [105]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Miscellaneous-utils.html#blkid-get-dev-size
//! [106]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Miscellaneous-utils.html#blkid-get-library-version
//! [107]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Miscellaneous-utils.html#blkid-parse-tag-string
//! [108]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Miscellaneous-utils.html#blkid-parse-version-string
//! [109]: https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/libblkid-docs/libblkid-Miscellaneous-utils.html#blkid-send-uevent

pub use error::*;

pub mod cache;
pub mod core;
pub mod debug;
mod error;
pub(crate) mod ffi_utils;
pub mod probe;
