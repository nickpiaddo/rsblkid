// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::marker::PhantomData;

// From this library
use crate::probe::Probe;

/// Block device topology.
///
/// > "[Allows access to device] `I/O Limits` information [...] to optimize placement of and access to data.
/// > I/O that is not properly aligned relative to a device's `I/O Limits` will result in reduced
/// > performance or, in the worst case, application failure".
///
/// Source: [[Engineering Notes] I/O Limits: block sizes, alignment and I/O hints](https://access.redhat.com/articles/3911611)
#[derive(Debug)]
pub struct Topology<'a> {
    pub(super) ptr: libblkid::blkid_topology,
    _marker: PhantomData<&'a Probe>,
}

impl<'a> Topology<'a> {
    /// Creates a device `Topology`.
    pub(super) fn new(_: &'a Probe, topology: libblkid::blkid_topology) -> Topology<'a> {
        log::debug!("Topology::new creating a new `Topology` instance");

        Self {
            ptr: topology,
            _marker: PhantomData,
        }
    }

    /// Returns the offset of a block device' beginning from its underlying physical alignment.
    pub fn alignment_offset_in_bytes(&self) -> u64 {
        let offset = unsafe { libblkid::blkid_topology_get_alignment_offset(self.ptr) };
        log::debug!("Topology::alignment_offset_in_bytes offset {:?}", offset);
        offset
    }

    /// Returns `true` when it is possible to directly access a storage device without the
    /// involvement of a file system.
    pub fn supports_dax(&self) -> bool {
        let dax = unsafe { libblkid::blkid_topology_get_dax(self.ptr) == 1 };
        log::debug!(
            "Topology::supports_dax supports storage direct access: {:?}",
            dax
        );
        dax
    }

    /// Returns the device's preferred minimum unit in bytes for random I/O.
    pub fn minimum_io_size(&self) -> u64 {
        let min_io = unsafe { libblkid::blkid_topology_get_minimum_io_size(self.ptr) };
        log::debug!("Topology::minimum_io_size minimum I/O size: {:?}", min_io);
        min_io
    }

    /// Returns the device's preferred minimum unit in bytes for streaming I/O.
    pub fn optimal_io_size(&self) -> u64 {
        let opt_io = unsafe { libblkid::blkid_topology_get_optimal_io_size(self.ptr) };
        log::debug!("Topology::optimal_io_size optimal I/O size: {:?}", opt_io);
        opt_io
    }

    /// Returns the finer-grained sector size in bytes exposed to Linux.
    pub fn logical_sector_size(&self) -> u64 {
        let size = unsafe { libblkid::blkid_topology_get_logical_sector_size(self.ptr) };
        log::debug!(
            "Topology::logical_sector_size logical sector size: {:?}",
            size
        );
        size
    }

    /// Returns the internal physical size in bytes of a sector on a device.
    pub fn physical_sector_size(&self) -> u64 {
        let phys_size = unsafe { libblkid::blkid_topology_get_physical_sector_size(self.ptr) };
        log::debug!(
            "Topology::physical_sector_size physical sector size: {:?}",
            phys_size
        );
        phys_size
    }
}
