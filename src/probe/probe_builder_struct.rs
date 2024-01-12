// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use typed_builder::TypedBuilder;

// From standard library
use std::fs::File;
use std::path::PathBuf;

// From this library
use crate::core::device::Usage;
use crate::core::partition::FileSystem;
use crate::probe::Filter;
use crate::probe::FsProperty;
use crate::probe::Probe;
use crate::probe::ProbeBuilderError;

#[derive(Debug, TypedBuilder)]
#[builder(builder_type(name = ProbeBuilder, vis = "pub", doc ="Configures and creates a new [`Probe`] instance.\n\nFor usage, see [`ProbeBuilder::build`] or the overview of the [`probe`](crate::probe#overview) module."),
    build_method(vis = "", name = __build))]
pub(crate) struct PrbBuilder {
    #[builder(
        default,
        setter(into, strip_option),
        setter(doc = "Sets the path to the device to associate with a [`Probe`].")
    )]
    scan_device: Option<PathBuf>,

    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets the [`File`] object, providing access to an open device, as the device to associate with a [`Probe`]."
        )
    )]
    scan_file: Option<File>,

    #[builder(
        setter(strip_bool),
        setter(doc = "Sets a [`Probe`] to read/write mode.")
    )]
    allow_writes: bool,

    #[builder(
        default = 512,
        setter(doc = "Sets the number of bytes per sector on the device.")
    )]
    bytes_per_sector: u32,

    #[builder(default = (0,0),
        setter(transform = |location: u64, size: u64| (location, size),
        doc = "Sets the region to scan on the [`Probe`]'s associated device.\n\n# Arguments\n\n-
`location` -- offset in bytes.\n- `size` -- region's size in bytes."))]
    scan_device_segment: (u64, u64),
    #[builder(
        default = true,
        setter(
            doc = "Deactivates file system search functions when set to `false`. By default, set to `true`."
        )
    )]
    scan_device_superblocks: bool,
    #[builder(default = None, setter(transform = |criterion: Filter, fs_types:
            Vec<FileSystem>| Some((criterion, fs_types)), doc = "Specifies which file systems to
search for/exclude when scanning a device. By default, a [`Probe`] will try to identify
any of the supported [`FileSystem`]s,")) ]
    scan_superblocks_for_file_systems: Option<(Filter, Vec<FileSystem>)>,

    #[builder(default = None, setter(transform = |criterion: Filter, usage: Vec<Usage>|
            Some((criterion, usage)), doc = "Limits file system scanning to superblocks with
particular [`Usage`](crate::core::device::Usage) flags."))]
    scan_superblocks_with_usage_flags: Option<(Filter, Vec<Usage>)>,
    #[builder(default = None, setter(strip_option, doc = "Sets the list of file system properties ([`FsProperty`](flag::FsProperty)) to collect."))]
    collect_fs_properties: Option<Vec<FsProperty>>,
}

#[allow(non_camel_case_types)]
impl<
        __scan_device: ::typed_builder::Optional<Option<PathBuf>>,
        __scan_file: ::typed_builder::Optional<Option<File>>,
        __allow_writes: ::typed_builder::Optional<bool>,
        __bytes_per_sector: ::typed_builder::Optional<u32>,
        __scan_device_segment: ::typed_builder::Optional<(u64, u64)>,
        __scan_device_superblocks: ::typed_builder::Optional<bool>,
        __scan_superblocks_for_file_systems: ::typed_builder::Optional<Option<(Filter, Vec<FileSystem>)>>,
        __scan_superblocks_with_usage_flags: ::typed_builder::Optional<Option<(Filter, Vec<Usage>)>>,
        __collect_fs_properties: ::typed_builder::Optional<Option<Vec<FsProperty>>>,
    >
    ProbeBuilder<(
        __scan_device,
        __scan_file,
        __allow_writes,
        __bytes_per_sector,
        __scan_device_segment,
        __scan_device_superblocks,
        __scan_superblocks_for_file_systems,
        __scan_superblocks_with_usage_flags,
        __collect_fs_properties,
    )>
{
    /// Finishes configuring, and creates a new [`Probe`] instance.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use rsblkid::core::device::Usage;
    /// use rsblkid::core::partition::FileSystem;
    /// use rsblkid::probe::{Filter, FsProperty, Probe};
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let probe_builder = Probe::builder();
    ///
    ///     let probe = probe_builder
    ///         .scan_device("/dev/vda")
    ///         // Open device in read/write mode. By default, a Probe opens a device in read-only
    ///         // mode.
    ///         .allow_writes()
    ///         // Set bytes per sector.
    ///         .bytes_per_sector(1024)
    ///         // Scan the whole device (i.e. start at byte 0, for a length of 0 which is
    ///         // interpreted as the whole disk).
    ///         .scan_device_segment(0, 0)
    ///         // Activate file system search functions. By default, device superblocks scanning
    ///         // is automatically activated.
    ///         .scan_device_superblocks(true)
    ///         // Specify which file systems to search for when scanning the device, by default all
    ///         // supported file system identification functions are tried.
    ///         .scan_superblocks_for_file_systems(Filter::In,
    ///             vec![
    ///                 FileSystem::APFS,
    ///                 FileSystem::Ext4,
    ///                 FileSystem::VFAT,
    ///             ])
    ///         // Exclude superblocks with usage flags matching the ones in the list.
    ///         .scan_superblocks_with_usage_flags(Filter::Out,
    ///             vec![
    ///                 Usage::Crypto,
    ///                 Usage::Raid
    ///             ])
    ///         // Collect file system properties matching the ones specified.
    ///         .collect_fs_properties(
    ///             vec![
    ///                 FsProperty::Label,
    ///                 FsProperty::Uuid,
    ///                 FsProperty::FsInfo,
    ///                 FsProperty::Version,
    ///             ]
    ///         )
    ///         .build();
    ///
    ///     assert!(probe.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn build(self) -> Result<Probe, ProbeBuilderError> {
        let builder = self.__build();
        let mut probe = match (builder.scan_device, builder.scan_file, builder.allow_writes) {
            (None, None, _) => Err(ProbeBuilderError::Required(
                "one of the options `scan_device` or `scan_file` must be set".to_string(),
            )),
            (Some(_), Some(_), _) => Err(ProbeBuilderError::MutuallyExclusive(
                "can not set `scan_device` and `scan_file` simultaneously".to_string(),
            )),
            // Scan device from path in read only mode.
            (Some(path), None, false) => Probe::new_read_only(path, builder.scan_device_segment)
                .map_err(ProbeBuilderError::from),
            // Scan device from path in read/write mode.
            (Some(path), None, true) => Probe::new_read_write(path, builder.scan_device_segment)
                .map_err(ProbeBuilderError::from),
            // Scan device from an already opened read-only device file.
            (None, Some(file), false) => Probe::new_from_file(file, builder.scan_device_segment)
                .map_err(ProbeBuilderError::from),
            // Scan device from an already opened read/write device file.
            (None, Some(file), true) => {
                Probe::new_from_file_read_write(file, builder.scan_device_segment)
                    .map_err(ProbeBuilderError::from)
            }
        }?;

        probe.set_bytes_per_sector(builder.bytes_per_sector)?;

        if builder.scan_device_superblocks {
            probe.enable_chain_superblocks()?
        } else {
            probe.disable_chain_superblocks()?
        }

        if let Some((criterion, fs_types)) = builder.scan_superblocks_for_file_systems {
            probe.scan_superblocks_for_file_systems(criterion, fs_types.as_slice())?
        }

        if let Some((criterion, usage)) = builder.scan_superblocks_with_usage_flags {
            probe.scan_superblocks_with_usage_flags(criterion, usage.as_slice())?
        }

        if let Some(sb_flags) = builder.collect_fs_properties {
            probe.collect_fs_properties(sb_flags.as_slice())?
        }

        Ok(probe)
    }
}
