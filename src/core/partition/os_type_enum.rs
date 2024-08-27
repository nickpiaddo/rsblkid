// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use enum_iterator::Sequence;
use num_enum::{IntoPrimitive, TryFromPrimitive};

// From standard library
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// Supported MBR partitions.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Sequence, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u8)]
#[non_exhaustive]
pub enum OSType {
    /// Empty partition entry.
    EmptyPartition = 0x00,

    /// XENIX root.
    FAT12 = 0x01,

    /// XENIX root.
    XenixRoot = 0x02,

    /// XENIX usr.
    XenixUser = 0x03,

    /// FAT16 with less than 65,536 sectors (32 MB).
    FAT16 = 0x04,

    /// Extended partition with CHS addressing.
    ExtendedPartition = 0x05,

    /// FAT16B with 65,536 or more sectors.
    FAT16B = 0x06,

    /// HPFS / NTFS / exFAT.
    HPFSNTFSExfat = 0x07,

    /// AIX boot/split.
    AIX = 0x08,

    /// AIX data/boot.
    AIXBootable = 0x09,

    /// OS/2 Boot Manager
    OS2BootManager = 0x0a,

    /// FAT32 with CHS addressing.
    W95FAT32 = 0x0b,

    /// FAT32 with LBA.
    W95FAT32LBA = 0x0c,

    /// FAT16B with LBA.
    W95FAT16LBA = 0x0e,

    /// Extended partition with LBA.
    W95ExtendedLBA = 0x0f,

    /// OPUS.
    OPUS = 0x10,

    /// Hidden FAT12.
    HiddenFAT12 = 0x11,

    /// Diagnostics and firmware partition (bootable FAT).
    CompaqDiagnostics = 0x12,

    /// Hidden FAT16.
    HiddenFAT16 = 0x14,

    /// Hidden FAT16B.
    HiddenFAT16B = 0x16,

    /// Hidden HPFS / NTFS / exFAT.
    HiddenHPFSNTFSExFat = 0x17,

    /// AST SmartSleep partition.
    ASTSmartSleep = 0x18,

    /// Hidden FAT32 with CHS addressing.
    HiddenW95FAT32 = 0x1b,

    /// Hidden FAT32 with LBA.
    HiddenW95FAT32LBA = 0x1c,

    /// Hidden FAT16B with LBA.
    HiddenW95FAT16LBA = 0x1e,

    /// NEC MS-DOS 3.30 Logical sectored FAT12 or FAT16.
    NecDOS = 0x24,

    /// Hidden NTFS rescue partition.
    HiddenNTFSRescue = 0x27,

    /// Plan 9 edition 3 partition.
    Plan9 = 0x39,

    /// PartitionMagic recovery partition.
    PartitionMagic = 0x3c,

    /// Venix 80286.
    Venix80286 = 0x40,

    /// PPC PReP (Power PC Reference Platform) Boot.
    PPCPrepBoot = 0x41,

    /// Secure File system (SFS).
    Sfs = 0x42,

    /// Primary QNX POSIX volume on disk .
    QNX4Primary = 0x4d,

    /// Secondary QNX POSIX volume on disk.
    QNX4Secondary = 0x4e,

    /// Tertiary QNX POSIX volume on disk.
    QNX4Tertiary = 0x4f,

    /// OnTrack Disk Manager 4 read-only partition.
    OnTrackDM = 0x50,

    /// OnTrack Disk Manager 4-6 read-write partition (Aux 1).
    OnTrackDM6Aux1 = 0x51,

    /// CP/M-80.
    CPM80 = 0x52,

    /// Disk Manager 6 Auxiliary 3 (WO).
    OnTrackDM6Aux3 = 0x53,

    /// Disk Manager 6 Dynamic Drive Overlay (DDO).
    OnTrackDM6Ddo = 0x54,

    /// EZ-Drive.
    EZDrive = 0x55,

    /// Golden Bow VFeature Partitioned Volume.
    GoldenBow = 0x56,

    /// Priam EDisk Partitioned Volume.
    PriamEDisk = 0x5c,

    /// SpeedStor Hidden FAT12.
    SpeedStor = 0x61,

    /// Unix System V (SCO, ISC Unix, UnixWare, ...), Mach, GNU Hurd.
    GNUHurdSystemV = 0x63,

    /// Novell Netware 286, 2.xx
    NovellNetware286 = 0x64,

    /// Novell Netware 386, 3.xx or 4.xx
    NovellNetware386 = 0x65,

    /// DiskSecure multiboot.
    DiskSecureMultiBoot = 0x70,

    /// PC/IX.
    PCIX = 0x75,

    /// Minix 1.1-1.4a MINIX file system (old).
    OldMinix = 0x80,

    /// Minix 1.4b+ MINIX file system.
    MinixOldLinux = 0x81,

    /// Linux SWAP space.
    LinuxSwap = 0x82,

    /// Native Linux file system.
    Linux = 0x83,

    /// OS/2 hidden C: drive.
    OS2HiddenCDrive = 0x84,

    /// Linux extended partition.
    LinuxExtended = 0x85,

    /// Fault-tolerant FAT16B mirrored volume set.
    FAT16VolumeSet = 0x86,

    /// Fault-tolerant HPFS/NTFS mirrored volume set.
    NTFSVolumeSet = 0x87,

    /// Linux plain text partition table .
    LinuxPlaintext = 0x88,

    /// Linux Logical Volume Manager partition.
    LinuxLVM = 0x8e,

    /// Amoeba native file system.
    Amoeba = 0x93,

    /// Amoeba bad block table.
    AmoebaBadBlockTable = 0x94,

    /// BSD/OS 3.0+, BSDI.
    BSDOs = 0x9f,

    /// IBM Thinkpad Laptop hibernation partition.
    IBMThinkpad = 0xa0,

    /// FreeBSD.
    FreeBSD = 0xa5,

    /// OpenBSD.
    OpenBSD = 0xa6,

    /// NeXTSTEP.
    NextStep = 0xa7,

    /// Apple Darwin, Mac OS X UFS.
    DarwinUFS = 0xa8,

    /// NetBSD slice.
    NetBSD = 0xa9,

    /// Apple Darwin, Mac OS X boot.
    DarwinBoot = 0xab,

    /// HFS and HFS+
    HFSHFSPlus = 0xaf,

    /// BSDI native file system.
    BSDIFs = 0xb7,

    /// BSDI native swap.
    BSDISwap = 0xb8,

    /// PTS BootWizard 4 / OS Selector 5 for hidden partitions.
    BootWizardHidden = 0xbb,

    /// Acronis backup partition (Acronis Secure Zone).
    AcronisFAT32LBA = 0xbc,

    /// Solaris 8 boot partition.
    SolarisBoot = 0xbe,

    /// New Solaris x86 partition.
    Solaris = 0xbf,

    /// DR DOS 6.0+ Secured FAT12.
    DRDOSSecuredFAT12 = 0xc1,

    /// DR DOS 6.0+ Secured FAT16.
    DRDOSSecuredFAT16 = 0xc4,

    /// DR DOS 6.0+ Secured FAT16B.
    DRDOSSecuredFAT16B = 0xc6,

    /// Syrinx boot.
    Syrinx = 0xc7,

    /// Non-file system data.
    NonFsData = 0xda,

    /// Digital Research CP/M, Concurrent CP/M, Concurrent DOS.
    CPMCtOs = 0xdb,

    /// Dell PowerEdge Server utilities (FAT16).
    DellUtilityFAT16 = 0xde,

    /// BootIt EMBRM.
    BootIt = 0xdf,

    /// DOS access or SpeedStor 12-bit FAT extended partition.
    DOSAccess = 0xe1,

    /// SpeedStor Read-only FAT12.
    DOSRO = 0xe3,

    /// SpeedStor 16-bit FAT extended partition < 1024 cylinders.
    SpeedStorFAT16 = 0xe4,

    /// Freedesktop boot.
    FreedesktopBoot = 0xea,

    /// BeOS, Haiku BFS.
    BeOSBFS = 0xeb,

    /// GPT protective MBR (indication that this legacy MBR is followed by an EFI header).
    GPTProtectiveMBR = 0xee,

    /// EFI system partition. Can be a FAT12, FAT16, FAT32 (or other) file system.
    EfiSystem = 0xef,

    /// PA-RISC Linux boot loader.
    PARISCLinux = 0xf0,

    /// Storage Dimensions SpeedStor.
    SDSpeedstor = 0xf1,

    /// SpeedStor large partition.
    SpeedStorFAT16B = 0xf4,

    /// DOS 3.3+ secondary partition.
    DOSSecondary = 0xf2,

    /// Arm EBBR 1.0 Protective partition for the area containing system firmware.
    EBBRProtective = 0xf8,

    /// VMware ESX VMware VMFS file system partition.
    VMWareVMFS = 0xfb,

    /// VMware ESX VMware swap / VMKCORE kernel dump partition.
    VMWareVMKCORE = 0xfc,

    /// Linux RAID superblock with auto-detect.
    LinuxRaidAuto = 0xfd,

    /// LANstep.
    LanStep = 0xfe,

    ///  Xenix Bad Block Table.
    XenixBadBlockTable = 0xff,
}

impl OSType {
    /// View this `OSType` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        match self {
            Self::EmptyPartition => "0x00",
            Self::FAT12 => "0x01",
            Self::XenixRoot => "0x02",
            Self::XenixUser => "0x03",
            Self::FAT16 => "0x04",
            Self::ExtendedPartition => "0x05",
            Self::FAT16B => "0x06",
            Self::HPFSNTFSExfat => "0x07",
            Self::AIX => "0x08",
            Self::AIXBootable => "0x09",
            Self::OS2BootManager => "0x0a",
            Self::W95FAT32 => "0x0b",
            Self::W95FAT32LBA => "0x0c",
            Self::W95FAT16LBA => "0x0e",
            Self::W95ExtendedLBA => "0x0f",
            Self::OPUS => "0x10",
            Self::HiddenFAT12 => "0x11",
            Self::CompaqDiagnostics => "0x12",
            Self::HiddenFAT16 => "0x14",
            Self::HiddenFAT16B => "0x16",
            Self::HiddenHPFSNTFSExFat => "0x17",
            Self::ASTSmartSleep => "0x18",
            Self::HiddenW95FAT32 => "0x1b",
            Self::HiddenW95FAT32LBA => "0x1c",
            Self::HiddenW95FAT16LBA => "0x1e",
            Self::NecDOS => "0x24",
            Self::HiddenNTFSRescue => "0x27",
            Self::Plan9 => "0x39",
            Self::PartitionMagic => "0x3c",
            Self::Venix80286 => "0x40",
            Self::PPCPrepBoot => "0x41",
            Self::Sfs => "0x42",
            Self::QNX4Primary => "0x4d",
            Self::QNX4Secondary => "0x4e",
            Self::QNX4Tertiary => "0x4f",
            Self::OnTrackDM => "0x50",
            Self::OnTrackDM6Aux1 => "0x51",
            Self::CPM80 => "0x52",
            Self::OnTrackDM6Aux3 => "0x53",
            Self::OnTrackDM6Ddo => "0x54",
            Self::EZDrive => "0x55",
            Self::GoldenBow => "0x56",
            Self::PriamEDisk => "0x5c",
            Self::SpeedStor => "0x61",
            Self::GNUHurdSystemV => "0x63",
            Self::NovellNetware286 => "0x64",
            Self::NovellNetware386 => "0x65",
            Self::DiskSecureMultiBoot => "0x70",
            Self::PCIX => "0x75",
            Self::OldMinix => "0x80",
            Self::MinixOldLinux => "0x81",
            Self::LinuxSwap => "0x82",
            Self::Linux => "0x83",
            Self::OS2HiddenCDrive => "0x84",
            Self::LinuxExtended => "0x85",
            Self::FAT16VolumeSet => "0x86",
            Self::NTFSVolumeSet => "0x87",
            Self::LinuxPlaintext => "0x88",
            Self::LinuxLVM => "0x8e",
            Self::Amoeba => "0x93",
            Self::AmoebaBadBlockTable => "0x94",
            Self::BSDOs => "0x9f",
            Self::IBMThinkpad => "0xa0",
            Self::FreeBSD => "0xa5",
            Self::OpenBSD => "0xa6",
            Self::NextStep => "0xa7",
            Self::DarwinUFS => "0xa8",
            Self::NetBSD => "0xa9",
            Self::DarwinBoot => "0xab",
            Self::HFSHFSPlus => "0xaf",
            Self::BSDIFs => "0xb7",
            Self::BSDISwap => "0xb8",
            Self::BootWizardHidden => "0xbb",
            Self::AcronisFAT32LBA => "0xbc",
            Self::SolarisBoot => "0xbe",
            Self::Solaris => "0xbf",
            Self::DRDOSSecuredFAT12 => "0xc1",
            Self::DRDOSSecuredFAT16 => "0xc4",
            Self::DRDOSSecuredFAT16B => "0xc6",
            Self::Syrinx => "0xc7",
            Self::NonFsData => "0xda",
            Self::CPMCtOs => "0xdb",
            Self::DellUtilityFAT16 => "0xde",
            Self::BootIt => "0xdf",
            Self::DOSAccess => "0xe1",
            Self::DOSRO => "0xe3",
            Self::SpeedStorFAT16 => "0xe4",
            Self::FreedesktopBoot => "0xea",
            Self::BeOSBFS => "0xeb",
            Self::GPTProtectiveMBR => "0xee",
            Self::EfiSystem => "0xef",
            Self::PARISCLinux => "0xf0",
            Self::SDSpeedstor => "0xf1",
            Self::SpeedStorFAT16B => "0xf4",
            Self::DOSSecondary => "0xf2",
            Self::EBBRProtective => "0xf8",
            Self::VMWareVMFS => "0xfb",
            Self::VMWareVMKCORE => "0xfc",
            Self::LinuxRaidAuto => "0xfd",
            Self::LanStep => "0xfe",
            Self::XenixBadBlockTable => "0xff",
        }
    }
}

impl AsRef<OSType> for OSType {
    #[inline]
    fn as_ref(&self) -> &OSType {
        self
    }
}

impl AsRef<str> for OSType {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for OSType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&[u8]> for OSType {
    type Error = ConversionError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(bytes)
            .map_err(|e| {
                ConversionError::OSType(format!(
                    "bytes to UTF-8 string slice conversion error. {:?}",
                    e
                ))
            })
            .and_then(|s| Self::from_str(s).map_err(|e| ConversionError::OSType(e.to_string())))
    }
}

impl TryFrom<Vec<u8>> for OSType {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl FromStr for OSType {
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
                .ok_or(ParserError::OSType(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::OSType(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        // Remove hex string prefix and convert to `OSType`.
        stripped
            .trim()
            .strip_prefix("0x")
            .ok_or(ParserError::OSType(format!(
                "missing '0x' prefix in: {}",
                s
            )))
            .and_then(|h| {
                u8::from_str_radix(h, 16).map_err(|e| {
                    let err_msg = format!("invalid hexadecimal string: {} {:?}", s, e);

                    ParserError::OSType(err_msg)
                })
            })
            .and_then(|n| {
                Self::try_from(n).map_err(|_| {
                    let err_msg = format!("unsupported OS type: {}", s);

                    ParserError::OSType(err_msg)
                })
            })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn os_type_can_not_parse_an_os_type_string_with_an_unclosed_double_quote() {
        let _: OSType = r#""0x82"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn os_type_can_not_parse_an_os_type_string_with_an_unclosed_quote() {
        let _: OSType = "'0x82".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing '0x' prefix")]
    fn os_type_can_not_parse_an_empty_string() {
        let _: OSType = "".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing '0x' prefix")]
    fn os_type_can_not_parse_an_os_type_missing_its_0x_prefix() {
        let _: OSType = "82".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid hexadecimal string")]
    fn os_type_can_not_parse_an_os_type_string_with_an_invalid_hexadecimal() {
        let _: OSType = "0xDUMMY".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid hexadecimal string")]
    fn os_type_can_not_parse_an_os_type_string_with_a_too_large_hexadecimal() {
        let _: OSType = "0xffffff".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string slice conversion error")]
    fn os_type_can_not_convert_invalid_bytes_into_an_os_type() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = OSType::try_from(bytes).unwrap();
    }

    #[test]
    fn os_type_can_convert_valid_bytes_into_an_os_type() -> crate::Result<()> {
        let bytes: Vec<u8> = b"0x83".to_vec();
        let actual = OSType::try_from(bytes)?;
        let expected = OSType::Linux;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn os_type_can_parse_a_valid_device_os_type() -> crate::Result<()> {
        let os_type_str = "0x00";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::EmptyPartition;
        assert_eq!(actual, expected);

        let os_type_str = "0x01";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::FAT12;
        assert_eq!(actual, expected);

        let os_type_str = "0x02";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::XenixRoot;
        assert_eq!(actual, expected);

        let os_type_str = "0x03";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::XenixUser;
        assert_eq!(actual, expected);

        let os_type_str = "0x04";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::FAT16;
        assert_eq!(actual, expected);

        let os_type_str = "0x05";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::ExtendedPartition;
        assert_eq!(actual, expected);

        let os_type_str = "0x06";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::FAT16B;
        assert_eq!(actual, expected);

        let os_type_str = "0x07";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::HPFSNTFSExfat;
        assert_eq!(actual, expected);

        let os_type_str = "0x08";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::AIX;
        assert_eq!(actual, expected);

        let os_type_str = "0x09";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::AIXBootable;
        assert_eq!(actual, expected);

        let os_type_str = "0x0a";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::OS2BootManager;
        assert_eq!(actual, expected);

        let os_type_str = "0x0b";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::W95FAT32;
        assert_eq!(actual, expected);

        let os_type_str = "0x0c";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::W95FAT32LBA;
        assert_eq!(actual, expected);

        let os_type_str = "0x0e";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::W95FAT16LBA;
        assert_eq!(actual, expected);

        let os_type_str = "0x0f";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::W95ExtendedLBA;
        assert_eq!(actual, expected);

        let os_type_str = "0x10";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::OPUS;
        assert_eq!(actual, expected);

        let os_type_str = "0x11";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::HiddenFAT12;
        assert_eq!(actual, expected);

        let os_type_str = "0x12";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::CompaqDiagnostics;
        assert_eq!(actual, expected);

        let os_type_str = "0x14";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::HiddenFAT16;
        assert_eq!(actual, expected);

        let os_type_str = "0x16";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::HiddenFAT16B;
        assert_eq!(actual, expected);

        let os_type_str = "0x17";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::HiddenHPFSNTFSExFat;
        assert_eq!(actual, expected);

        let os_type_str = "0x18";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::ASTSmartSleep;
        assert_eq!(actual, expected);

        let os_type_str = "0x1b";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::HiddenW95FAT32;
        assert_eq!(actual, expected);

        let os_type_str = "0x1c";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::HiddenW95FAT32LBA;
        assert_eq!(actual, expected);

        let os_type_str = "0x1e";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::HiddenW95FAT16LBA;
        assert_eq!(actual, expected);

        let os_type_str = "0x24";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::NecDOS;
        assert_eq!(actual, expected);

        let os_type_str = "0x27";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::HiddenNTFSRescue;
        assert_eq!(actual, expected);

        let os_type_str = "0x39";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::Plan9;
        assert_eq!(actual, expected);

        let os_type_str = "0x3c";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::PartitionMagic;
        assert_eq!(actual, expected);

        let os_type_str = "0x40";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::Venix80286;
        assert_eq!(actual, expected);

        let os_type_str = "0x41";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::PPCPrepBoot;
        assert_eq!(actual, expected);

        let os_type_str = "0x42";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::Sfs;
        assert_eq!(actual, expected);

        let os_type_str = "0x4d";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::QNX4Primary;
        assert_eq!(actual, expected);

        let os_type_str = "0x4e";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::QNX4Secondary;
        assert_eq!(actual, expected);

        let os_type_str = "0x4f";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::QNX4Tertiary;
        assert_eq!(actual, expected);

        let os_type_str = "0x50";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::OnTrackDM;
        assert_eq!(actual, expected);

        let os_type_str = "0x51";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::OnTrackDM6Aux1;
        assert_eq!(actual, expected);

        let os_type_str = "0x52";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::CPM80;
        assert_eq!(actual, expected);

        let os_type_str = "0x53";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::OnTrackDM6Aux3;
        assert_eq!(actual, expected);

        let os_type_str = "0x54";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::OnTrackDM6Ddo;
        assert_eq!(actual, expected);

        let os_type_str = "0x55";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::EZDrive;
        assert_eq!(actual, expected);

        let os_type_str = "0x56";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::GoldenBow;
        assert_eq!(actual, expected);

        let os_type_str = "0x5c";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::PriamEDisk;
        assert_eq!(actual, expected);

        let os_type_str = "0x61";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::SpeedStor;
        assert_eq!(actual, expected);

        let os_type_str = "0x63";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::GNUHurdSystemV;
        assert_eq!(actual, expected);

        let os_type_str = "0x64";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::NovellNetware286;
        assert_eq!(actual, expected);

        let os_type_str = "0x65";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::NovellNetware386;
        assert_eq!(actual, expected);

        let os_type_str = "0x70";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::DiskSecureMultiBoot;
        assert_eq!(actual, expected);

        let os_type_str = "0x75";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::PCIX;
        assert_eq!(actual, expected);

        let os_type_str = "0x80";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::OldMinix;
        assert_eq!(actual, expected);

        let os_type_str = "0x81";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::MinixOldLinux;
        assert_eq!(actual, expected);

        let os_type_str = "0x82";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::LinuxSwap;
        assert_eq!(actual, expected);

        let os_type_str = "0x83";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::Linux;
        assert_eq!(actual, expected);

        let os_type_str = "0x84";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::OS2HiddenCDrive;
        assert_eq!(actual, expected);

        let os_type_str = "0x85";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::LinuxExtended;
        assert_eq!(actual, expected);

        let os_type_str = "0x86";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::FAT16VolumeSet;
        assert_eq!(actual, expected);

        let os_type_str = "0x87";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::NTFSVolumeSet;
        assert_eq!(actual, expected);

        let os_type_str = "0x88";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::LinuxPlaintext;
        assert_eq!(actual, expected);

        let os_type_str = "0x8e";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::LinuxLVM;
        assert_eq!(actual, expected);

        let os_type_str = "0x93";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::Amoeba;
        assert_eq!(actual, expected);

        let os_type_str = "0x94";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::AmoebaBadBlockTable;
        assert_eq!(actual, expected);

        let os_type_str = "0x9f";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::BSDOs;
        assert_eq!(actual, expected);

        let os_type_str = "0xa0";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::IBMThinkpad;
        assert_eq!(actual, expected);

        let os_type_str = "0xa5";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::FreeBSD;
        assert_eq!(actual, expected);

        let os_type_str = "0xa6";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::OpenBSD;
        assert_eq!(actual, expected);

        let os_type_str = "0xa7";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::NextStep;
        assert_eq!(actual, expected);

        let os_type_str = "0xa8";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::DarwinUFS;
        assert_eq!(actual, expected);

        let os_type_str = "0xa9";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::NetBSD;
        assert_eq!(actual, expected);

        let os_type_str = "0xab";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::DarwinBoot;
        assert_eq!(actual, expected);

        let os_type_str = "0xaf";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::HFSHFSPlus;
        assert_eq!(actual, expected);

        let os_type_str = "0xb7";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::BSDIFs;
        assert_eq!(actual, expected);

        let os_type_str = "0xb8";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::BSDISwap;
        assert_eq!(actual, expected);

        let os_type_str = "0xbb";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::BootWizardHidden;
        assert_eq!(actual, expected);

        let os_type_str = "0xbc";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::AcronisFAT32LBA;
        assert_eq!(actual, expected);

        let os_type_str = "0xbe";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::SolarisBoot;
        assert_eq!(actual, expected);

        let os_type_str = "0xbf";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::Solaris;
        assert_eq!(actual, expected);

        let os_type_str = "0xc1";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::DRDOSSecuredFAT12;
        assert_eq!(actual, expected);

        let os_type_str = "0xc4";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::DRDOSSecuredFAT16;
        assert_eq!(actual, expected);

        let os_type_str = "0xc6";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::DRDOSSecuredFAT16B;
        assert_eq!(actual, expected);

        let os_type_str = "0xc7";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::Syrinx;
        assert_eq!(actual, expected);

        let os_type_str = "0xda";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::NonFsData;
        assert_eq!(actual, expected);

        let os_type_str = "0xdb";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::CPMCtOs;
        assert_eq!(actual, expected);

        let os_type_str = "0xde";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::DellUtilityFAT16;
        assert_eq!(actual, expected);

        let os_type_str = "0xdf";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::BootIt;
        assert_eq!(actual, expected);

        let os_type_str = "0xe1";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::DOSAccess;
        assert_eq!(actual, expected);

        let os_type_str = "0xe3";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::DOSRO;
        assert_eq!(actual, expected);

        let os_type_str = "0xe4";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::SpeedStorFAT16;
        assert_eq!(actual, expected);

        let os_type_str = "0xea";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::FreedesktopBoot;
        assert_eq!(actual, expected);

        let os_type_str = "0xeb";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::BeOSBFS;
        assert_eq!(actual, expected);

        let os_type_str = "0xee";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::GPTProtectiveMBR;
        assert_eq!(actual, expected);

        let os_type_str = "0xef";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::EfiSystem;
        assert_eq!(actual, expected);

        let os_type_str = "0xf0";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::PARISCLinux;
        assert_eq!(actual, expected);

        let os_type_str = "0xf1";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::SDSpeedstor;
        assert_eq!(actual, expected);

        let os_type_str = "0xf4";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::SpeedStorFAT16B;
        assert_eq!(actual, expected);

        let os_type_str = "0xf2";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::DOSSecondary;
        assert_eq!(actual, expected);

        let os_type_str = "0xf8";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::EBBRProtective;
        assert_eq!(actual, expected);

        let os_type_str = "0xfb";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::VMWareVMFS;
        assert_eq!(actual, expected);

        let os_type_str = "0xfc";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::VMWareVMKCORE;
        assert_eq!(actual, expected);

        let os_type_str = "0xfd";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::LinuxRaidAuto;
        assert_eq!(actual, expected);

        let os_type_str = "0xfe";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::LanStep;
        assert_eq!(actual, expected);

        let os_type_str = "0xff";
        let actual: OSType = os_type_str.parse()?;
        let expected = OSType::XenixBadBlockTable;
        assert_eq!(actual, expected);

        Ok(())
    }
}
