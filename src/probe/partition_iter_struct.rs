// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::probe::{Partition, PartitionTable, Probe};

/// Iterator over a collection of [`Partition`]s.
#[derive(Debug)]
pub struct PartitionIter<'a> {
    ptr: Option<libblkid::blkid_partlist>,
    marker: &'a Probe,
    counter: usize,
}

impl<'a> PartitionIter<'a> {
    /// Returns the top-level [`PartitionTable`] if it exists.
    pub fn partition_table(&self) -> Option<PartitionTable> {
        self.ptr.and_then(|ptr| {

            log::debug!("PartitionIter::partition_table accessing `PartitionTable`");
            let mut table_ptr = MaybeUninit::<libblkid::blkid_parttable>::uninit();

            unsafe {
                table_ptr.write(libblkid::blkid_partlist_get_table(ptr));
            }

            match unsafe { table_ptr.assume_init() } {
                table if table.is_null() => {
                    log::debug!("PartitionIter::partition_table found no partition table. libblkid::blkid_partlist_get_table returned a NULL pointer");

                    None
                }
                table => {
                    log::debug!("PartitionIter::partition_table found a `PartitionTable`");

                    Some(PartitionTable::new(self.marker, table))
                }
            }
        })
    }

    /// Returns a [`Partition`] from a partition number if it exists.
    pub fn nth_by_partition_number(&mut self, partition_number: usize) -> Option<Partition> {
        log::debug!(
            "PartitionIter::nth_by_partition_number accessing partition {:?}",
            partition_number
        );
        if partition_number > 0 {
            log::debug!(
                "PartitionIter::nth_by_partition_number found partition numbered {:?}",
                partition_number
            );

            self.nth(partition_number - 1)
        } else {
            log::debug!(
                "PartitionIter::nth_by_partition_number no partition numbered {:?}",
                partition_number
            );

            None
        }
    }

    // This function tries to get start and size for devno from sysfs and returns a partition from ls which matches with the values from sysfs.
    //
    // This function is necessary when you want to make a relation between an entry in the partition table (ls ) and block devices in your system.
    pub fn partition_from_device_number(&self, device_number: u64) -> Option<Partition> {
        self.ptr.and_then(|ptr| {
            log::debug!("PartitionIter::partition_from_device_number getting `Partition` from device number {:?}", device_number);
            let mut partition_ptr = MaybeUninit::<libblkid::blkid_partition>::uninit();

            unsafe {
                partition_ptr.write(libblkid::blkid_partlist_devno_to_partition(
                    ptr,
                    device_number,
                ));
            }

            match unsafe { partition_ptr.assume_init() } {
                partition if partition.is_null() => {
                    log::debug!("PartitionIter::partition_from_device_number found no partition with device number {:?}. libblkid::blkid_partlist_devno_to_partition returned a NULL pointer", device_number);

                    None
                }
                partition => {
                    log::debug!("PartitionIter::partition_from_device_number found `Partition` with device number {:?}", device_number);

                    Some(Partition::new(self.marker, partition))
                }
            }

        })
    }
}

impl<'a> Iterator for PartitionIter<'a> {
    type Item = Partition<'a>;

    /// Advances the iterator and returns the next value.
    fn next(&mut self) -> Option<Self::Item> {
        log::debug!(
            "PartitionIter::next advancing to element: {:?}",
            self.counter
        );

        self.ptr.and_then(|ptr| {
            // Assume that we reached the end of the iterator if the number of calls to
            // PartitionIter exceed the largest possible i32 value.
            let index = i32::try_from(self.counter).ok()?;

            let mut partition_ptr = MaybeUninit::<libblkid::blkid_partition>::uninit();

            unsafe {
                partition_ptr.write(libblkid::blkid_partlist_get_partition(ptr, index));
            }

            match unsafe { partition_ptr.assume_init() } {
                partition if partition.is_null() => {
                    log::debug!("PartitionIter::next reached end of iterator");

                    None
                }
                partition => {
                    log::debug!(
                        "PartitionIter::next found partition: {:?}",
                        self.counter + 1
                    );

                    self.counter += 1;
                    Some(Partition::new(self.marker, partition))
                }
            }
        })
    }
}
