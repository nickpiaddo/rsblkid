// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Activate debug message output.
//!
//! `rsblkid` provides a facility to log debug messages through the
//! [log](https://crates.io/crates/log) lightweight logging *facade*.
//!
//! From the package's README, you need to provide a logger implementation compatible with the
//! *facade*:
//!
//! > In order to produce log output, executables have to use a logger implementation compatible with the facade.
//! > There are many available implementations to choose from, here are some options:
//! >
//! > * Simple minimal loggers:
//! >     * [`env_logger`](https://docs.rs/env_logger/*/env_logger/)
//! >     * [`colog`](https://docs.rs/colog/*/colog/)
//! >     * [`simple_logger`](https://docs.rs/simple_logger/*/simple_logger/)
//! >     * [`simplelog`](https://docs.rs/simplelog/*/simplelog/)
//! >     * [`pretty_env_logger`](https://docs.rs/pretty_env_logger/*/pretty_env_logger/)
//! >     * [`stderrlog`](https://docs.rs/stderrlog/*/stderrlog/)
//! >     * [`flexi_logger`](https://docs.rs/flexi_logger/*/flexi_logger/)
//! >     * [`call_logger`](https://docs.rs/call_logger/*/call_logger/)
//! >     * [`std-logger`](https://docs.rs/std-logger/*/std_logger/)
//! >     * [`structured-logger`](https://docs.rs/structured-logger/latest/structured_logger/)
//! > * Complex configurable frameworks:
//! >     * [`log4rs`](https://docs.rs/log4rs/*/log4rs/)
//! >     * [`logforth`](https://docs.rs/logforth/*/logforth/)
//! >     * [`fern`](https://docs.rs/fern/*/fern/)
//! > * Adaptors for other facilities:
//! >     * [`syslog`](https://docs.rs/syslog/*/syslog/)
//! >     * [`systemd-journal-logger`](https://docs.rs/systemd-journal-logger/*/systemd_journal_logger/)
//! >     * [`slog-stdlog`](https://docs.rs/slog-stdlog/*/slog_stdlog/)
//! >     * [`android_log`](https://docs.rs/android_log/*/android_log/)
//! >     * [`win_dbg_logger`](https://docs.rs/win_dbg_logger/*/win_dbg_logger/)
//! >     * [`db_logger`](https://docs.rs/db_logger/*/db_logger/)
//! >     * [`log-to-defmt`](https://docs.rs/log-to-defmt/*/log_to_defmt/)
//! >     * [`logcontrol-log`](https://docs.rs/logcontrol-log/*/logcontrol_log/)
//! > * For WebAssembly binaries:
//! >     * [`console_log`](https://docs.rs/console_log/*/console_log/)
//! > * For dynamic libraries:
//! >     * You may need to construct [an FFI-safe wrapper over `log`](https://github.com/rust-lang/log/issues/421) to initialize in your libraries.
//! > * Utilities:
//! >     * [`log_err`](https://docs.rs/log_err/*/log_err/)
//! >     * [`log-reload`](https://docs.rs/log-reload/*/log_reload/)
//! >     * [`alterable_logger`](https://docs.rs/alterable_logger/*/alterable_logger)
//! >
//! > Executables should choose a logger implementation and initialize it early in the
//! > runtime of the program. Logger implementations will typically include a
//! > function to do this. Any log messages generated before the logger is
//! > initialized will be ignored.
//! >
//! > The executable itself may use the `log` crate to log as well.
//!
//! Here is an example of debug message initialization using the
//! [`env_logger`](https://docs.rs/env_logger/*/env_logger/) crate, and `libblkid`'s own debug
//! interface.
//!
//! ```ignore
//! static INIT: std::sync::Once = std::sync::Once::new();
//!
//! fn main() {
//!    // Initialize debug output
//!    INIT.call_once(|| {
//!        // rsblkid debug messages
//!        env_logger::init();
//!        // libblkid debug messages
//!        rsblkid::debug::init_default_debug();
//!    });
//!
//!    // The rest of your program...
//!
//! }
//!
//! ```
//!
//! Assuming your executable is called `main` you can adjust the log-level of `libblkid` and/or
//! `rsblkid` by setting respectively the `LIBBLKID_DEBUG` and/or `RUST_LOG` environment variables.
//!
//! ```text
//! # libblkid debug messages only
//! # (look to the `init_default_debug` function's documentation for an exhaustive list of options)
//! $ LIBBLKID_DEBUG="lowprobe,buffer,dev" ./main
//! ```
//!
//! Example output:
//! ```text
//! libblkid: LOWPROBE: allocate a new probe
//! libblkid: LOWPROBE: zeroize wiper
//! libblkid: LOWPROBE: ready for low-probing, offset=0, size=10485760, zonesize=0
//! libblkid: LOWPROBE: whole-disk: NO, regfile: YES
//! libblkid: LOWPROBE: start probe
//! libblkid: LOWPROBE: zeroize wiper
//! libblkid: LOWPROBE: chain fullprobe superblocks: DISABLED
//! libblkid: LOWPROBE: chain fullprobe topology: DISABLED
//! libblkid: LOWPROBE: chain fullprobe partitions: ENABLED
//! libblkid: LOWPROBE: --> starting probing loop [PARTS idx=-1]
//! libblkid: LOWPROBE:  read: off=0 len=1024
//! libblkid:   BUFFER:  reuse: off=0 len=1024 (for off=0 len=1024)
//! libblkid:   BUFFER:  reuse: off=0 len=1024 (for off=0 len=1024)
//! libblkid:   BUFFER:  reuse: off=0 len=1024 (for off=0 len=1024)
//! libblkid: LOWPROBE:  magic sboff=510, kboff=0
//! libblkid: LOWPROBE: dos: ---> call probefunc()
//! libblkid:   BUFFER:  reuse: off=0 len=1024 (for off=0 len=512)
//! libblkid: LOWPROBE: probably GPT -- ignore
//! libblkid: LOWPROBE: dos: <--- (rc = 1)
//! libblkid: LOWPROBE: gpt: ---> call probefunc()
//! ...snip...
//! ```
//!
//! ```text
//! # rsblkid debug messages only
//! $ RUST_LOG=debug ./main
//! ```
//!
//! Example output:
//! ```text
//! [DEBUG rsblkid::probe::probe_struct] Probe::builder creating new `ProbeBuilder` instance
//! [DEBUG rsblkid::probe::probe_struct] Probe::new_from_file creating new `Probe` instance from `File`
//! [DEBUG rsblkid::probe::probe_struct] Probe::set_device setting device to scan
//! [DEBUG rsblkid::probe::probe_struct] Probe::set_device set device to scan
//! [DEBUG rsblkid::probe::probe_struct] Probe::new created a new `Probe` instance
//! [DEBUG rsblkid::probe::probe_struct] Probe::set_bytes_per_sector setting sector size
//! [DEBUG rsblkid::probe::probe_struct] Probe::set_bytes_per_sector set bytes per sector to 512
//! [DEBUG rsblkid::probe::probe_struct] Probe::disable_chain_superblocks disabling superblocks chain
//! [DEBUG rsblkid::probe::probe_struct] Probe::configure_chain_superblocks enable: false
//! [DEBUG rsblkid::probe::probe_struct] Probe::configure_chain_superblocks disabled superblocks chain
//! [DEBUG rsblkid::probe::probe_struct] Probe::enable_chain_partitions enabling partitions chain
//! [DEBUG rsblkid::probe::probe_struct] Probe::configure_chain_partitions enable: true
//! [DEBUG rsblkid::probe::probe_struct] Probe::configure_chain_partitions enabled partitions chain
//! [DEBUG rsblkid::probe::probe_struct] Probe::find_device_properties collecting all device properties
//! [DEBUG rsblkid::probe::probe_struct] Probe::find_device_properties returned FoundProperties
//! [DEBUG rsblkid::probe::probe_struct] Probe::iter_partitions iterating over list of device partitions
//! [DEBUG rsblkid::probe::partition_iter_struct] PartitionIter::new creating a new `PartitionIter` instance
//! [DEBUG rsblkid::probe::partition_iter_struct] PartitionIter::partition_table accessing `PartitionTable`
//! [DEBUG rsblkid::probe::partition_iter_struct] PartitionIter::partition_table found a `PartitionTable`
//! ...snip...
//! ```
//!
//! Debugging modes can not be modified after calling [`init_default_debug`] or [`init_full_debug`]
//! once. The first function to get called sets the debug mode; a debug mode you can NOT change as
//! long as your program is running.

/// Initializes program debugging messages. This function reads the `LIBBLKID_DEBUG` environment
/// variable to set the level of debug output.
///
/// It accepts the following values:
/// - `all`:      info about all subsystems
/// - `cache`:    blkid tags cache
/// - `config`:   config file utils
/// - `dev`:      device utils
/// - `devname`:  /proc/partitions evaluation
/// - `devno`:    conversions to device name
/// - `evaluate`: tags resolving
/// - `help`:     this help
/// - `lowprobe`: superblock/raids/partitions probing
/// - `buffer`:   low-probing buffers
/// - `probe`:    devices verification
/// - `read`:     cache parsing
/// - `save`:     cache writing
/// - `tag`:      tags utils
///
/// # Examples
///
/// ```text
/// # You can set multiple values separated by commas
/// LIBBLKID_DEBUG="lowprobe,buffer,dev"
/// ```
pub fn init_default_debug() {
    unsafe { libblkid::blkid_init_debug(0) }
}

/// Enables full debugging.
pub fn init_full_debug() {
    unsafe { libblkid::blkid_init_debug(0xffff) }
}
