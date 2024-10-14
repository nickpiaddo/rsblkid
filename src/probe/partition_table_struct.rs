// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::core::partition::PartitionTableType;
use crate::ffi_utils;
use crate::probe::{Partition, Probe};

/// A device's partition table.
#[derive(Debug)]
pub struct PartitionTable<'a> {
    pub(super) ptr: libblkid::blkid_parttable,
    marker: &'a Probe,
}

impl<'a> PartitionTable<'a> {
    #[doc(hidden)]
    /// Creates a new `PartitionTable` instance.
    pub(super) fn new(marker: &'a Probe, table: libblkid::blkid_parttable) -> PartitionTable<'a> {
        log::debug!("PartitionTable::new creating a new `PartitionTable` instance");

        Self { ptr: table, marker }
    }

    /// Returns a `GPT GUID` or a `DOS ID` in hexadecimal, if the type of partition table is `DOS` or `GPT`.
    pub fn id(&self) -> Option<String> {
        log::debug!("PartitionTable::id getting a partition table's ID");

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libblkid::blkid_parttable_get_id(self.ptr));
        }

        match unsafe { ptr.assume_init() } {
            id_ptr if id_ptr.is_null() => {
                log::debug!("PartitionTable::id failed to get partition table's ID. libblkid::blkid_parttable_get_id returned a NULL pointer");

                None
            }
            id_ptr => {
                let hex_id = ffi_utils::c_char_array_to_string(id_ptr);
                log::debug!("PartitionTable::id partition table ID: {:?}", hex_id);

                Some(hex_id)
            }
        }
    }

    /// Returns the partition table's location, in bytes:
    /// - with respect to the beginning of a device for a **primary partition table**,
    /// - relative to a parent partition's location for a **nested partition table**.
    ///
    /// To get a nested partition table's absolute location, add the value of its parent
    /// partition absolute location.
    ///
    /// ```ignore
    /// let parent_partition = nested_partition_table.parent().unwrap();
    /// let location_of_parent_partition = parent_partition.location_in_bytes();
    ///
    /// let relative_location_of_nested_table = nested_partition_table.location_in_bytes().unwrap();
    ///
    /// // nested table's location with respect to a device's first byte
    /// let absolute_location = location_of_parent_partition + relative_location_of_nested_table;
    /// ```
    ///
    pub fn location_in_bytes(&self) -> Option<u64> {
        log::debug!("PartitionTable::location_in_bytes getting partition table's location");

        let result = unsafe { libblkid::blkid_parttable_get_offset(self.ptr) };

        match result {
            location if location >= 0 => {
                let location = location as u64;
                log::debug!(
                    "PartitionTable::location_in_bytes partition table's location: {:?}",
                    location
                );

                Some(location)
            }
            code => {
                let err_msg = "failed to get partition table's location on device".to_owned();
                log::debug!("PartitionTable::location_in_bytes {}. libblkid::blkid_parttable_get_offset returned error code {:?}",err_msg, code);

                None
            }
        }
    }

    /// Returns the nested partition table's parent partition, if applicable.
    pub fn parent(&self) -> Option<Partition> {
        log::debug!("PartitionTable::parent getting a partition table's parent partition");

        unsafe {
            let mut ptr = MaybeUninit::<libblkid::blkid_partition>::zeroed();
            ptr.write(libblkid::blkid_parttable_get_parent(self.ptr));

            match ptr.assume_init() {
                partition if partition.is_null() => {
                    log::debug!("PartitionTable::parent failed to get partition table's parent partition. libblkid::blkid_parttable_get_parent returned a NULL pointer");

                    None
                }
                partition => {
                    log::debug!("PartitionTable::parent got partition table's parent partition");

                    Some(Partition::new(self.marker, partition))
                }
            }
        }
    }

    /// Returns the partition tables type.
    pub fn partition_table_type(&self) -> Option<PartitionTableType> {
        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libblkid::blkid_parttable_get_type(self.ptr));
        }

        let ptr = unsafe { ptr.assume_init() };
        let pt_type = ffi_utils::const_c_char_array_to_bytes(ptr);
        let table_type = PartitionTableType::try_from(pt_type).ok();

        log::debug!(
            "PartitionTable::partition_table_type partition table's type: {:?}",
            table_type
        );

        table_type
    }
}

impl<'a> PartialEq for PartitionTable<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<'a> Eq for PartitionTable<'a> {}
