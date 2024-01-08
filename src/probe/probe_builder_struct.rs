// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use typed_builder::TypedBuilder;

// From standard library
use std::fs::File;
use std::path::PathBuf;

// From this library
use crate::probe::{Probe, ProbeBuilderError};

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
}

#[allow(non_camel_case_types)]
impl<
        __scan_device: ::typed_builder::Optional<Option<PathBuf>>,
        __scan_file: ::typed_builder::Optional<Option<File>>,
        __allow_writes: ::typed_builder::Optional<bool>,
        __bytes_per_sector: ::typed_builder::Optional<u32>,
        __scan_device_segment: ::typed_builder::Optional<(u64, u64)>,
    >
    ProbeBuilder<(
        __scan_device,
        __scan_file,
        __allow_writes,
        __bytes_per_sector,
        __scan_device_segment,
    )>
{
    /// Finishes configuring, and creates a new [`Probe`] instance.
    pub fn build(self) -> Result<Probe, ProbeBuilderError> {
        let builder = self.__build();
        let probe = match (builder.scan_device, builder.scan_file, builder.allow_writes) {
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

        Ok(probe)
    }
}
