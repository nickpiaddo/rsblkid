// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::IntoPrimitive;

// From standard library
use std::fmt;

// From this library

/// File system properties.
///
/// Extractable properties:
/// - `TYPE`: file system type.
/// - `SEC_TYPE`: secondary file system type.
/// - `LABEL`: file system label.
/// - `LABEL_RAW`: raw label from a file system superblock.
/// - `UUID`: file system's UUID (lower case).
/// - `UUID_SUB`: subvolume UUID (e.g. for `BTRFS`).
/// - `LOGUUID`: external log UUID (e.g. for `XFS`).
/// - `UUID_RAW`: raw UUID from a file system superblock.
/// - `USAGE`: usage string (i.e. "raid", "file system", etc.).
/// - `VERSION`: file system version.
/// - `SBMAGIC`: super block magic string.
/// - `SBMAGIC_OFFSET`: offset of `SBMAGIC`.
/// - `FSSIZE`: file system size.
/// - `FSLASTBLOCK`: last fsblock/number of file system blocks.
/// - `FSBLOCKSIZE`: size of a file system block in bytes.
///
/// If applicable, the following keys are always extracted:
/// - `BLOCK_SIZE`: minimal block size accessible to the file system.
/// - `MOUNT`: cluster mount name (OCFS only).
/// - `EXT_JOURNAL`: external journal UUID.
/// - `SYSTEM_ID`: ISO9660 system identifier.
/// - `VOLUME_SET_ID`: ISO9660 volume set identifier.
/// - `DATA_PREPARER_ID`: ISO9660 data identifier.
/// - `PUBLISHER_ID`: ISO9660 publisher identifier.
/// - `APPLICATION_ID`: ISO9660 application identifier.
/// - `BOOT_SYSTEM_ID`: ISO9660 boot system identifier.
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive)]
#[non_exhaustive]
#[repr(i32)]
pub enum FsProperty {
    /// Accept bad checksums.
    BadChecksum = libblkid::BLKID_SUBLKS_BADCSUM,
    /// Combination of `Label`, `Uuid`, `Type` and `SecondType`.
    Default = libblkid::BLKID_SUBLKS_DEFAULT,
    /// Extract property `LABEL`.
    Label = libblkid::BLKID_SUBLKS_LABEL,
    /// Extract property `LABEL_RAW`.
    LabelRaw = libblkid::BLKID_SUBLKS_LABELRAW,
    /// Extract properties `SBMAGIC`, and `SBMAGIC_OFFSET`.
    Magic = libblkid::BLKID_SUBLKS_MAGIC,
    /// Extract property `SECTYPE`.
    SecondType = libblkid::BLKID_SUBLKS_SECTYPE,
    /// Extract property `TYPE`.
    Type = libblkid::BLKID_SUBLKS_TYPE,
    /// Extract property `USAGE`.
    Usage = libblkid::BLKID_SUBLKS_USAGE,
    /// Extract properties `UUID`, `UUID_SUB`, and `LOGUUID`.
    Uuid = libblkid::BLKID_SUBLKS_UUID,
    /// Extract property `UUID_RAW`.
    UuidRaw = libblkid::BLKID_SUBLKS_UUIDRAW,
    /// Extract property `VERSION`.
    Version = libblkid::BLKID_SUBLKS_VERSION,
    /// Extract properties `FSSIZE`, `FSLASTBLOCK`, `FSBLOCKSIZE`.
    FsInfo = libblkid::BLKID_SUBLKS_FSINFO,
}

impl FsProperty {
    /// View this `FsProperty` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        match self {
            FsProperty::BadChecksum => "Bad checksum",
            FsProperty::Default => "Default",
            FsProperty::Label => "Label",
            FsProperty::LabelRaw => "Label raw",
            FsProperty::Magic => "Magic",
            FsProperty::SecondType => "Second type",
            FsProperty::Type => "Type",
            FsProperty::Usage => "Usage",
            FsProperty::Uuid => "UUID",
            FsProperty::UuidRaw => "UUID raw",
            FsProperty::Version => "Version",
            FsProperty::FsInfo => "Fs Info",
        }
    }
}

impl fmt::Display for FsProperty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
