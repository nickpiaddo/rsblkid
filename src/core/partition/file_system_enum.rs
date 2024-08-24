// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use enum_iterator::Sequence;

// From standard library
use std::ffi::CString;
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// Supported file systems.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Sequence)]
#[non_exhaustive]
pub enum FileSystem {
    /// Name: `"adaptec_raid_member"`
    AdaptecRaid,
    /// Name: `"apfs"`
    APFS,
    /// Name: `"bcache"`
    Bcache,
    /// Name: `"bcachefs"`
    BcacheFs,
    /// Name: `"befs"`
    BeFS,
    /// Name: `"bfs"`
    BFS,
    /// Name: `"BitLocker"`
    BitLocker,
    /// Name: `"ceph_bluestore"`
    BlueStore,
    /// Name: `"btrfs"`
    BTRFS,
    /// Name: `"cramfs"`
    Cramfs,
    /// Name: `"ddf_raid_member"`
    DDFRaid,
    /// Name: `"DM_integrity"`
    DmIntegrity,
    /// Name: `"DM_snapshot_cow"`
    DmSnapshot,
    /// Name: `"DM_verify_hash"`
    DmVerify,
    /// Name: `"drbd"`
    DRBD,
    /// Name: `"drbdmanage_control_volume"`
    DRBDManage,
    /// Name: `"drbdproxy_datalog"`
    DRBDProxyDatalog,
    /// Name: `"erofs"`
    EROFS,
    /// Name: `"exfat"`
    ExFAT,
    /// Name: `"exfs"`
    Exfs,
    /// Name: `"ext2"`
    Ext2,
    /// Name: `"ext3"`
    Ext3,
    /// Name: `"ext4"`
    Ext4,
    /// Name: `"ext4dev"`
    Ext4Dev,
    /// Name: `"f2fs"`
    F2FS,
    /// Name: `"cs_fvault2"`
    FileVault,
    /// Name: `"gfs"`
    GFS,
    /// Name: `"gfs2"`
    GFS2,
    /// Name: `"hfs"`
    HFS,
    /// Name: `"hfsplus"`
    HFSPlus,
    /// Name: `"hpt37x_raid_member"`
    HighPoint37x,
    /// Name: `"hpt45x_raid_member"`
    HighPoint45x,
    /// Name: `"hpfs"`
    HPFS,
    /// Name: `"iso9660"`
    Iso9660,
    /// Name: `"isw_raid_member"`
    ISWRaid,
    /// Name: `"jbd"`
    JBD,
    /// Name: `"jfs"`
    JFS,
    /// Name: `"jmicron_raid_member"`
    JmicronRaid,
    /// Name: `"linux_raid_member"`
    LinuxRaid,
    /// Name: `"lsi_mega_raid_member"`
    LSIRaid,
    /// Name: `"crypto_LUKS"`
    LUKS,
    /// Name: `"LVM1_member"`
    LVM1,
    /// Name: `"LVM2_member"`
    LVM2,
    /// Name: `"minix"`
    Minix,
    /// Name: `"mpool"`
    Mpool,
    /// Name: `"msdos"`
    MSDOS,
    /// Name: `"nss"`
    Netware,
    /// Name: `"nilfs2"`
    Nilfs2,
    /// Name: `"ntfs"`
    NTFS,
    /// Name: `"nvidia_raid_member"`
    NvidiaRaid,
    /// Name: `"ocfs"`
    OCFS,
    /// Name: `"ocfs2"`
    OCFS2,
    /// Name: `"promise_fasttrack_raid_member"`
    PromiseRaid,
    /// Name: `"ReFs"`
    ReFs,
    /// Name: `"reiserfs"`
    Reiserfs,
    /// Name: `"reiser4"`
    Reiser4,
    /// Name: `"romfs"`
    Romfs,
    /// Name: `"silicon_medley_raid_member"`
    SiliconRaid,
    /// Name: `"squashfs"`
    Squashfs,
    /// Name: `"squashfs3"`
    Squashfs3,
    /// Name: `"stratis"`
    Stratis,
    /// Name: `"swap"`
    Swap,
    /// Name: `"swsuspend"`
    SwapSuspend,
    /// Name: `"sysv"`
    SYSV,
    /// Name: `"ubi"`
    UBI,
    /// Name: `"ubifs"`
    UBIFS,
    /// Name: `"udf"`
    UDF,
    /// Name: `"ufs"`
    UFS,
    /// Name: `"vdo"`
    VDO,
    /// Name: `"vfat"`
    VFAT,
    /// Name: `"via_raid_member"`
    VIARaid,
    /// Name: `"VMFS"`
    VMFS,
    /// Name: `"VMFS_volume_member"`
    VMFSVolume,
    /// Name: `"vxfs"`
    Vxfs,
    /// Name: `"xenix"`
    Xenix,
    /// Name: `"xfs"`
    XFS,
    /// Name: `"xfs_external_log"`
    XFSLog,
    /// Name: `"zfs_member"`
    ZFS,
    /// Name: `"zonefs"`
    ZoneFS,
}

impl FileSystem {
    // Each known filesystem is represented in `util-linux/libblkid/src/superblocks`
    // by a structure at the end of each file in the directory.
    //
    // For example in `util-linux/libblkid/src/superblock/svia_raid.c`
    //
    // const struct blkid_idinfo viaraid_idinfo = {
    //	.name		= "via_raid_member",
    //	.usage		= BLKID_USAGE_RAID,
    //	.probefunc	= probe_viaraid,
    //	.magics		= BLKID_NONE_MAGIC
    //};
    //
    // the attribute `name` is the ID used by `libblkid` to access the function `probe_viaraid`
    // to identify the type of superblock encountered during a probe.

    /// View this `FileSystem` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        match self {
            Self::AdaptecRaid => "adaptec_raid_member",
            Self::APFS => "apfs",
            Self::Bcache => "bcache",
            Self::BcacheFs => "bcachefs",
            Self::BeFS => "befs",
            Self::BFS => "bfs",
            Self::BitLocker => "BitLocker",
            Self::BlueStore => "ceph_bluestore",
            Self::BTRFS => "btrfs",
            Self::Cramfs => "cramfs",
            Self::DDFRaid => "ddf_raid_member",
            Self::DmIntegrity => "DM_integrity",
            Self::DmSnapshot => "DM_snapshot_cow",
            Self::DmVerify => "DM_verify_hash",
            Self::DRBD => "drbd",
            Self::DRBDManage => "drbdmanage_control_volume",
            Self::DRBDProxyDatalog => "drbdproxy_datalog",
            Self::EROFS => "erofs",
            Self::ExFAT => "exfat",
            Self::Exfs => "exfs",
            Self::Ext2 => "ext2",
            Self::Ext3 => "ext3",
            Self::Ext4 => "ext4",
            Self::Ext4Dev => "ext4dev",
            Self::F2FS => "f2fs",
            Self::FileVault => "cs_fvault2",
            Self::GFS => "gfs",
            Self::GFS2 => "gfs2",
            Self::HFS => "hfs",
            Self::HFSPlus => "hfsplus",
            Self::HighPoint37x => "hpt37x_raid_member",
            Self::HighPoint45x => "hpt45x_raid_member",
            Self::HPFS => "hpfs",
            Self::Iso9660 => "iso9660",
            Self::ISWRaid => "isw_raid_member",
            Self::JBD => "jbd",
            Self::JFS => "jfs",
            Self::JmicronRaid => "jmicron_raid_member",
            Self::LinuxRaid => "linux_raid_member",
            Self::LSIRaid => "lsi_mega_raid_member",
            Self::LUKS => "crypto_LUKS",
            Self::LVM1 => "LVM1_member",
            Self::LVM2 => "LVM2_member",
            Self::Minix => "minix",
            Self::Mpool => "mpool",
            Self::MSDOS => "msdos",
            Self::Netware => "nss",
            Self::Nilfs2 => "nilfs2",
            Self::NTFS => "ntfs",
            Self::NvidiaRaid => "nvidia_raid_member",
            Self::OCFS => "ocfs",
            Self::OCFS2 => "ocfs2",
            Self::PromiseRaid => "promise_fasttrack_raid_member",
            Self::ReFs => "ReFs",
            Self::Reiserfs => "reiserfs",
            Self::Reiser4 => "reiser4",
            Self::Romfs => "romfs",
            Self::SiliconRaid => "silicon_medley_raid_member",
            Self::Squashfs => "squashfs",
            Self::Squashfs3 => "squashfs3",
            Self::Stratis => "stratis",
            Self::Swap => "swap",
            Self::SwapSuspend => "swsuspend",
            Self::SYSV => "sysv",
            Self::UBI => "ubi",
            Self::UBIFS => "ubifs",
            Self::UDF => "udf",
            Self::UFS => "ufs",
            Self::VDO => "vdo",
            Self::VFAT => "vfat",
            Self::VIARaid => "via_raid_member",
            Self::VMFS => "VMFS",
            Self::VMFSVolume => "VMFS_volume_member",
            Self::Vxfs => "vxfs",
            Self::Xenix => "xenix",
            Self::XFS => "xfs",
            Self::XFSLog => "xfs_external_log",
            Self::ZFS => "zfs_member",
            Self::ZoneFS => "zonefs",
        }
    }

    /// Converts this `Filesystem` to a [`CString`].
    pub fn to_c_string(&self) -> CString {
        // FileSystem's string representation does not contain NULL characters,  we can safely
        // unwrap the new CString.
        CString::new(self.as_str()).unwrap()
    }
}

impl AsRef<FileSystem> for FileSystem {
    #[inline]
    fn as_ref(&self) -> &FileSystem {
        self
    }
}

impl AsRef<str> for FileSystem {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&[u8]> for FileSystem {
    type Error = ConversionError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(bytes)
            .map_err(|e| {
                ConversionError::FileSystem(format!(
                    "bytes to UTF-8 string slice conversion error. {:?}",
                    e
                ))
            })
            .and_then(|s| Self::from_str(s).map_err(|e| ConversionError::FileSystem(e.to_string())))
    }
}

impl TryFrom<Vec<u8>> for FileSystem {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl FromStr for FileSystem {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Remove opening opening/closing quotes/double-quotes if present
        let err_missing_dquote = format!("missing closing double-quote in: {}", s);
        let err_missing_quote = format!("missing closing quote in: {}", s);

        let trimmed = s.trim();
        let stripped = if trimmed.starts_with('"') {
            trimmed
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .ok_or(ParserError::FileSystem(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::FileSystem(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        match stripped.trim() {
            "adaptec_raid_member" => Ok(Self::AdaptecRaid),
            "apfs" => Ok(Self::APFS),
            "bcache" => Ok(Self::Bcache),
            "bcachefs" => Ok(Self::BcacheFs),
            "befs" => Ok(Self::BeFS),
            "bfs" => Ok(Self::BFS),
            "BitLocker" => Ok(Self::BitLocker),
            "ceph_bluestore" => Ok(Self::BlueStore),
            "btrfs" => Ok(Self::BTRFS),
            "cramfs" => Ok(Self::Cramfs),
            "ddf_raid_member" => Ok(Self::DDFRaid),
            "DM_integrity" => Ok(Self::DmIntegrity),
            "DM_snapshot_cow" => Ok(Self::DmSnapshot),
            "DM_verify_hash" => Ok(Self::DmVerify),
            "drbd" => Ok(Self::DRBD),
            "drbdmanage_control_volume" => Ok(Self::DRBDManage),
            "drbdproxy_datalog" => Ok(Self::DRBDProxyDatalog),
            "erofs" => Ok(Self::EROFS),
            "exfat" => Ok(Self::ExFAT),
            "exfs" => Ok(Self::Exfs),
            "ext2" => Ok(Self::Ext2),
            "ext3" => Ok(Self::Ext3),
            "ext4" => Ok(Self::Ext4),
            "ext4dev" => Ok(Self::Ext4Dev),
            "f2fs" => Ok(Self::F2FS),
            "cs_fvault2" => Ok(Self::FileVault),
            "gfs" => Ok(Self::GFS),
            "gfs2" => Ok(Self::GFS2),
            "hfs" => Ok(Self::HFS),
            "hfsplus" => Ok(Self::HFSPlus),
            "hpt37x_raid_member" => Ok(Self::HighPoint37x),
            "hpt45x_raid_member" => Ok(Self::HighPoint45x),
            "hpfs" => Ok(Self::HPFS),
            "iso9660" => Ok(Self::Iso9660),
            "isw_raid_member" => Ok(Self::ISWRaid),
            "jbd" => Ok(Self::JBD),
            "jfs" => Ok(Self::JFS),
            "jmicron_raid_member" => Ok(Self::JmicronRaid),
            "linux_raid_member" => Ok(Self::LinuxRaid),
            "lsi_mega_raid_member" => Ok(Self::LSIRaid),
            "crypto_LUKS" => Ok(Self::LUKS),
            "LVM1_member" => Ok(Self::LVM1),
            "LVM2_member" => Ok(Self::LVM2),
            "minix" => Ok(Self::Minix),
            "mpool" => Ok(Self::Mpool),
            "msdos" => Ok(Self::MSDOS),
            "nss" => Ok(Self::Netware),
            "nilfs2" => Ok(Self::Nilfs2),
            "ntfs" => Ok(Self::NTFS),
            "nvidia_raid_member" => Ok(Self::NvidiaRaid),
            "ocfs" => Ok(Self::OCFS),
            "ocfs2" => Ok(Self::OCFS2),
            "promise_fasttrack_raid_member" => Ok(Self::PromiseRaid),
            "ReFs" => Ok(Self::ReFs),
            "reiserfs" => Ok(Self::Reiserfs),
            "reiser4" => Ok(Self::Reiser4),
            "romfs" => Ok(Self::Romfs),
            "silicon_medley_raid_member" => Ok(Self::SiliconRaid),
            "squashfs" => Ok(Self::Squashfs),
            "squashfs3" => Ok(Self::Squashfs3),
            "stratis" => Ok(Self::Stratis),
            "swap" => Ok(Self::Swap),
            "swsuspend" => Ok(Self::SwapSuspend),
            "sysv" => Ok(Self::SYSV),
            "ubi" => Ok(Self::UBI),
            "ubifs" => Ok(Self::UBIFS),
            "udf" => Ok(Self::UDF),
            "ufs" => Ok(Self::UFS),
            "vdo" => Ok(Self::VDO),
            "vfat" => Ok(Self::VFAT),
            "via_raid_member" => Ok(Self::VIARaid),
            "VMFS" => Ok(Self::VMFS),
            "VMFS_volume_member" => Ok(Self::VMFSVolume),
            "vxfs" => Ok(Self::Vxfs),
            "xenix" => Ok(Self::Xenix),
            "xfs" => Ok(Self::XFS),
            "xfs_external_log" => Ok(Self::XFSLog),
            "zfs_member" => Ok(Self::ZFS),
            "zonefs" => Ok(Self::ZoneFS),
            _unsupported => {
                let err_msg = format!("unsupported file system: {:?}", s);
                Err(ParserError::FileSystem(err_msg))
            }
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(expected = "unsupported file system")]
    fn file_system_can_not_parse_an_empty_string() {
        let _: FileSystem = "".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn file_system_can_not_parse_a_file_system_string_with_an_unclosed_double_quote() {
        let _: FileSystem = r#""ufs"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn file_system_can_not_parse_a_file_system_string_with_an_unclosed_quote() {
        let _: FileSystem = "'ufs".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "unsupported file system")]
    fn file_system_can_not_parse_an_invalid_file_system_type() {
        let _: FileSystem = "DUMMY".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string slice conversion error")]
    fn file_system_can_not_convert_invalid_bytes_into_a_file_system() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = FileSystem::try_from(bytes).unwrap();
    }

    #[test]
    fn file_system_can_convert_valid_bytes_into_a_file_system() -> crate::Result<()> {
        let bytes: Vec<u8> = b"ext4".to_vec();
        let actual = FileSystem::try_from(bytes)?;
        let expected = FileSystem::Ext4;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn file_system_can_parse_a_valid_file_system_type() -> crate::Result<()> {
        let fs_str = "adaptec_raid_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::AdaptecRaid;
        assert_eq!(actual, expected);

        let fs_str = "apfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::APFS;
        assert_eq!(actual, expected);

        let fs_str = "bcache";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Bcache;
        assert_eq!(actual, expected);

        let fs_str = "bcachefs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::BcacheFs;
        assert_eq!(actual, expected);

        let fs_str = "befs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::BeFS;
        assert_eq!(actual, expected);

        let fs_str = "bfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::BFS;
        assert_eq!(actual, expected);

        let fs_str = "BitLocker";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::BitLocker;
        assert_eq!(actual, expected);

        let fs_str = "ceph_bluestore";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::BlueStore;
        assert_eq!(actual, expected);

        let fs_str = "btrfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::BTRFS;
        assert_eq!(actual, expected);

        let fs_str = "cramfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Cramfs;
        assert_eq!(actual, expected);

        let fs_str = "ddf_raid_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::DDFRaid;
        assert_eq!(actual, expected);

        let fs_str = "DM_integrity";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::DmIntegrity;
        assert_eq!(actual, expected);

        let fs_str = "DM_snapshot_cow";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::DmSnapshot;
        assert_eq!(actual, expected);

        let fs_str = "DM_verify_hash";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::DmVerify;
        assert_eq!(actual, expected);

        let fs_str = "drbd";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::DRBD;
        assert_eq!(actual, expected);

        let fs_str = "drbdmanage_control_volume";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::DRBDManage;
        assert_eq!(actual, expected);

        let fs_str = "drbdproxy_datalog";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::DRBDProxyDatalog;
        assert_eq!(actual, expected);

        let fs_str = "erofs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::EROFS;
        assert_eq!(actual, expected);

        let fs_str = "exfat";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::ExFAT;
        assert_eq!(actual, expected);

        let fs_str = "exfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Exfs;
        assert_eq!(actual, expected);

        let fs_str = "ext2";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Ext2;
        assert_eq!(actual, expected);

        let fs_str = "ext3";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Ext3;
        assert_eq!(actual, expected);

        let fs_str = "ext4";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Ext4;
        assert_eq!(actual, expected);

        let fs_str = "ext4dev";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Ext4Dev;
        assert_eq!(actual, expected);

        let fs_str = "f2fs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::F2FS;
        assert_eq!(actual, expected);

        let fs_str = "cs_fvault2";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::FileVault;
        assert_eq!(actual, expected);

        let fs_str = "gfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::GFS;
        assert_eq!(actual, expected);

        let fs_str = "gfs2";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::GFS2;
        assert_eq!(actual, expected);

        let fs_str = "hfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::HFS;
        assert_eq!(actual, expected);

        let fs_str = "hfsplus";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::HFSPlus;
        assert_eq!(actual, expected);

        let fs_str = "hpt37x_raid_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::HighPoint37x;
        assert_eq!(actual, expected);

        let fs_str = "hpt45x_raid_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::HighPoint45x;
        assert_eq!(actual, expected);

        let fs_str = "hpfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::HPFS;
        assert_eq!(actual, expected);

        let fs_str = "iso9660";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Iso9660;
        assert_eq!(actual, expected);

        let fs_str = "isw_raid_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::ISWRaid;
        assert_eq!(actual, expected);

        let fs_str = "jbd";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::JBD;
        assert_eq!(actual, expected);

        let fs_str = "jfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::JFS;
        assert_eq!(actual, expected);

        let fs_str = "jmicron_raid_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::JmicronRaid;
        assert_eq!(actual, expected);

        let fs_str = "linux_raid_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::LinuxRaid;
        assert_eq!(actual, expected);

        let fs_str = "lsi_mega_raid_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::LSIRaid;
        assert_eq!(actual, expected);

        let fs_str = "crypto_LUKS";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::LUKS;
        assert_eq!(actual, expected);

        let fs_str = "LVM1_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::LVM1;
        assert_eq!(actual, expected);

        let fs_str = "LVM2_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::LVM2;
        assert_eq!(actual, expected);

        let fs_str = "minix";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Minix;
        assert_eq!(actual, expected);

        let fs_str = "mpool";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Mpool;
        assert_eq!(actual, expected);

        let fs_str = "msdos";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::MSDOS;
        assert_eq!(actual, expected);

        let fs_str = "nss";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Netware;
        assert_eq!(actual, expected);

        let fs_str = "nilfs2";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Nilfs2;
        assert_eq!(actual, expected);

        let fs_str = "ntfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::NTFS;
        assert_eq!(actual, expected);

        let fs_str = "nvidia_raid_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::NvidiaRaid;
        assert_eq!(actual, expected);

        let fs_str = "ocfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::OCFS;
        assert_eq!(actual, expected);

        let fs_str = "ocfs2";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::OCFS2;
        assert_eq!(actual, expected);

        let fs_str = "promise_fasttrack_raid_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::PromiseRaid;
        assert_eq!(actual, expected);

        let fs_str = "ReFs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::ReFs;
        assert_eq!(actual, expected);

        let fs_str = "reiserfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Reiserfs;
        assert_eq!(actual, expected);

        let fs_str = "reiser4";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Reiser4;
        assert_eq!(actual, expected);

        let fs_str = "romfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Romfs;
        assert_eq!(actual, expected);

        let fs_str = "silicon_medley_raid_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::SiliconRaid;
        assert_eq!(actual, expected);

        let fs_str = "squashfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Squashfs;
        assert_eq!(actual, expected);

        let fs_str = "squashfs3";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Squashfs3;
        assert_eq!(actual, expected);

        let fs_str = "stratis";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Stratis;
        assert_eq!(actual, expected);

        let fs_str = "swap";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Swap;
        assert_eq!(actual, expected);

        let fs_str = "swsuspend";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::SwapSuspend;
        assert_eq!(actual, expected);

        let fs_str = "sysv";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::SYSV;
        assert_eq!(actual, expected);

        let fs_str = "ubi";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::UBI;
        assert_eq!(actual, expected);

        let fs_str = "ubifs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::UBIFS;
        assert_eq!(actual, expected);

        let fs_str = "udf";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::UDF;
        assert_eq!(actual, expected);

        let fs_str = "ufs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::UFS;
        assert_eq!(actual, expected);

        let fs_str = "vdo";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::VDO;
        assert_eq!(actual, expected);

        let fs_str = "vfat";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::VFAT;
        assert_eq!(actual, expected);

        let fs_str = "via_raid_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::VIARaid;
        assert_eq!(actual, expected);

        let fs_str = "VMFS";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::VMFS;
        assert_eq!(actual, expected);

        let fs_str = "VMFS_volume_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::VMFSVolume;
        assert_eq!(actual, expected);

        let fs_str = "vxfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Vxfs;
        assert_eq!(actual, expected);

        let fs_str = "xenix";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::Xenix;
        assert_eq!(actual, expected);

        let fs_str = "xfs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::XFS;
        assert_eq!(actual, expected);

        let fs_str = "xfs_external_log";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::XFSLog;
        assert_eq!(actual, expected);

        let fs_str = "zfs_member";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::ZFS;
        assert_eq!(actual, expected);

        let fs_str = "zonefs";
        let actual: FileSystem = fs_str.parse()?;
        let expected = FileSystem::ZoneFS;
        assert_eq!(actual, expected);

        Ok(())
    }
}
