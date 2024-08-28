// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use enum_iterator::Sequence;

// From standard library
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// Supported GPT partitions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Sequence)]
#[non_exhaustive]
pub enum Guid {
    /// Unused entry
    EmptyPartition,
    /// MBR partition scheme
    MBRPartition,
    /// EFI System partition or Fuchsia legacy uchsia-esp
    EfiSystem,
    /// BIOS boot partition
    BIOSBoot,
    /// Intel Fast Flash (iFFS) partition (for Intel Rapid Start technology)
    IntelFastFlash,
    /// Sony boot partition
    SonyBoot,
    /// Lenovo boot partition
    LenovoBoot,

    /// Windows Microsoft Reserved Partition (MSR)
    MicrosoftReserved,
    /// Windows Basic data partition
    WindowsBasicData,
    /// Windows Logical Disk Manager (LDM) metadata partition
    WindowsLDM,
    /// Windows Logical Disk Manager data partition
    WindowsLDMData,
    /// Windows Windows Recovery Environment
    WindowsRecovery,
    /// Windows IBM General Parallel File System (GPFS) partition
    IBMGPFS,
    /// Windows Storage Spaces partition
    WindowsStorageSpaces,
    /// Windows Storage Replica partition
    WindowsStorageReplica,

    /// HP-UX Data partition
    HPUXData,
    /// HP-UX Service partition
    HPUXService,

    /// Linux filesystem data
    LinuxData,
    /// Linux RAID partition
    LinuxRaid,
    /// Linux Root partition (Alpha)
    LinuxRootAlpha,
    /// Linux Root partition (ARC)
    LinuxRootARC,
    /// Linux Root partition (ARM 32‐bit)
    LinuxRootARM32,
    /// Linux Root partition (AArch64)
    LinuxRootARM64,
    /// Linux Root partition (IA-64)
    LinuxRootIA64,
    /// Linux Root partition (LoongArch 64‐bit)
    LinuxRootLoongArch64,
    /// Linux Root partition (mips: 32‐bit MIPS big‐endian)
    LinuxRootMIPS,
    /// Linux Root partition (mips64: 64‐bit MIPS big‐endian)
    LinuxRootMIPS64,
    /// Linux Root partition (mipsel: 32‐bit MIPS little‐endian)
    LinuxRootMIPSEL,
    /// Linux Root partition (mips64el: 64‐bit MIPS little‐endian)
    LinuxRootMIPS64EL,
    /// Linux Root partition (PA-RISC)
    LinuxRootPARISC,
    /// Linux Root partition (32‐bit PowerPC)
    LinuxRootPPC32,
    /// Linux Root partition (64‐bit PowerPC big‐endian)
    LinuxRootPPC64,
    /// Linux Root partition (64‐bit PowerPC little‐endian)
    LinuxRootPPC64LE,
    /// Linux Root partition (RISC-V 32‐bit)
    LinuxRootRISCV32,
    /// Linux Root partition (RISC-V 64‐bit)
    LinuxRootRISCV64,
    /// Linux Root partition (s390)
    LinuxRootS390,
    /// Linux Root partition (s390x)
    LinuxRootS390x,
    /// Linux Root partition (TILE-Gx)
    LinuxRootTILEGx,
    /// Linux Root partition (x86)
    LinuxRootx86,
    /// Linux Root partition (x86-64)
    LinuxRootx86_64,

    /// Linux /usr partition (Alpha)
    LinuxUsrAlpha,
    /// Linux /usr partition (ARC)
    LinuxUsrARC,
    /// Linux /usr partition (ARM 32‐bit)
    LinuxUsrARM32,
    /// Linux /usr partition (AArch64)
    LinuxUsrARM64,
    /// Linux /usr partition (IA-64)
    LinuxUsrIA64,
    /// Linux /usr partition (LoongArch 64‐bit)
    LinuxUsrLoongArch64,
    /// Linux /usr partition (mips: 32‐bit MIPS big‐endian)
    LinuxUsrMIPS,
    /// Linux /usr partition (mips64: 64‐bit MIPS big‐endian)
    LinuxUsrMIPS64,
    /// Linux /usr partition (mipsel: 32‐bit MIPS little‐endian)
    LinuxUsrMIPSEL,
    /// Linux /usr partition (mips64el: 64‐bit MIPS little‐endian)
    LinuxUsrMIPS64EL,
    /// Linux /usr partition (PA-RISC)
    LinuxUsrPARISC,
    /// Linux /usr partition (32‐bit PowerPC)
    LinuxUsrPPC32,
    /// Linux /usr partition (64‐bit PowerPC big‐endian)
    LinuxUsrPPC64,
    /// Linux /usr partition (64‐bit PowerPC little‐endian)
    LinuxUsrPPC64LE,
    /// Linux /usr partition (RISC-V 32‐bit)
    LinuxUsrRISCV32,
    /// Linux /usr partition (RISC-V 64‐bit)
    LinuxUsrRISCV64,
    /// Linux /usr partition (s390)
    LinuxUsrS390,
    /// Linux /usr partition (s390x)
    LinuxUsrS390x,
    /// Linux /usr partition (TILE-Gx)
    LinuxUsrTILEGx,
    /// Linux /usr partition (x86)
    LinuxUsrx86,
    /// Linux /usr partition (x86-64)
    LinuxUsrx86_64,

    /// Linux Root verity partition for dm-verity (Alpha)
    LinuxRootVerityAlpha,
    /// Linux Root verity partition for dm-verity (ARC)
    LinuxRootVerityARC,
    /// Linux Root verity partition for dm-verity (ARM 32‐bit)
    LinuxRootVerityARM32,
    /// Linux Root verity partition for dm-verity (AArch64)
    LinuxRootVerityARM64,
    /// Linux Root verity partition for dm-verity (IA-64)
    LinuxRootVerityIA64,
    /// Linux Root verity partition for dm-verity (LoongArch 64‐bit)
    LinuxRootVerityLoongArch64,
    /// Linux Root verity partition for dm-verity (mips: 32‐bit MIPS big‐endian)
    LinuxRootVerityMIPS,
    /// Linux Root verity partition for dm-verity (mips64: 64‐bit MIPS big‐endian)
    LinuxRootVerityMIPS64,
    /// Linux Root verity partition for dm-verity (mipsel: 32‐bit MIPS little‐endian)
    LinuxRootVerityMIPSEL,
    /// Linux Root verity partition for dm-verity (mips64el: 64‐bit MIPS little‐endian)
    LinuxRootVerityMIPS64EL,
    /// Linux Root verity partition for dm-verity (PA-RISC)
    LinuxRootVerityPARISC,
    /// Linux Root verity partition for dm-verity (32‐bit PowerPC)
    LinuxRootVerityPPC32,
    /// Linux Root verity partition for dm-verity (64‐bit PowerPC big‐endian)
    LinuxRootVerityPPC64,
    /// Linux Root verity partition for dm-verity (64‐bit PowerPC little‐endian)
    LinuxRootVerityPPC64LE,
    /// Linux Root verity partition for dm-verity (RISC-V 32‐bit)
    LinuxRootVerityRISCV32,
    /// Linux Root verity partition for dm-verity (RISC-V 64‐bit)
    LinuxRootVerityRISCV64,
    /// Linux Root verity partition for dm-verity (s390)
    LinuxRootVerityS390,
    /// Linux Root verity partition for dm-verity (s390x)
    LinuxRootVerityS390x,
    /// Linux Root verity partition for dm-verity (TILE-Gx)
    LinuxRootVerityTILEGx,
    /// Linux Root verity partition for dm-verity (x86)
    LinuxRootVerityx86,
    /// Linux Root verity partition for dm-verity (x86-64)
    LinuxRootVerityx86_64,

    /// Linux /usr verity partition for dm-verity (Alpha)
    LinuxUsrVerityAlpha,
    /// Linux /usr verity partition for dm-verity (ARC)
    LinuxUsrVerityARC,
    /// Linux /usr verity partition for dm-verity (ARM 32‐bit)
    LinuxUsrVerityARM32,
    /// Linux /usr verity partition for dm-verity (AArch64)
    LinuxUsrVerityARM64,
    /// Linux /usr verity partition for dm-verity (IA-64)
    LinuxUsrVerityIA64,
    /// Linux /usr verity partition for dm-verity (LoongArch 64‐bit)
    LinuxUsrVerityLoongArch64,
    /// Linux /usr verity partition for dm-verity (mips: 32‐bit MIPS big‐endian)
    LinuxUsrVerityMIPS,
    /// Linux /usr verity partition for dm-verity (mips64: 64‐bit MIPS big‐endian)
    LinuxUsrVerityMIPS64,
    /// Linux /usr verity partition for dm-verity (mipsel: 32‐bit MIPS little‐endian)
    LinuxUsrVerityMIPSEL,
    /// Linux /usr verity partition for dm-verity (mips64el: 64‐bit MIPS little‐endian)
    LinuxUsrVerityMIPS64EL,
    /// Linux /usr verity partition for dm-verity (PA-RISC)
    LinuxUsrVerityPARISC,
    /// Linux /usr verity partition for dm-verity (32‐bit PowerPC)
    LinuxUsrVerityPPC32,
    /// Linux /usr verity partition for dm-verity (64‐bit PowerPC big‐endian)
    LinuxUsrVerityPPC64,
    /// Linux /usr verity partition for dm-verity (64‐bit PowerPC little‐endian)
    LinuxUsrVerityPPC64LE,
    /// Linux /usr verity partition for dm-verity (RISC-V 32‐bit)
    LinuxUsrVerityRISCV32,
    /// Linux /usr verity partition for dm-verity (RISC-V 64‐bit)
    LinuxUsrVerityRISCV64,
    /// Linux /usr verity partition for dm-verity (s390)
    LinuxUsrVerityS390,
    /// Linux /usr verity partition for dm-verity (s390x)
    LinuxUsrVerityS390x,
    /// Linux /usr verity partition for dm-verity (TILE-Gx)
    LinuxUsrVerityTILEGx,
    /// Linux /usr verity partition for dm-verity (x86)
    LinuxUsrVerityx86,
    /// Linux /usr verity partition for dm-verity (x86-64)
    LinuxUsrVerityx86_64,

    /// Linux Root verity signature partition for dm-verity (Alpha)
    LinuxRootVeritySigAlpha,
    /// Linux Root verity signature partition for dm-verity (ARC)
    LinuxRootVeritySigARC,
    /// Linux Root verity signature partition for dm-verity (ARM 32‐bit)
    LinuxRootVeritySigARM32,
    /// Linux Root verity signature partition for dm-verity (AArch64)
    LinuxRootVeritySigARM64,
    /// Linux Root verity signature partition for dm-verity (IA-64)
    LinuxRootVeritySigIA64,
    /// Linux Root verity signature partition for dm-verity (LoongArch 64‐bit)
    LinuxRootVeritySigLoongArch64,
    /// Linux Root verity signature partition for dm-verity (mips: 32‐bit MIPS big‐endian)
    LinuxRootVeritySigMIPS,
    /// Linux Root verity signature partition for dm-verity (mips64: 64‐bit MIPS big‐endian)
    LinuxRootVeritySigMIPS64,
    /// Linux Root verity signature partition for dm-verity (mipsel: 32‐bit MIPS little‐endian)
    LinuxRootVeritySigMIPSEL,
    /// Linux Root verity signature partition for dm-verity (mips64el: 64‐bit MIPS little‐endian)
    LinuxRootVeritySigMIPS64EL,
    /// Linux Root verity signature partition for dm-verity (PA-RISC)
    LinuxRootVeritySigPARISC,
    /// Linux Root verity signature partition for dm-verity (32‐bit PowerPC)
    LinuxRootVeritySigPPC32,
    /// Linux Root verity signature partition for dm-verity (64‐bit PowerPC big‐endian)
    LinuxRootVeritySigPPC64,
    /// Linux Root verity signature partition for dm-verity (64‐bit PowerPC little‐endian)
    LinuxRootVeritySigPPC64LE,
    /// Linux Root verity signature partition for dm-verity (RISC-V 32‐bit)
    LinuxRootVeritySigRISCV32,
    /// Linux Root verity signature partition for dm-verity (RISC-V 64‐bit)
    LinuxRootVeritySigRISCV64,
    /// Linux Root verity signature partition for dm-verity (s390)
    LinuxRootVeritySigS390,
    /// Linux Root verity signature partition for dm-verity (s390x)
    LinuxRootVeritySigS390x,
    /// Linux Root verity signature partition for dm-verity (TILE-Gx)
    LinuxRootVeritySigTILEGx,
    /// Linux Root verity signature partition for dm-verity (x86)
    LinuxRootVeritySigx86,
    /// Linux Root verity signature partition for dm-verity (x86-64)
    LinuxRootVeritySigx86_64,

    /// Linux /usr verity signature partition for dm-verity (Alpha)
    LinuxUsrVeritySigAlpha,
    /// Linux /usr verity signature partition for dm-verity (ARC)
    LinuxUsrVeritySigARC,
    /// Linux /usr verity signature partition for dm-verity (ARM 32‐bit)
    LinuxUsrVeritySigARM32,
    /// Linux /usr verity signature partition for dm-verity (AArch64)
    LinuxUsrVeritySigARM64,
    /// Linux /usr verity signature partition for dm-verity (IA-64)
    LinuxUsrVeritySigIA64,
    /// Linux /usr verity signature partition for dm-verity (LoongArch 64‐bit)
    LinuxUsrVeritySigLoongArch64,
    /// Linux /usr verity signature partition for dm-verity (mips: 32‐bit MIPS big‐endian)
    LinuxUsrVeritySigMIPS,
    /// Linux /usr verity signature partition for dm-verity (mips64: 64‐bit MIPS big‐endian)
    LinuxUsrVeritySigMIPS64,
    /// Linux /usr verity signature partition for dm-verity (mipsel: 32‐bit MIPS little‐endian)
    LinuxUsrVeritySigMIPSEL,
    /// Linux /usr verity signature partition for dm-verity (mips64el: 64‐bit MIPS little‐endian)
    LinuxUsrVeritySigMIPS64EL,
    /// Linux /usr verity signature partition for dm-verity (PA-RISC)
    LinuxUsrVeritySigPARISC,
    /// Linux /usr verity signature partition for dm-verity (32‐bit PowerPC)
    LinuxUsrVeritySigPPC32,
    /// Linux /usr verity signature partition for dm-verity (64‐bit PowerPC big‐endian)
    LinuxUsrVeritySigPPC64,
    /// Linux /usr verity signature partition for dm-verity (64‐bit PowerPC little‐endian)
    LinuxUsrVeritySigPPC64LE,
    /// Linux /usr verity signature partition for dm-verity (RISC-V 32‐bit)
    LinuxUsrVeritySigRISCV32,
    /// Linux /usr verity signature partition for dm-verity (RISC-V 64‐bit)
    LinuxUsrVeritySigRISCV64,
    /// Linux /usr verity signature partition for dm-verity (s390)
    LinuxUsrVeritySigS390,
    /// Linux /usr verity signature partition for dm-verity (s390x)
    LinuxUsrVeritySigS390x,
    /// Linux /usr verity signature partition for dm-verity (TILE-Gx)
    LinuxUsrVeritySigTILEGx,
    /// Linux /usr verity signature partition for dm-verity (x86)
    LinuxUsrVeritySigx86,
    /// Linux /usr verity signature partition for dm-verity (x86-64)
    LinuxUsrVeritySigx86_64,

    /// Linux /boot, as an Extended Boot Loader (XBOOTLDR) partition
    LinuxXBOOTLDR,
    /// Linux Swap partition
    LinuxSwap,
    /// Linux Logical Volume Manager (LVM) partition
    LinuxLVM,
    /// Linux /home partition
    LinuxHome,
    /// Linux /srv (server data) partition
    LinuxServerData,
    /// Linux Per‐user home partition
    LinuxPerUserHome,
    /// Linux Plain dm-crypt partition
    LinuxPlain,
    /// Linux LUKS partition
    LinuxLUKS,
    /// Linux Reserved
    LinuxReserved,

    // /// GNU-Hurd Linux filesystem data
    // GNUHurdData,
    // /// GNU-Hurd Linux Swap partition
    // GNUHurdSwap,
    /// FreeBSD Boot partition
    FreeBSDBoot,
    /// FreeBSD BSD disklabel partition
    FreeBSDDisklabel,
    /// FreeBSD Swap partition
    FreeBSDSwap,
    /// FreeBSD Unix File System (UFS) partition
    FreeBSDUFS,
    /// FreeBSD Vinum volume manager partition
    FreeBSDVinum,
    /// FreeBSD ZFS partition
    FreeBSDZFS,
    /// FreeBSD nandfs partition
    FreeBSDnandfs,

    /// macOS Darwin Hierarchical File System Plus (HFS+) partition
    MacOSHFSPlus,
    /// macOS Darwin Apple APFS container APFS FileVault volume container
    MacOSAPFS,
    /// macOS Darwin Apple UFS container
    MacOSUFS,
    // /// macOS Darwin ZFS
    // MacOSZFS,
    /// macOS Darwin Apple RAID partition
    MacOSRAID,
    /// macOS Darwin Apple RAID partition, offline
    MacOSRAIDOffline,
    /// macOS Darwin Apple Boot partition (Recovery HD)
    MacOSBootRecovery,
    /// macOS Darwin Apple Label
    MacOSLabel,
    /// macOS Darwin Apple TV Recovery partition
    MacOSAppleTVRecovery,
    /// macOS Darwin Apple Core Storage Container HFS+ FileVault volume container
    MacOSHFSPlusFileVault,
    /// macOS Darwin Apple APFS Preboot partition
    MacOSAPFSPreboot,
    /// macOS Darwin Apple APFS Recovery partition
    MacOSAPFSRecovery,

    /// Solaris/Illumos Boot partition
    SolarisBoot,
    /// Solaris/Illumos Root partition
    SolarisRoot,
    /// Solaris/Illumos Swap partition
    SolarisSwap,
    /// Solaris/Illumos Backup partition
    SolarisBackup,
    /// Solaris/Illumos /usr partition or MacOSZFS,
    SolarisUsr,
    /// Solaris/Illumos /var partition
    SolarisVar,
    /// Solaris/Illumos /home partition
    SolarisHome,
    /// Solaris/Illumos Alternate sector
    SolarisAlternateSector,
    /// Solaris/Illumos Reserved partition
    SolarisReserved1,
    /// Solaris/Illumos Reserved partition
    SolarisReserved2,
    /// Solaris/Illumos Reserved partition
    SolarisReserved3,
    /// Solaris/Illumos Reserved partition
    SolarisReserved4,
    /// Solaris/Illumos Reserved partition
    SolarisReserved5,

    /// NetBSD Swap partition
    NetBSDSwap,
    /// NetBSD FFS partition
    NetBSDFFS,
    /// NetBSD LFS partition
    NetBSDLFS,
    /// NetBSD RAID partition
    NetBSDRAID,
    /// NetBSD Concatenated partition
    NetBSDConcatenated,
    /// NetBSD Encrypted partition
    NetBSDEncrypted,

    /// ChromeOS kernel
    ChromeOSKernel,
    /// ChromeOS rootfs
    ChromeOSRootFs,
    /// ChromeOS firmware
    ChromeOSFirmware,
    /// ChromeOS future use
    ChromeOSFuture,
    /// ChromeOS miniOS
    ChromeOSMiniOS,
    /// ChromeOS hibernate
    ChromeOSHibernate,

    /// CoreOS /usr partition (coreos-usr)
    CoreOSUsr,
    /// CoreOS Resizable rootfs (coreos-resize)
    CoreOSResize,
    /// CoreOS OEM customizations (coreos-reserved)
    CoreOSReserved,
    /// CoreOS Root filesystem on RAID (coreos-root-raid)
    CoreOSRootRAID,

    ///  Haiku BFS
    HaikuBFS,

    /// MidnightBSD Boot partition
    MidnightBSDBoot,
    /// MidnightBSD Data partition
    MidnightBSDData,
    /// MidnightBSD Swap partition
    MidnightBSDSwap,
    /// MidnightBSD Unix File System (UFS) partition
    MidnightBSDUFS,
    /// MidnightBSD Vinum volume manager partition
    MidnightBSDVinum,
    /// MidnightBSD ZFS partition
    MidnightBSDZFS,

    /// Ceph Journal
    CephJournal,
    /// Ceph dm-crypt journal
    CephDMCryptJournal,
    /// Ceph OSD
    CephOSD,
    /// Ceph dm-crypt OSD
    CephDMCryptOSD,
    /// Ceph Disk in creation
    CephDisk,
    /// Ceph dm-crypt disk in creation
    CephDMCryptDisk,
    /// Ceph Block
    CephBlock,
    /// Ceph Block DB
    CephBlockDB,
    /// Ceph Block write-ahead log
    CephBlockLog,
    /// Ceph Lockbox for dm-crypt keys
    CephLockbox,
    /// Ceph Multipath OSD
    CephMultipathOSD,
    /// Ceph Multipath journal
    CephMultipathJournal,
    /// Ceph Multipath block
    CephMultipathBlock1,
    /// Ceph Multipath block
    CephMultipathBlock2,
    /// Ceph Multipath block DB
    CephMultipathBlockDB,
    /// Ceph Multipath block write-ahead log
    CephMultipathLog,
    /// Ceph dm-crypt block
    CephDMCryptBlock,
    /// Ceph dm-crypt block DB
    CephDMCryptBlockDB,
    /// Ceph dm-crypt block write-ahead log
    CephDMCryptBlockLog,
    /// Ceph dm-crypt LUKS OSD
    CephLUKSOSD,
    /// Ceph dm-crypt LUKS journal
    CephLUKSJournal,
    /// Ceph dm-crypt LUKS block
    CephLUKSBlock,
    /// Ceph dm-crypt LUKS block DB
    CephLUKSBlockDB,
    /// Ceph dm-crypt LUKS block write-ahead log
    CephLUKSBlockLog,

    /// OpenBSD Data partition
    OpenBSDData,

    /// QNX Power-safe (QNX6) file system
    QNX6Fs,

    /// Plan 9 partition
    Plan9,

    /// VMWare ESX vmkcore (coredump partition)
    VMWareVMKCORE,
    /// VMWare ESX VMFS filesystem partition
    VMWareVMFS,
    /// VMWare ESX VMware Reserved
    VMWareWmkReserved,

    /// Android-IA Bootloader
    AndroidBootloader,
    /// Android-IA Bootloader2
    AndroidBootloader2,
    /// Android-IA Boot
    AndroidBoot,
    /// Android-IA Recovery
    AndroidRecovery,
    /// Android-IA Misc
    AndroidMisc,
    /// Android-IA Metadata
    AndroidMetadata,
    /// Android-IA System
    AndroidSystem,
    /// Android-IA Cache
    AndroidCache,
    /// Android-IA Data
    AndroidData,
    /// Android-IA Persistent
    AndroidPersistent,
    /// Android-IA Vendor
    AndroidVendor,
    /// Android-IA Config
    AndroidConfig,
    /// Android-IA Factory
    AndroidFactory,
    /// Android-IA Factory (alt)
    AndroidFactoryAlt,
    /// Android-IA Fastboot / Tertiary
    AndroidFastboot,
    /// Android-IA OEM
    AndroidOEM,

    /// Android 6.0+ ARM Android Meta
    Android6Meta,
    /// Android 6.0+ ARM Android EXT
    Android6Ext,

    /// Open Network Install Environment (ONIE) Boot
    ONIEBoot,
    /// Open Network Install Environment (ONIE) Config
    ONIEConfig,

    /// PowerPC PReP boot
    PPCPrePBoot,

    // /// freedesktop.org OSes (Linux, etc.) Shared boot loader configuration
    // FreeDesktopConfig,
    /// Atari TOS Basic data partition (GEM, BGM, F32)
    AtariTOSBasicData,
    /// Atari TOS Raw data partition (RAW), XHDI
    AtariTOSRawData,

    /// VeraCrypt Encrypted data partition
    VeraCryptEncrypted,

    /// OS/2 ArcaOS Type 1
    OS2ArcaOS,

    /// Storage Performance Development Kit (SPDK) SPDK block device
    SPDK,

    /// barebox bootloader  barebox-state
    BareboxState,

    /// U-Boot bootloader  U-Boot environment
    UBootEnv,

    /// SoftRAID SoftRAID_Status
    SoftRAIDStatus,
    /// SoftRAID SoftRAID_Scratch
    SoftRAIDScratch,
    /// SoftRAID SoftRAID_Volume
    SoftRAIDVolume,
    /// SoftRAID SoftRAID_Cache
    SoftRAIDCache,

    /// Fuchsia Bootloader (slot A/B/R)
    FuchsiaBoot,
    /// Fuchsia Durable mutable encrypted system data
    FuchsiaSystemData,
    /// Fuchsia Durable mutable bootloader data (including A/B/R metadata)
    FuchsiaBootData,
    /// Fuchsia Factory-provisioned read-only system data
    FuchsiaFactorySystemData,
    /// Fuchsia Factory-provisioned read-only bootloader data
    FuchsiaFactoryBootData,
    /// Fuchsia Fuchsia Volume Manager
    FuchsiaVolumeManager,
    /// Fuchsia Verified boot metadata (slot A/B/R)
    FuchsiaVerifiedBoot,
    /// Fuchsia Zircon boot image (slot A/B/R)
    FuchsiaZirconBoot,

    // /// Fuchsia legacy uchsia-esp
    // FuchsiaLegacyESP,
    /// Fuchsia legacy fuchsia-system
    FuchsiaLegacySystem,
    /// Fuchsia legacy fuchsia-data
    FuchsiaLegacyData,
    /// Fuchsia legacy fuchsia-install
    FuchsiaLegacyInstall,
    /// Fuchsia legacy fuchsia-blob
    FuchsiaLegacyBlob,
    /// Fuchsia legacy fuchsia-fvm
    FuchsiaLegacyFVM,
    /// Fuchsia legacy Zircon boot image (slot A)
    FuchsiaLegacyZirconBootSlotA,
    /// Fuchsia legacy Zircon boot image (slot B)
    FuchsiaLegacyZirconBootSlotB,
    /// Fuchsia legacy Zircon boot image (slot R)
    FuchsiaLegacyZirconBootSlotR,
    /// Fuchsia legacy sys-config
    FuchsiaLegacySysConfig,
    /// Fuchsia legacy factory-config
    FuchsiaLegacyFactoryConfig,
    /// Fuchsia legacy bootloader
    FuchsiaLegacyBoot,
    /// Fuchsia legacy guid-test
    FuchsiaLegacyGuidTest,
    /// Fuchsia legacy Verified boot metadata (slot A)
    FuchsiaLegacyVerifiedBootSlotA,
    /// Fuchsia legacy Verified boot metadata (slot B)
    FuchsiaLegacyVerifiedBootSlotB,
    /// Fuchsia legacy Verified boot metadata (slot R)
    FuchsiaLegacyVerifiedBootSlotR,
    /// Fuchsia legacy misc
    FuchsiaLegacyMisc,
    /// Fuchsia legacy emmc-boot1
    FuchsiaLegacyEmmcBoot1,
    /// Fuchsia legacy emmc-boot2
    FuchsiaLegacyEmmcBoot2,

    /// Minix filesystem
    Minix,
}

impl Guid {
    /// View this `Guid` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        match self {
            Self::EmptyPartition => "00000000-0000-0000-0000-000000000000",
            Self::MBRPartition => "024dee41-33e7-11d3-9d69-0008c781f39f",
            Self::EfiSystem => "c12a7328-f81f-11d2-ba4b-00a0c93ec93b",
            Self::BIOSBoot => "21686148-6449-6e6f-744e-656564454649",
            Self::IntelFastFlash => "d3bfe2de-3daf-11df-ba40-e3a556d89593",
            Self::SonyBoot => "f4019732-066e-4e12-8273-346c5641494f",
            Self::LenovoBoot => "bfbfafe7-a34f-448a-9a5b-6213eb736c22",
            Self::MicrosoftReserved => "e3c9e316-0b5c-4db8-817d-f92df00215ae",
            Self::WindowsBasicData => "ebd0a0a2-b9e5-4433-87c0-68b6b72699c7",
            Self::WindowsLDM => "5808c8aa-7e8f-42e0-85d2-e1e90434cfb3",
            Self::WindowsLDMData => "af9b60a0-1431-4f62-bc68-3311714a69ad",
            Self::WindowsRecovery => "de94bba4-06d1-4d40-a16a-bfd50179d6ac",
            Self::IBMGPFS => "37affc90-ef7d-4e96-91c3-2d7ae055b174",
            Self::WindowsStorageSpaces => "e75caf8f-f680-4cee-afa3-b001e56efc2d",
            Self::WindowsStorageReplica => "558d43c5-a1ac-43c0-aac8-d1472b2923d1",
            Self::HPUXData => "75894c1e-3aeb-11d3-b7c1-7b03a0000000",
            Self::HPUXService => "e2a1e728-32e3-11d6-a682-7b03a0000000",
            Self::LinuxData => "0fc63daf-8483-4772-8e79-3d69d8477de4",
            Self::LinuxRaid => "a19d880f-05fc-4d3b-a006-743f0f84911e",
            Self::LinuxRootAlpha => "6523f8ae-3eb1-4e2a-a05a-18b695ae656f",
            Self::LinuxRootARC => "d27f46ed-2919-4cb8-bd25-9531f3c16534",
            Self::LinuxRootARM32 => "69dad710-2ce4-4e3c-b16c-21a1d49abed3",
            Self::LinuxRootARM64 => "b921b045-1df0-41c3-af44-4c6f280d3fae",
            Self::LinuxRootIA64 => "993d8d3d-f80e-4225-855a-9daf8ed7ea97",
            Self::LinuxRootLoongArch64 => "77055800-792c-4f94-b39a-98c91b762bb6",
            Self::LinuxRootMIPS => "e9434544-6e2c-47cc-bae2-12d6deafb44c",
            Self::LinuxRootMIPS64 => "d113af76-80ef-41b4-bdb6-0cff4d3d4a25",
            Self::LinuxRootMIPSEL => "37c58c8a-d913-4156-a25f-48b1b64e07f0",
            Self::LinuxRootMIPS64EL => "700bda43-7a34-4507-b179-eeb93d7a7ca3",
            Self::LinuxRootPARISC => "1aacdb3b-5444-4138-bd9e-e5c2239b2346",
            Self::LinuxRootPPC32 => "1de3f1ef-fa98-47b5-8dcd-4a860a654d78",
            Self::LinuxRootPPC64 => "912ade1d-a839-4913-8964-a10eee08fbd2",
            Self::LinuxRootPPC64LE => "c31c45e6-3f39-412e-80fb-4809c4980599",
            Self::LinuxRootRISCV32 => "60d5a7fe-8e7d-435c-b714-3dd8162144e1",
            Self::LinuxRootRISCV64 => "72ec70a6-cf74-40e6-bd49-4bda08e8f224",
            Self::LinuxRootS390 => "08a7acea-624c-4a20-91e8-6e0fa67d23f9",
            Self::LinuxRootS390x => "5eead9a9-fe09-4a1e-a1d7-520d00531306",
            Self::LinuxRootTILEGx => "c50cdd70-3862-4cc3-90e1-809a8c93ee2c",
            Self::LinuxRootx86 => "44479540-f297-41b2-9af7-d131d5f0458a",
            Self::LinuxRootx86_64 => "4f68bce3-e8cd-4db1-96e7-fbcaf984b709",
            Self::LinuxUsrAlpha => "e18cf08c-33ec-4c0d-8246-c6c6fb3da024",
            Self::LinuxUsrARC => "7978a683-6316-4922-bbee-38bff5a2fecc",
            Self::LinuxUsrARM32 => "7d0359a3-02b3-4f0a-865c-654403e70625",
            Self::LinuxUsrARM64 => "b0e01050-ee5f-4390-949a-9101b17104e9",
            Self::LinuxUsrIA64 => "4301d2a6-4e3b-4b2a-bb94-9e0b2c4225ea",
            Self::LinuxUsrLoongArch64 => "e611c702-575c-4cbe-9a46-434fa0bf7e3f",
            Self::LinuxUsrMIPS => "773b2abc-2a99-4398-8bf5-03baac40d02b",
            Self::LinuxUsrMIPS64 => "57e13958-7331-4365-8e6e-35eeee17c61b",
            Self::LinuxUsrMIPSEL => "0f4868e9-9952-4706-979f-3ed3a473e947",
            Self::LinuxUsrMIPS64EL => "c97c1f32-ba06-40b4-9f22-236061b08aa8",
            Self::LinuxUsrPARISC => "dc4a4480-6917-4262-a4ec-db9384949f25",
            Self::LinuxUsrPPC32 => "7d14fec5-cc71-415d-9d6c-06bf0b3c3eaf",
            Self::LinuxUsrPPC64 => "2c9739e2-f068-46b3-9fd0-01c5a9afbcca",
            Self::LinuxUsrPPC64LE => "15bb03af-77e7-4d4a-b12b-c0d084f7491c",
            Self::LinuxUsrRISCV32 => "b933fb22-5c3f-4f91-af90-e2bb0fa50702",
            Self::LinuxUsrRISCV64 => "beaec34b-8442-439b-a40b-984381ed097d",
            Self::LinuxUsrS390 => "cd0f869b-d0fb-4ca0-b141-9ea87cc78d66",
            Self::LinuxUsrS390x => "8a4f5770-50aa-4ed3-874a-99b710db6fea",
            Self::LinuxUsrTILEGx => "55497029-c7c1-44cc-aa39-815ed1558630",
            Self::LinuxUsrx86 => "75250d76-8cc6-458e-bd66-bd47cc81a812",
            Self::LinuxUsrx86_64 => "8484680c-9521-48c6-9c11-b0720656f69e",
            Self::LinuxRootVerityAlpha => "fc56d9e9-e6e5-4c06-be32-e74407ce09a5",
            Self::LinuxRootVerityARC => "24b2d975-0f97-4521-afa1-cd531e421b8d",
            Self::LinuxRootVerityARM32 => "7386cdf2-203c-47a9-a498-f2ecce45a2d6",
            Self::LinuxRootVerityARM64 => "df3300ce-d69f-4c92-978c-9bfb0f38d820",
            Self::LinuxRootVerityIA64 => "86ed10d5-b607-45bb-8957-d350f23d0571",
            Self::LinuxRootVerityLoongArch64 => "f3393b22-e9af-4613-a948-9d3bfbd0c535",
            Self::LinuxRootVerityMIPS => "7a430799-f711-4c7e-8e5b-1d685bd48607",
            Self::LinuxRootVerityMIPS64 => "579536f8-6a33-4055-a95a-df2d5e2c42a8",
            Self::LinuxRootVerityMIPSEL => "d7d150d2-2a04-4a33-8f12-16651205ff7b",
            Self::LinuxRootVerityMIPS64EL => "16b417f8-3e06-4f57-8dd2-9b5232f41aa6",
            Self::LinuxRootVerityPARISC => "d212a430-fbc5-49f9-a983-a7feef2b8d0e",
            Self::LinuxRootVerityPPC32 => "98cfe649-1588-46dc-b2f0-add147424925",
            Self::LinuxRootVerityPPC64 => "9225a9a3-3c19-4d89-b4f6-eeff88f17631",
            Self::LinuxRootVerityPPC64LE => "906bd944-4589-4aae-a4e4-dd983917446a",
            Self::LinuxRootVerityRISCV32 => "ae0253be-1167-4007-ac68-43926c14c5de",
            Self::LinuxRootVerityRISCV64 => "b6ed5582-440b-4209-b8da-5ff7c419ea3d",
            Self::LinuxRootVerityS390 => "7ac63b47-b25c-463b-8df8-b4a94e6c90e1",
            Self::LinuxRootVerityS390x => "b325bfbe-c7be-4ab8-8357-139e652d2f6b",
            Self::LinuxRootVerityTILEGx => "966061ec-28e4-4b2e-b4a5-1f0a825a1d84",
            Self::LinuxRootVerityx86 => "d13c5d3b-b5d1-422a-b29f-9454fdc89d76",
            Self::LinuxRootVerityx86_64 => "2c7357ed-ebd2-46d9-aec1-23d437ec2bf5",
            Self::LinuxUsrVerityAlpha => "8cce0d25-c0d0-4a44-bd87-46331bf1df67",
            Self::LinuxUsrVerityARC => "fca0598c-d880-4591-8c16-4eda05c7347c",
            Self::LinuxUsrVerityARM32 => "c215d751-7bcd-4649-be90-6627490a4c05",
            Self::LinuxUsrVerityARM64 => "6e11a4e7-fbca-4ded-b9e9-e1a512bb664e",
            Self::LinuxUsrVerityIA64 => "6a491e03-3be7-4545-8e38-83320e0ea880",
            Self::LinuxUsrVerityLoongArch64 => "f46b2c26-59ae-48f0-9106-c50ed47f673d",
            Self::LinuxUsrVerityMIPS => "6e5a1bc8-d223-49b7-bca8-37a5fcceb996",
            Self::LinuxUsrVerityMIPS64 => "81cf9d90-7458-4df4-8dcf-c8a3a404f09b",
            Self::LinuxUsrVerityMIPSEL => "46b98d8d-b55c-4e8f-aab3-37fca7f80752",
            Self::LinuxUsrVerityMIPS64EL => "3c3d61fe-b5f3-414d-bb71-8739a694a4ef",
            Self::LinuxUsrVerityPARISC => "5843d618-ec37-48d7-9f12-cea8e08768b2",
            Self::LinuxUsrVerityPPC32 => "df765d00-270e-49e5-bc75-f47bb2118b09",
            Self::LinuxUsrVerityPPC64 => "bdb528a5-a259-475f-a87d-da53fa736a07",
            Self::LinuxUsrVerityPPC64LE => "ee2b9983-21e8-4153-86d9-b6901a54d1ce",
            Self::LinuxUsrVerityRISCV32 => "cb1ee4e3-8cd0-4136-a0a4-aa61a32e8730",
            Self::LinuxUsrVerityRISCV64 => "8f1056be-9b05-47c4-81d6-be53128e5b54",
            Self::LinuxUsrVerityS390 => "b663c618-e7bc-4d6d-90aa-11b756bb1797",
            Self::LinuxUsrVerityS390x => "31741cc4-1a2a-4111-a581-e00b447d2d06",
            Self::LinuxUsrVerityTILEGx => "2fb4bf56-07fa-42da-8132-6b139f2026ae",
            Self::LinuxUsrVerityx86 => "8f461b0d-14ee-4e81-9aa9-049b6fb97abd",
            Self::LinuxUsrVerityx86_64 => "77ff5f63-e7b6-4633-acf4-1565b864c0e6",
            Self::LinuxRootVeritySigAlpha => "d46495b7-a053-414f-80f7-700c99921ef8",
            Self::LinuxRootVeritySigARC => "143a70ba-cbd3-4f06-919f-6c05683a78bc",
            Self::LinuxRootVeritySigARM32 => "42b0455f-eb11-491d-98d3-56145ba9d037",
            Self::LinuxRootVeritySigARM64 => "6db69de6-29f4-4758-a7a5-962190f00ce3",
            Self::LinuxRootVeritySigIA64 => "e98b36ee-32ba-4882-9b12-0ce14655f46a",
            Self::LinuxRootVeritySigLoongArch64 => "5afb67eb-ecc8-4f85-ae8e-ac1e7c50e7d0",
            Self::LinuxRootVeritySigMIPS => "bba210a2-9c5d-45ee-9e87-ff2ccbd002d0",
            Self::LinuxRootVeritySigMIPS64 => "43ce94d4-0f3d-4999-8250-b9deafd98e6e",
            Self::LinuxRootVeritySigMIPSEL => "c919cc1f-4456-4eff-918c-f75e94525ca5",
            Self::LinuxRootVeritySigMIPS64EL => "904e58ef-5c65-4a31-9c57-6af5fc7c5de7",
            Self::LinuxRootVeritySigPARISC => "15de6170-65d3-431c-916e-b0dcd8393f25",
            Self::LinuxRootVeritySigPPC32 => "1b31b5aa-add9-463a-b2ed-bd467fc857e7",
            Self::LinuxRootVeritySigPPC64 => "f5e2c20c-45b2-4ffa-bce9-2a60737e1aaf",
            Self::LinuxRootVeritySigPPC64LE => "d4a236e7-e873-4c07-bf1d-bf6cf7f1c3c6",
            Self::LinuxRootVeritySigRISCV32 => "3a112a75-8729-4380-b4cf-764d79934448",
            Self::LinuxRootVeritySigRISCV64 => "efe0f087-ea8d-4469-821a-4c2a96a8386a",
            Self::LinuxRootVeritySigS390 => "3482388e-4254-435a-a241-766a065f9960",
            Self::LinuxRootVeritySigS390x => "c80187a5-73a3-491a-901a-017c3fa953e9",
            Self::LinuxRootVeritySigTILEGx => "b3671439-97b0-4a53-90f7-2d5a8f3ad47b",
            Self::LinuxRootVeritySigx86 => "5996fc05-109c-48de-808b-23fa0830b676",
            Self::LinuxRootVeritySigx86_64 => "41092b05-9fc8-4523-994f-2def0408b176",
            Self::LinuxUsrVeritySigAlpha => "5c6e1c76-076a-457a-a0fe-f3b4cd21ce6e",
            Self::LinuxUsrVeritySigARC => "94f9a9a1-9971-427a-a400-50cb297f0f35",
            Self::LinuxUsrVeritySigARM32 => "d7ff812f-37d1-4902-a810-d76ba57b975a",
            Self::LinuxUsrVeritySigARM64 => "c23ce4ff-44bd-4b00-b2d4-b41b3419e02a",
            Self::LinuxUsrVeritySigIA64 => "8de58bc2-2a43-460d-b14e-a76e4a17b47f",
            Self::LinuxUsrVeritySigLoongArch64 => "b024f315-d330-444c-8461-44bbde524e99",
            Self::LinuxUsrVeritySigMIPS => "97ae158d-f216-497b-8057-f7f905770f54",
            Self::LinuxUsrVeritySigMIPS64 => "05816ce2-dd40-4ac6-a61d-37d32dc1ba7d",
            Self::LinuxUsrVeritySigMIPSEL => "3e23ca0b-a4bc-4b4e-8087-5ab6a26aa8a9",
            Self::LinuxUsrVeritySigMIPS64EL => "f2c2c7ee-adcc-4351-b5c6-ee9816b66e16",
            Self::LinuxUsrVeritySigPARISC => "450dd7d1-3224-45ec-9cf2-a43a346d71ee",
            Self::LinuxUsrVeritySigPPC32 => "7007891d-d371-4a80-86a4-5cb875b9302e",
            Self::LinuxUsrVeritySigPPC64 => "0b888863-d7f8-4d9e-9766-239fce4d58af",
            Self::LinuxUsrVeritySigPPC64LE => "c8bfbd1e-268e-4521-8bba-bf314c399557",
            Self::LinuxUsrVeritySigRISCV32 => "c3836a13-3137-45ba-b583-b16c50fe5eb4",
            Self::LinuxUsrVeritySigRISCV64 => "d2f9000a-7a18-453f-b5cd-4d32f77a7b32",
            Self::LinuxUsrVeritySigS390 => "17440e4f-a8d0-467f-a46e-3912ae6ef2c5",
            Self::LinuxUsrVeritySigS390x => "3f324816-667b-46ae-86ee-9b0c0c6c11b4",
            Self::LinuxUsrVeritySigTILEGx => "4ede75e2-6ccc-4cc8-b9c7-70334b087510",
            Self::LinuxUsrVeritySigx86 => "974a71c0-de41-43c3-be5d-5c5ccd1ad2c0",
            Self::LinuxUsrVeritySigx86_64 => "e7bb33fb-06cf-4e81-8273-e543b413e2e2",
            Self::LinuxXBOOTLDR => "bc13c2ff-59e6-4262-a352-b275fd6f7172",
            Self::LinuxSwap => "0657fd6d-a4ab-43c4-84e5-0933c84b4f4f",
            Self::LinuxLVM => "e6d6d379-f507-44c2-a23c-238f2a3df928",
            Self::LinuxHome => "933ac7e1-2eb4-4f13-b844-0e14e2aef915",
            Self::LinuxServerData => "3b8f8425-20e0-4f3b-907f-1a25a76f98e8",
            Self::LinuxPerUserHome => "773f91ef-66d4-49b5-bd83-d683bf40ad16",
            Self::LinuxPlain => "7ffec5c9-2d00-49b7-8941-3ea10a5586b7",
            Self::LinuxLUKS => "ca7d7ccb-63ed-4c53-861c-1742536059cc",
            Self::LinuxReserved => "8da63339-0007-60c0-c436-083ac8230908",
            // Self::GNUHurdData => "0fc63daf-8483-4772-8e79-3d69d8477de4",
            // Self::GNUHurdSwap => "0657fd6d-a4ab-43c4-84e5-0933c84b4f4f",
            Self::FreeBSDBoot => "83bd6b9d-7f41-11dc-be0b-001560b84f0f",
            Self::FreeBSDDisklabel => "516e7cb4-6ecf-11d6-8ff8-00022d09712b",
            Self::FreeBSDSwap => "516e7cb5-6ecf-11d6-8ff8-00022d09712b",
            Self::FreeBSDUFS => "516e7cb6-6ecf-11d6-8ff8-00022d09712b",
            Self::FreeBSDVinum => "516e7cb8-6ecf-11d6-8ff8-00022d09712b",
            Self::FreeBSDZFS => "516e7cba-6ecf-11d6-8ff8-00022d09712b",
            Self::FreeBSDnandfs => "74ba7dd9-a689-11e1-bd04-00e081286acf",
            Self::MacOSHFSPlus => "48465300-0000-11aa-aa11-00306543ecac",
            Self::MacOSAPFS => "7c3457ef-0000-11aa-aa11-00306543ecac",
            Self::MacOSUFS => "55465300-0000-11aa-aa11-00306543ecac",
            // Self::MacOSZFS => "6a898cc3-1dd2-11b2-99a6-080020736631",
            Self::MacOSRAID => "52414944-0000-11aa-aa11-00306543ecac",
            Self::MacOSRAIDOffline => "52414944-5f4f-11aa-aa11-00306543ecac",
            Self::MacOSBootRecovery => "426f6f74-0000-11aa-aa11-00306543ecac",
            Self::MacOSLabel => "4c616265-6c00-11aa-aa11-00306543ecac",
            Self::MacOSAppleTVRecovery => "5265636f-7665-11aa-aa11-00306543ecac",
            Self::MacOSHFSPlusFileVault => "53746f72-6167-11aa-aa11-00306543ecac",
            Self::MacOSAPFSPreboot => "69646961-6700-11aa-aa11-00306543ecac",
            Self::MacOSAPFSRecovery => "52637672-7900-11aa-aa11-00306543ecac",
            Self::SolarisBoot => "6a82cb45-1dd2-11b2-99a6-080020736631",
            Self::SolarisRoot => "6a85cf4d-1dd2-11b2-99a6-080020736631",
            Self::SolarisSwap => "6a87c46f-1dd2-11b2-99a6-080020736631",
            Self::SolarisBackup => "6a8b642b-1dd2-11b2-99a6-080020736631",
            Self::SolarisUsr => "6a898cc3-1dd2-11b2-99a6-080020736631",
            Self::SolarisVar => "6a8ef2e9-1dd2-11b2-99a6-080020736631",
            Self::SolarisHome => "6a90ba39-1dd2-11b2-99a6-080020736631",
            Self::SolarisAlternateSector => "6a9283a5-1dd2-11b2-99a6-080020736631",
            Self::SolarisReserved1 => "6a945a3b-1dd2-11b2-99a6-080020736631",
            Self::SolarisReserved2 => "6a9630d1-1dd2-11b2-99a6-080020736631",
            Self::SolarisReserved3 => "6a980767-1dd2-11b2-99a6-080020736631",
            Self::SolarisReserved4 => "6a96237f-1dd2-11b2-99a6-080020736631",
            Self::SolarisReserved5 => "6a8d2ac7-1dd2-11b2-99a6-080020736631",
            Self::NetBSDSwap => "49f48d32-b10e-11dc-b99b-0019d1879648",
            Self::NetBSDFFS => "49f48d5a-b10e-11dc-b99b-0019d1879648",
            Self::NetBSDLFS => "49f48d82-b10e-11dc-b99b-0019d1879648",
            Self::NetBSDRAID => "49f48daa-b10e-11dc-b99b-0019d1879648",
            Self::NetBSDConcatenated => "2db519c4-b10f-11dc-b99b-0019d1879648",
            Self::NetBSDEncrypted => "2db519ec-b10f-11dc-b99b-0019d1879648",
            Self::ChromeOSKernel => "fe3a2a5d-4f32-41a7-b725-accc3285a309",
            Self::ChromeOSRootFs => "3cb8e202-3b7e-47dd-8a3c-7ff2a13cfcec",
            Self::ChromeOSFirmware => "cab6e88e-abf3-4102-a07a-d4bb9be3c1d3",
            Self::ChromeOSFuture => "2e0a753d-9e48-43b0-8337-b15192cb1b5e",
            Self::ChromeOSMiniOS => "09845860-705f-4bb5-b16c-8a8a099caf52",
            Self::ChromeOSHibernate => "3f0f8318-f146-4e6b-8222-c28c8f02e0d5",
            Self::CoreOSUsr => "5dfbf5f4-2848-4bac-aa5e-0d9a20b745a6",
            Self::CoreOSResize => "3884dd41-8582-4404-b9a8-e9b84f2df50e",
            Self::CoreOSReserved => "c95dc21a-df0e-4340-8d7b-26cbfa9a03e0",
            Self::CoreOSRootRAID => "be9067b9-ea49-4f15-b4f6-f36f8c9e1818",
            Self::HaikuBFS => "42465331-3ba3-10f1-802a-4861696b7521",
            Self::MidnightBSDBoot => "85d5e45e-237c-11e1-b4b3-e89a8f7fc3a7",
            Self::MidnightBSDData => "85d5e45a-237c-11e1-b4b3-e89a8f7fc3a7",
            Self::MidnightBSDSwap => "85d5e45b-237c-11e1-b4b3-e89a8f7fc3a7",
            Self::MidnightBSDUFS => "0394ef8b-237e-11e1-b4b3-e89a8f7fc3a7",
            Self::MidnightBSDVinum => "85d5e45c-237c-11e1-b4b3-e89a8f7fc3a7",
            Self::MidnightBSDZFS => "85d5e45d-237c-11e1-b4b3-e89a8f7fc3a7",
            Self::CephJournal => "45b0969e-9b03-4f30-b4c6-b4b80ceff106",
            Self::CephDMCryptJournal => "45b0969e-9b03-4f30-b4c6-5ec00ceff106",
            Self::CephOSD => "4fbd7e29-9d25-41b8-afd0-062c0ceff05d",
            Self::CephDMCryptOSD => "4fbd7e29-9d25-41b8-afd0-5ec00ceff05d",
            Self::CephDisk => "89c57f98-2fe5-4dc0-89c1-f3ad0ceff2be",
            Self::CephDMCryptDisk => "89c57f98-2fe5-4dc0-89c1-5ec00ceff2be",
            Self::CephBlock => "cafecafe-9b03-4f30-b4c6-b4b80ceff106",
            Self::CephBlockDB => "30cd0809-c2b2-499c-8879-2d6b78529876",
            Self::CephBlockLog => "5ce17fce-4087-4169-b7ff-056cc58473f9",
            Self::CephLockbox => "fb3aabf9-d25f-47cc-bf5e-721d1816496b",
            Self::CephMultipathOSD => "4fbd7e29-8ae0-4982-bf9d-5a8d867af560",
            Self::CephMultipathJournal => "45b0969e-8ae0-4982-bf9d-5a8d867af560",
            Self::CephMultipathBlock1 => "cafecafe-8ae0-4982-bf9d-5a8d867af560",
            Self::CephMultipathBlock2 => "7f4a666a-16f3-47a2-8445-152ef4d03f6c",
            Self::CephMultipathBlockDB => "ec6d6385-e346-45dc-be91-da2a7c8b3261",
            Self::CephMultipathLog => "01b41e1b-002a-453c-9f17-88793989ff8f",
            Self::CephDMCryptBlock => "cafecafe-9b03-4f30-b4c6-5ec00ceff106",
            Self::CephDMCryptBlockDB => "93b0052d-02d9-4d8a-a43b-33a3ee4dfbc3",
            Self::CephDMCryptBlockLog => "306e8683-4fe2-4330-b7c0-00a917c16966",
            Self::CephLUKSOSD => "4fbd7e29-9d25-41b8-afd0-35865ceff05d",
            Self::CephLUKSJournal => "45b0969e-9b03-4f30-b4c6-35865ceff106",
            Self::CephLUKSBlock => "cafecafe-9b03-4f30-b4c6-35865ceff106",
            Self::CephLUKSBlockDB => "166418da-c469-4022-adf4-b30afd37f176",
            Self::CephLUKSBlockLog => "86a32090-3647-40b9-bbbd-38d8c573aa86",
            Self::OpenBSDData => "824cc7a0-36a8-11e3-890a-952519ad3f61",
            Self::QNX6Fs => "cef5a9ad-73bc-4601-89f3-cdeeeee321a1",
            Self::Plan9 => "c91818f9-8025-47af-89d2-f030d7000c2c",
            Self::VMWareVMKCORE => "9d275380-40ad-11db-bf97-000c2911d1b8",
            Self::VMWareVMFS => "aa31e02a-400f-11db-9590-000c2911d1b8",
            Self::VMWareWmkReserved => "9198effc-31c0-11db-8f78-000c2911d1b8",
            Self::AndroidBootloader => "2568845d-2332-4675-bc39-8fa5a4748d15",
            Self::AndroidBootloader2 => "114eaffe-1552-4022-b26e-9b053604cf84",
            Self::AndroidBoot => "49a4d17f-93a3-45c1-a0de-f50b2ebe2599",
            Self::AndroidRecovery => "4177c722-9e92-4aab-8644-43502bfd5506",
            Self::AndroidMisc => "ef32a33b-a409-486c-9141-9ffb711f6266",
            Self::AndroidMetadata => "20ac26be-20b7-11e3-84c5-6cfdb94711e9",
            Self::AndroidSystem => "38f428e6-d326-425d-9140-6e0ea133647c",
            Self::AndroidCache => "a893ef21-e428-470a-9e55-0668fd91a2d9",
            Self::AndroidData => "dc76dda9-5ac1-491c-af42-a82591580c0d",
            Self::AndroidPersistent => "ebc597d0-2053-4b15-8b64-e0aac75f4db1",
            Self::AndroidVendor => "c5a0aeec-13ea-11e5-a1b1-001e67ca0c3c",
            Self::AndroidConfig => "bd59408b-4514-490d-bf12-9878d963f378",
            Self::AndroidFactory => "8f68cc74-c5e5-48da-be91-a0c8c15e9c80",
            Self::AndroidFactoryAlt => "9fdaa6ef-4b3f-40d2-ba8d-bff16bfb887b",
            Self::AndroidFastboot => "767941d0-2085-11e3-ad3b-6cfdb94711e9",
            Self::AndroidOEM => "ac6d7924-eb71-4df8-b48d-e267b27148ff",
            Self::Android6Meta => "19a710a2-b3ca-11e4-b026-10604b889dcf",
            Self::Android6Ext => "193d1ea4-b3ca-11e4-b075-10604b889dcf",
            Self::ONIEBoot => "7412f7d5-a156-4b13-81dc-867174929325",
            Self::ONIEConfig => "d4e6e2cd-4469-46f3-b5cb-1bff57afc149",
            Self::PPCPrePBoot => "9e1a2d38-c612-4316-aa26-8b49521e5a8b",
            // Self::FreeDesktopConfig => "bc13c2ff-59e6-4262-a352-b275fd6f7172",
            Self::AtariTOSBasicData => "734e5afe-f61a-11e6-bc64-92361f002671",
            Self::AtariTOSRawData => "35540011-b055-499f-842d-c69aeca357b7",
            Self::VeraCryptEncrypted => "8c8f8eff-ac95-4770-814a-21994f2dbc8f",
            Self::OS2ArcaOS => "90b6ff38-b98f-4358-a21f-48f35b4a8ad3",
            Self::SPDK => "7c5222bd-8f5d-4087-9c00-bf9843c7b58c",
            Self::BareboxState => "4778ed65-bf42-45fa-9c5b-287a1dc4aab1",
            Self::UBootEnv => "3de21764-95bd-54bd-a5c3-4abe786f38a8",
            Self::SoftRAIDStatus => "b6fa30da-92d2-4a9a-96f1-871ec6486200",
            Self::SoftRAIDScratch => "2e313465-19b9-463f-8126-8a7993773801",
            Self::SoftRAIDVolume => "fa709c7e-65b1-4593-bfd5-e71d61de9b02",
            Self::SoftRAIDCache => "bbba6df5-f46f-4a89-8f59-8765b2727503",
            Self::FuchsiaBoot => "fe8a2634-5e2e-46ba-99e3-3a192091a350",
            Self::FuchsiaSystemData => "d9fd4535-106c-4cec-8d37-dfc020ca87cb",
            Self::FuchsiaBootData => "a409e16b-78aa-4acc-995c-302352621a41",
            Self::FuchsiaFactorySystemData => "f95d940e-caba-4578-9b93-bb6c90f29d3e",
            Self::FuchsiaFactoryBootData => "10b8dbaa-d2bf-42a9-98c6-a7c5db3701e7",
            Self::FuchsiaVolumeManager => "49fd7cb8-df15-4e73-b9d9-992070127f0f",
            Self::FuchsiaVerifiedBoot => "421a8bfc-85d9-4d85-acda-b64eec0133e9",
            Self::FuchsiaZirconBoot => "9b37fff6-2e58-466a-983a-f7926d0b04e0",
            // Self::FuchsiaLegacyESP => "c12a7328-f81f-11d2-ba4b-00a0c93ec93b",
            Self::FuchsiaLegacySystem => "606b000b-b7c7-4653-a7d5-b737332c899d",
            Self::FuchsiaLegacyData => "08185f0c-892d-428a-a789-dbeec8f55e6a",
            Self::FuchsiaLegacyInstall => "48435546-4953-2041-494e-5354414c4c52",
            Self::FuchsiaLegacyBlob => "2967380e-134c-4cbb-b6da-17e7ce1ca45d",
            Self::FuchsiaLegacyFVM => "41d0e340-57e3-954e-8c1e-17ecac44cff5",
            Self::FuchsiaLegacyZirconBootSlotA => "de30cc86-1f4a-4a31-93c4-66f147d33e05",
            Self::FuchsiaLegacyZirconBootSlotB => "23cc04df-c278-4ce7-8471-897d1a4bcdf7",
            Self::FuchsiaLegacyZirconBootSlotR => "a0e5cf57-2def-46be-a80c-a2067c37cd49",
            Self::FuchsiaLegacySysConfig => "4e5e989e-4c86-11e8-a15b-480fcf35f8e6",
            Self::FuchsiaLegacyFactoryConfig => "5a3a90be-4c86-11e8-a15b-480fcf35f8e6",
            Self::FuchsiaLegacyBoot => "5ece94fe-4c86-11e8-a15b-480fcf35f8e6",
            Self::FuchsiaLegacyGuidTest => "8b94d043-30be-4871-9dfa-d69556e8c1f3",
            Self::FuchsiaLegacyVerifiedBootSlotA => "a13b4d9a-ec5f-11e8-97d8-6c3be52705bf",
            Self::FuchsiaLegacyVerifiedBootSlotB => "a288abf2-ec5f-11e8-97d8-6c3be52705bf",
            Self::FuchsiaLegacyVerifiedBootSlotR => "6a2460c3-cd11-4e8b-80a8-12cce268ed0a",
            Self::FuchsiaLegacyMisc => "1d75395d-f2c6-476b-a8b7-45cc1c97b476",
            Self::FuchsiaLegacyEmmcBoot1 => "900b0fc5-90cd-4d4f-84f9-9f8ed579db88",
            Self::FuchsiaLegacyEmmcBoot2 => "b2b2e8d1-7c10-4ebc-a2d0-4614568260ad",
            Self::Minix => "481b2a38-0561-420b-b72a-f1c4988efc16",
        }
    }
}

impl AsRef<Guid> for Guid {
    #[inline]
    fn as_ref(&self) -> &Guid {
        self
    }
}

impl AsRef<str> for Guid {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&[u8]> for Guid {
    type Error = ConversionError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(bytes)
            .map_err(|e| {
                ConversionError::Guid(format!(
                    "bytes to UTF-8 string slice conversion error. {:?}",
                    e
                ))
            })
            .and_then(|s| Self::from_str(s).map_err(|e| ConversionError::Guid(e.to_string())))
    }
}

impl TryFrom<Vec<u8>> for Guid {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl FromStr for Guid {
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
                .ok_or(ParserError::Guid(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::Guid(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        match stripped.trim().to_lowercase().as_str() {
            "00000000-0000-0000-0000-000000000000" => Ok(Self::EmptyPartition),
            "024dee41-33e7-11d3-9d69-0008c781f39f" => Ok(Self::MBRPartition),
            "c12a7328-f81f-11d2-ba4b-00a0c93ec93b" => Ok(Self::EfiSystem),
            "21686148-6449-6e6f-744e-656564454649" => Ok(Self::BIOSBoot),
            "d3bfe2de-3daf-11df-ba40-e3a556d89593" => Ok(Self::IntelFastFlash),
            "f4019732-066e-4e12-8273-346c5641494f" => Ok(Self::SonyBoot),
            "bfbfafe7-a34f-448a-9a5b-6213eb736c22" => Ok(Self::LenovoBoot),
            "e3c9e316-0b5c-4db8-817d-f92df00215ae" => Ok(Self::MicrosoftReserved),
            "ebd0a0a2-b9e5-4433-87c0-68b6b72699c7" => Ok(Self::WindowsBasicData),
            "5808c8aa-7e8f-42e0-85d2-e1e90434cfb3" => Ok(Self::WindowsLDM),
            "af9b60a0-1431-4f62-bc68-3311714a69ad" => Ok(Self::WindowsLDMData),
            "de94bba4-06d1-4d40-a16a-bfd50179d6ac" => Ok(Self::WindowsRecovery),
            "37affc90-ef7d-4e96-91c3-2d7ae055b174" => Ok(Self::IBMGPFS),
            "e75caf8f-f680-4cee-afa3-b001e56efc2d" => Ok(Self::WindowsStorageSpaces),
            "558d43c5-a1ac-43c0-aac8-d1472b2923d1" => Ok(Self::WindowsStorageReplica),
            "75894c1e-3aeb-11d3-b7c1-7b03a0000000" => Ok(Self::HPUXData),
            "e2a1e728-32e3-11d6-a682-7b03a0000000" => Ok(Self::HPUXService),
            "0fc63daf-8483-4772-8e79-3d69d8477de4" => Ok(Self::LinuxData),
            "a19d880f-05fc-4d3b-a006-743f0f84911e" => Ok(Self::LinuxRaid),
            "6523f8ae-3eb1-4e2a-a05a-18b695ae656f" => Ok(Self::LinuxRootAlpha),
            "d27f46ed-2919-4cb8-bd25-9531f3c16534" => Ok(Self::LinuxRootARC),
            "69dad710-2ce4-4e3c-b16c-21a1d49abed3" => Ok(Self::LinuxRootARM32),
            "b921b045-1df0-41c3-af44-4c6f280d3fae" => Ok(Self::LinuxRootARM64),
            "993d8d3d-f80e-4225-855a-9daf8ed7ea97" => Ok(Self::LinuxRootIA64),
            "77055800-792c-4f94-b39a-98c91b762bb6" => Ok(Self::LinuxRootLoongArch64),
            "e9434544-6e2c-47cc-bae2-12d6deafb44c" => Ok(Self::LinuxRootMIPS),
            "d113af76-80ef-41b4-bdb6-0cff4d3d4a25" => Ok(Self::LinuxRootMIPS64),
            "37c58c8a-d913-4156-a25f-48b1b64e07f0" => Ok(Self::LinuxRootMIPSEL),
            "700bda43-7a34-4507-b179-eeb93d7a7ca3" => Ok(Self::LinuxRootMIPS64EL),
            "1aacdb3b-5444-4138-bd9e-e5c2239b2346" => Ok(Self::LinuxRootPARISC),
            "1de3f1ef-fa98-47b5-8dcd-4a860a654d78" => Ok(Self::LinuxRootPPC32),
            "912ade1d-a839-4913-8964-a10eee08fbd2" => Ok(Self::LinuxRootPPC64),
            "c31c45e6-3f39-412e-80fb-4809c4980599" => Ok(Self::LinuxRootPPC64LE),
            "60d5a7fe-8e7d-435c-b714-3dd8162144e1" => Ok(Self::LinuxRootRISCV32),
            "72ec70a6-cf74-40e6-bd49-4bda08e8f224" => Ok(Self::LinuxRootRISCV64),
            "08a7acea-624c-4a20-91e8-6e0fa67d23f9" => Ok(Self::LinuxRootS390),
            "5eead9a9-fe09-4a1e-a1d7-520d00531306" => Ok(Self::LinuxRootS390x),
            "c50cdd70-3862-4cc3-90e1-809a8c93ee2c" => Ok(Self::LinuxRootTILEGx),
            "44479540-f297-41b2-9af7-d131d5f0458a" => Ok(Self::LinuxRootx86),
            "4f68bce3-e8cd-4db1-96e7-fbcaf984b709" => Ok(Self::LinuxRootx86_64),
            "e18cf08c-33ec-4c0d-8246-c6c6fb3da024" => Ok(Self::LinuxUsrAlpha),
            "7978a683-6316-4922-bbee-38bff5a2fecc" => Ok(Self::LinuxUsrARC),
            "7d0359a3-02b3-4f0a-865c-654403e70625" => Ok(Self::LinuxUsrARM32),
            "b0e01050-ee5f-4390-949a-9101b17104e9" => Ok(Self::LinuxUsrARM64),
            "4301d2a6-4e3b-4b2a-bb94-9e0b2c4225ea" => Ok(Self::LinuxUsrIA64),
            "e611c702-575c-4cbe-9a46-434fa0bf7e3f" => Ok(Self::LinuxUsrLoongArch64),
            "773b2abc-2a99-4398-8bf5-03baac40d02b" => Ok(Self::LinuxUsrMIPS),
            "57e13958-7331-4365-8e6e-35eeee17c61b" => Ok(Self::LinuxUsrMIPS64),
            "0f4868e9-9952-4706-979f-3ed3a473e947" => Ok(Self::LinuxUsrMIPSEL),
            "c97c1f32-ba06-40b4-9f22-236061b08aa8" => Ok(Self::LinuxUsrMIPS64EL),
            "dc4a4480-6917-4262-a4ec-db9384949f25" => Ok(Self::LinuxUsrPARISC),
            "7d14fec5-cc71-415d-9d6c-06bf0b3c3eaf" => Ok(Self::LinuxUsrPPC32),
            "2c9739e2-f068-46b3-9fd0-01c5a9afbcca" => Ok(Self::LinuxUsrPPC64),
            "15bb03af-77e7-4d4a-b12b-c0d084f7491c" => Ok(Self::LinuxUsrPPC64LE),
            "b933fb22-5c3f-4f91-af90-e2bb0fa50702" => Ok(Self::LinuxUsrRISCV32),
            "beaec34b-8442-439b-a40b-984381ed097d" => Ok(Self::LinuxUsrRISCV64),
            "cd0f869b-d0fb-4ca0-b141-9ea87cc78d66" => Ok(Self::LinuxUsrS390),
            "8a4f5770-50aa-4ed3-874a-99b710db6fea" => Ok(Self::LinuxUsrS390x),
            "55497029-c7c1-44cc-aa39-815ed1558630" => Ok(Self::LinuxUsrTILEGx),
            "75250d76-8cc6-458e-bd66-bd47cc81a812" => Ok(Self::LinuxUsrx86),
            "8484680c-9521-48c6-9c11-b0720656f69e" => Ok(Self::LinuxUsrx86_64),
            "fc56d9e9-e6e5-4c06-be32-e74407ce09a5" => Ok(Self::LinuxRootVerityAlpha),
            "24b2d975-0f97-4521-afa1-cd531e421b8d" => Ok(Self::LinuxRootVerityARC),
            "7386cdf2-203c-47a9-a498-f2ecce45a2d6" => Ok(Self::LinuxRootVerityARM32),
            "df3300ce-d69f-4c92-978c-9bfb0f38d820" => Ok(Self::LinuxRootVerityARM64),
            "86ed10d5-b607-45bb-8957-d350f23d0571" => Ok(Self::LinuxRootVerityIA64),
            "f3393b22-e9af-4613-a948-9d3bfbd0c535" => Ok(Self::LinuxRootVerityLoongArch64),
            "7a430799-f711-4c7e-8e5b-1d685bd48607" => Ok(Self::LinuxRootVerityMIPS),
            "579536f8-6a33-4055-a95a-df2d5e2c42a8" => Ok(Self::LinuxRootVerityMIPS64),
            "d7d150d2-2a04-4a33-8f12-16651205ff7b" => Ok(Self::LinuxRootVerityMIPSEL),
            "16b417f8-3e06-4f57-8dd2-9b5232f41aa6" => Ok(Self::LinuxRootVerityMIPS64EL),
            "d212a430-fbc5-49f9-a983-a7feef2b8d0e" => Ok(Self::LinuxRootVerityPARISC),
            "98cfe649-1588-46dc-b2f0-add147424925" => Ok(Self::LinuxRootVerityPPC32),
            "9225a9a3-3c19-4d89-b4f6-eeff88f17631" => Ok(Self::LinuxRootVerityPPC64),
            "906bd944-4589-4aae-a4e4-dd983917446a" => Ok(Self::LinuxRootVerityPPC64LE),
            "ae0253be-1167-4007-ac68-43926c14c5de" => Ok(Self::LinuxRootVerityRISCV32),
            "b6ed5582-440b-4209-b8da-5ff7c419ea3d" => Ok(Self::LinuxRootVerityRISCV64),
            "7ac63b47-b25c-463b-8df8-b4a94e6c90e1" => Ok(Self::LinuxRootVerityS390),
            "b325bfbe-c7be-4ab8-8357-139e652d2f6b" => Ok(Self::LinuxRootVerityS390x),
            "966061ec-28e4-4b2e-b4a5-1f0a825a1d84" => Ok(Self::LinuxRootVerityTILEGx),
            "d13c5d3b-b5d1-422a-b29f-9454fdc89d76" => Ok(Self::LinuxRootVerityx86),
            "2c7357ed-ebd2-46d9-aec1-23d437ec2bf5" => Ok(Self::LinuxRootVerityx86_64),
            "8cce0d25-c0d0-4a44-bd87-46331bf1df67" => Ok(Self::LinuxUsrVerityAlpha),
            "fca0598c-d880-4591-8c16-4eda05c7347c" => Ok(Self::LinuxUsrVerityARC),
            "c215d751-7bcd-4649-be90-6627490a4c05" => Ok(Self::LinuxUsrVerityARM32),
            "6e11a4e7-fbca-4ded-b9e9-e1a512bb664e" => Ok(Self::LinuxUsrVerityARM64),
            "6a491e03-3be7-4545-8e38-83320e0ea880" => Ok(Self::LinuxUsrVerityIA64),
            "f46b2c26-59ae-48f0-9106-c50ed47f673d" => Ok(Self::LinuxUsrVerityLoongArch64),
            "6e5a1bc8-d223-49b7-bca8-37a5fcceb996" => Ok(Self::LinuxUsrVerityMIPS),
            "81cf9d90-7458-4df4-8dcf-c8a3a404f09b" => Ok(Self::LinuxUsrVerityMIPS64),
            "46b98d8d-b55c-4e8f-aab3-37fca7f80752" => Ok(Self::LinuxUsrVerityMIPSEL),
            "3c3d61fe-b5f3-414d-bb71-8739a694a4ef" => Ok(Self::LinuxUsrVerityMIPS64EL),
            "5843d618-ec37-48d7-9f12-cea8e08768b2" => Ok(Self::LinuxUsrVerityPARISC),
            "df765d00-270e-49e5-bc75-f47bb2118b09" => Ok(Self::LinuxUsrVerityPPC32),
            "bdb528a5-a259-475f-a87d-da53fa736a07" => Ok(Self::LinuxUsrVerityPPC64),
            "ee2b9983-21e8-4153-86d9-b6901a54d1ce" => Ok(Self::LinuxUsrVerityPPC64LE),
            "cb1ee4e3-8cd0-4136-a0a4-aa61a32e8730" => Ok(Self::LinuxUsrVerityRISCV32),
            "8f1056be-9b05-47c4-81d6-be53128e5b54" => Ok(Self::LinuxUsrVerityRISCV64),
            "b663c618-e7bc-4d6d-90aa-11b756bb1797" => Ok(Self::LinuxUsrVerityS390),
            "31741cc4-1a2a-4111-a581-e00b447d2d06" => Ok(Self::LinuxUsrVerityS390x),
            "2fb4bf56-07fa-42da-8132-6b139f2026ae" => Ok(Self::LinuxUsrVerityTILEGx),
            "8f461b0d-14ee-4e81-9aa9-049b6fb97abd" => Ok(Self::LinuxUsrVerityx86),
            "77ff5f63-e7b6-4633-acf4-1565b864c0e6" => Ok(Self::LinuxUsrVerityx86_64),
            "d46495b7-a053-414f-80f7-700c99921ef8" => Ok(Self::LinuxRootVeritySigAlpha),
            "143a70ba-cbd3-4f06-919f-6c05683a78bc" => Ok(Self::LinuxRootVeritySigARC),
            "42b0455f-eb11-491d-98d3-56145ba9d037" => Ok(Self::LinuxRootVeritySigARM32),
            "6db69de6-29f4-4758-a7a5-962190f00ce3" => Ok(Self::LinuxRootVeritySigARM64),
            "e98b36ee-32ba-4882-9b12-0ce14655f46a" => Ok(Self::LinuxRootVeritySigIA64),
            "5afb67eb-ecc8-4f85-ae8e-ac1e7c50e7d0" => Ok(Self::LinuxRootVeritySigLoongArch64),
            "bba210a2-9c5d-45ee-9e87-ff2ccbd002d0" => Ok(Self::LinuxRootVeritySigMIPS),
            "43ce94d4-0f3d-4999-8250-b9deafd98e6e" => Ok(Self::LinuxRootVeritySigMIPS64),
            "c919cc1f-4456-4eff-918c-f75e94525ca5" => Ok(Self::LinuxRootVeritySigMIPSEL),
            "904e58ef-5c65-4a31-9c57-6af5fc7c5de7" => Ok(Self::LinuxRootVeritySigMIPS64EL),
            "15de6170-65d3-431c-916e-b0dcd8393f25" => Ok(Self::LinuxRootVeritySigPARISC),
            "1b31b5aa-add9-463a-b2ed-bd467fc857e7" => Ok(Self::LinuxRootVeritySigPPC32),
            "f5e2c20c-45b2-4ffa-bce9-2a60737e1aaf" => Ok(Self::LinuxRootVeritySigPPC64),
            "d4a236e7-e873-4c07-bf1d-bf6cf7f1c3c6" => Ok(Self::LinuxRootVeritySigPPC64LE),
            "3a112a75-8729-4380-b4cf-764d79934448" => Ok(Self::LinuxRootVeritySigRISCV32),
            "efe0f087-ea8d-4469-821a-4c2a96a8386a" => Ok(Self::LinuxRootVeritySigRISCV64),
            "3482388e-4254-435a-a241-766a065f9960" => Ok(Self::LinuxRootVeritySigS390),
            "c80187a5-73a3-491a-901a-017c3fa953e9" => Ok(Self::LinuxRootVeritySigS390x),
            "b3671439-97b0-4a53-90f7-2d5a8f3ad47b" => Ok(Self::LinuxRootVeritySigTILEGx),
            "5996fc05-109c-48de-808b-23fa0830b676" => Ok(Self::LinuxRootVeritySigx86),
            "41092b05-9fc8-4523-994f-2def0408b176" => Ok(Self::LinuxRootVeritySigx86_64),
            "5c6e1c76-076a-457a-a0fe-f3b4cd21ce6e" => Ok(Self::LinuxUsrVeritySigAlpha),
            "94f9a9a1-9971-427a-a400-50cb297f0f35" => Ok(Self::LinuxUsrVeritySigARC),
            "d7ff812f-37d1-4902-a810-d76ba57b975a" => Ok(Self::LinuxUsrVeritySigARM32),
            "c23ce4ff-44bd-4b00-b2d4-b41b3419e02a" => Ok(Self::LinuxUsrVeritySigARM64),
            "8de58bc2-2a43-460d-b14e-a76e4a17b47f" => Ok(Self::LinuxUsrVeritySigIA64),
            "b024f315-d330-444c-8461-44bbde524e99" => Ok(Self::LinuxUsrVeritySigLoongArch64),
            "97ae158d-f216-497b-8057-f7f905770f54" => Ok(Self::LinuxUsrVeritySigMIPS),
            "05816ce2-dd40-4ac6-a61d-37d32dc1ba7d" => Ok(Self::LinuxUsrVeritySigMIPS64),
            "3e23ca0b-a4bc-4b4e-8087-5ab6a26aa8a9" => Ok(Self::LinuxUsrVeritySigMIPSEL),
            "f2c2c7ee-adcc-4351-b5c6-ee9816b66e16" => Ok(Self::LinuxUsrVeritySigMIPS64EL),
            "450dd7d1-3224-45ec-9cf2-a43a346d71ee" => Ok(Self::LinuxUsrVeritySigPARISC),
            "7007891d-d371-4a80-86a4-5cb875b9302e" => Ok(Self::LinuxUsrVeritySigPPC32),
            "0b888863-d7f8-4d9e-9766-239fce4d58af" => Ok(Self::LinuxUsrVeritySigPPC64),
            "c8bfbd1e-268e-4521-8bba-bf314c399557" => Ok(Self::LinuxUsrVeritySigPPC64LE),
            "c3836a13-3137-45ba-b583-b16c50fe5eb4" => Ok(Self::LinuxUsrVeritySigRISCV32),
            "d2f9000a-7a18-453f-b5cd-4d32f77a7b32" => Ok(Self::LinuxUsrVeritySigRISCV64),
            "17440e4f-a8d0-467f-a46e-3912ae6ef2c5" => Ok(Self::LinuxUsrVeritySigS390),
            "3f324816-667b-46ae-86ee-9b0c0c6c11b4" => Ok(Self::LinuxUsrVeritySigS390x),
            "4ede75e2-6ccc-4cc8-b9c7-70334b087510" => Ok(Self::LinuxUsrVeritySigTILEGx),
            "974a71c0-de41-43c3-be5d-5c5ccd1ad2c0" => Ok(Self::LinuxUsrVeritySigx86),
            "e7bb33fb-06cf-4e81-8273-e543b413e2e2" => Ok(Self::LinuxUsrVeritySigx86_64),
            "bc13c2ff-59e6-4262-a352-b275fd6f7172" => Ok(Self::LinuxXBOOTLDR),
            "0657fd6d-a4ab-43c4-84e5-0933c84b4f4f" => Ok(Self::LinuxSwap),
            "e6d6d379-f507-44c2-a23c-238f2a3df928" => Ok(Self::LinuxLVM),
            "933ac7e1-2eb4-4f13-b844-0e14e2aef915" => Ok(Self::LinuxHome),
            "3b8f8425-20e0-4f3b-907f-1a25a76f98e8" => Ok(Self::LinuxServerData),
            "773f91ef-66d4-49b5-bd83-d683bf40ad16" => Ok(Self::LinuxPerUserHome),
            "7ffec5c9-2d00-49b7-8941-3ea10a5586b7" => Ok(Self::LinuxPlain),
            "ca7d7ccb-63ed-4c53-861c-1742536059cc" => Ok(Self::LinuxLUKS),
            "8da63339-0007-60c0-c436-083ac8230908" => Ok(Self::LinuxReserved),
            // "0fc63daf-8483-4772-8e79-3d69d8477de4" => Ok(Self::GNUHurdData),
            // "0657fd6d-a4ab-43c4-84e5-0933c84b4f4f" => Ok(Self::GNUHurdSwap),
            "83bd6b9d-7f41-11dc-be0b-001560b84f0f" => Ok(Self::FreeBSDBoot),
            "516e7cb4-6ecf-11d6-8ff8-00022d09712b" => Ok(Self::FreeBSDDisklabel),
            "516e7cb5-6ecf-11d6-8ff8-00022d09712b" => Ok(Self::FreeBSDSwap),
            "516e7cb6-6ecf-11d6-8ff8-00022d09712b" => Ok(Self::FreeBSDUFS),
            "516e7cb8-6ecf-11d6-8ff8-00022d09712b" => Ok(Self::FreeBSDVinum),
            "516e7cba-6ecf-11d6-8ff8-00022d09712b" => Ok(Self::FreeBSDZFS),
            "74ba7dd9-a689-11e1-bd04-00e081286acf" => Ok(Self::FreeBSDnandfs),
            "48465300-0000-11aa-aa11-00306543ecac" => Ok(Self::MacOSHFSPlus),
            "7c3457ef-0000-11aa-aa11-00306543ecac" => Ok(Self::MacOSAPFS),
            "55465300-0000-11aa-aa11-00306543ecac" => Ok(Self::MacOSUFS),
            // "6a898cc3-1dd2-11b2-99a6-080020736631" => Ok(Self::MacOSZFS),
            "52414944-0000-11aa-aa11-00306543ecac" => Ok(Self::MacOSRAID),
            "52414944-5f4f-11aa-aa11-00306543ecac" => Ok(Self::MacOSRAIDOffline),
            "426f6f74-0000-11aa-aa11-00306543ecac" => Ok(Self::MacOSBootRecovery),
            "4c616265-6c00-11aa-aa11-00306543ecac" => Ok(Self::MacOSLabel),
            "5265636f-7665-11aa-aa11-00306543ecac" => Ok(Self::MacOSAppleTVRecovery),
            "53746f72-6167-11aa-aa11-00306543ecac" => Ok(Self::MacOSHFSPlusFileVault),
            "69646961-6700-11aa-aa11-00306543ecac" => Ok(Self::MacOSAPFSPreboot),
            "52637672-7900-11aa-aa11-00306543ecac" => Ok(Self::MacOSAPFSRecovery),
            "6a82cb45-1dd2-11b2-99a6-080020736631" => Ok(Self::SolarisBoot),
            "6a85cf4d-1dd2-11b2-99a6-080020736631" => Ok(Self::SolarisRoot),
            "6a87c46f-1dd2-11b2-99a6-080020736631" => Ok(Self::SolarisSwap),
            "6a8b642b-1dd2-11b2-99a6-080020736631" => Ok(Self::SolarisBackup),
            "6a898cc3-1dd2-11b2-99a6-080020736631" => Ok(Self::SolarisUsr),
            "6a8ef2e9-1dd2-11b2-99a6-080020736631" => Ok(Self::SolarisVar),
            "6a90ba39-1dd2-11b2-99a6-080020736631" => Ok(Self::SolarisHome),
            "6a9283a5-1dd2-11b2-99a6-080020736631" => Ok(Self::SolarisAlternateSector),
            "6a945a3b-1dd2-11b2-99a6-080020736631" => Ok(Self::SolarisReserved1),
            "6a9630d1-1dd2-11b2-99a6-080020736631" => Ok(Self::SolarisReserved2),
            "6a980767-1dd2-11b2-99a6-080020736631" => Ok(Self::SolarisReserved3),
            "6a96237f-1dd2-11b2-99a6-080020736631" => Ok(Self::SolarisReserved4),
            "6a8d2ac7-1dd2-11b2-99a6-080020736631" => Ok(Self::SolarisReserved5),
            "49f48d32-b10e-11dc-b99b-0019d1879648" => Ok(Self::NetBSDSwap),
            "49f48d5a-b10e-11dc-b99b-0019d1879648" => Ok(Self::NetBSDFFS),
            "49f48d82-b10e-11dc-b99b-0019d1879648" => Ok(Self::NetBSDLFS),
            "49f48daa-b10e-11dc-b99b-0019d1879648" => Ok(Self::NetBSDRAID),
            "2db519c4-b10f-11dc-b99b-0019d1879648" => Ok(Self::NetBSDConcatenated),
            "2db519ec-b10f-11dc-b99b-0019d1879648" => Ok(Self::NetBSDEncrypted),
            "fe3a2a5d-4f32-41a7-b725-accc3285a309" => Ok(Self::ChromeOSKernel),
            "3cb8e202-3b7e-47dd-8a3c-7ff2a13cfcec" => Ok(Self::ChromeOSRootFs),
            "cab6e88e-abf3-4102-a07a-d4bb9be3c1d3" => Ok(Self::ChromeOSFirmware),
            "2e0a753d-9e48-43b0-8337-b15192cb1b5e" => Ok(Self::ChromeOSFuture),
            "09845860-705f-4bb5-b16c-8a8a099caf52" => Ok(Self::ChromeOSMiniOS),
            "3f0f8318-f146-4e6b-8222-c28c8f02e0d5" => Ok(Self::ChromeOSHibernate),
            "5dfbf5f4-2848-4bac-aa5e-0d9a20b745a6" => Ok(Self::CoreOSUsr),
            "3884dd41-8582-4404-b9a8-e9b84f2df50e" => Ok(Self::CoreOSResize),
            "c95dc21a-df0e-4340-8d7b-26cbfa9a03e0" => Ok(Self::CoreOSReserved),
            "be9067b9-ea49-4f15-b4f6-f36f8c9e1818" => Ok(Self::CoreOSRootRAID),
            "42465331-3ba3-10f1-802a-4861696b7521" => Ok(Self::HaikuBFS),
            "85d5e45e-237c-11e1-b4b3-e89a8f7fc3a7" => Ok(Self::MidnightBSDBoot),
            "85d5e45a-237c-11e1-b4b3-e89a8f7fc3a7" => Ok(Self::MidnightBSDData),
            "85d5e45b-237c-11e1-b4b3-e89a8f7fc3a7" => Ok(Self::MidnightBSDSwap),
            "0394ef8b-237e-11e1-b4b3-e89a8f7fc3a7" => Ok(Self::MidnightBSDUFS),
            "85d5e45c-237c-11e1-b4b3-e89a8f7fc3a7" => Ok(Self::MidnightBSDVinum),
            "85d5e45d-237c-11e1-b4b3-e89a8f7fc3a7" => Ok(Self::MidnightBSDZFS),
            "45b0969e-9b03-4f30-b4c6-b4b80ceff106" => Ok(Self::CephJournal),
            "45b0969e-9b03-4f30-b4c6-5ec00ceff106" => Ok(Self::CephDMCryptJournal),
            "4fbd7e29-9d25-41b8-afd0-062c0ceff05d" => Ok(Self::CephOSD),
            "4fbd7e29-9d25-41b8-afd0-5ec00ceff05d" => Ok(Self::CephDMCryptOSD),
            "89c57f98-2fe5-4dc0-89c1-f3ad0ceff2be" => Ok(Self::CephDisk),
            "89c57f98-2fe5-4dc0-89c1-5ec00ceff2be" => Ok(Self::CephDMCryptDisk),
            "cafecafe-9b03-4f30-b4c6-b4b80ceff106" => Ok(Self::CephBlock),
            "30cd0809-c2b2-499c-8879-2d6b78529876" => Ok(Self::CephBlockDB),
            "5ce17fce-4087-4169-b7ff-056cc58473f9" => Ok(Self::CephBlockLog),
            "fb3aabf9-d25f-47cc-bf5e-721d1816496b" => Ok(Self::CephLockbox),
            "4fbd7e29-8ae0-4982-bf9d-5a8d867af560" => Ok(Self::CephMultipathOSD),
            "45b0969e-8ae0-4982-bf9d-5a8d867af560" => Ok(Self::CephMultipathJournal),
            "cafecafe-8ae0-4982-bf9d-5a8d867af560" => Ok(Self::CephMultipathBlock1),
            "7f4a666a-16f3-47a2-8445-152ef4d03f6c" => Ok(Self::CephMultipathBlock2),
            "ec6d6385-e346-45dc-be91-da2a7c8b3261" => Ok(Self::CephMultipathBlockDB),
            "01b41e1b-002a-453c-9f17-88793989ff8f" => Ok(Self::CephMultipathLog),
            "cafecafe-9b03-4f30-b4c6-5ec00ceff106" => Ok(Self::CephDMCryptBlock),
            "93b0052d-02d9-4d8a-a43b-33a3ee4dfbc3" => Ok(Self::CephDMCryptBlockDB),
            "306e8683-4fe2-4330-b7c0-00a917c16966" => Ok(Self::CephDMCryptBlockLog),
            "4fbd7e29-9d25-41b8-afd0-35865ceff05d" => Ok(Self::CephLUKSOSD),
            "45b0969e-9b03-4f30-b4c6-35865ceff106" => Ok(Self::CephLUKSJournal),
            "cafecafe-9b03-4f30-b4c6-35865ceff106" => Ok(Self::CephLUKSBlock),
            "166418da-c469-4022-adf4-b30afd37f176" => Ok(Self::CephLUKSBlockDB),
            "86a32090-3647-40b9-bbbd-38d8c573aa86" => Ok(Self::CephLUKSBlockLog),
            "824cc7a0-36a8-11e3-890a-952519ad3f61" => Ok(Self::OpenBSDData),
            "cef5a9ad-73bc-4601-89f3-cdeeeee321a1" => Ok(Self::QNX6Fs),
            "c91818f9-8025-47af-89d2-f030d7000c2c" => Ok(Self::Plan9),
            "9d275380-40ad-11db-bf97-000c2911d1b8" => Ok(Self::VMWareVMKCORE),
            "aa31e02a-400f-11db-9590-000c2911d1b8" => Ok(Self::VMWareVMFS),
            "9198effc-31c0-11db-8f78-000c2911d1b8" => Ok(Self::VMWareWmkReserved),
            "2568845d-2332-4675-bc39-8fa5a4748d15" => Ok(Self::AndroidBootloader),
            "114eaffe-1552-4022-b26e-9b053604cf84" => Ok(Self::AndroidBootloader2),
            "49a4d17f-93a3-45c1-a0de-f50b2ebe2599" => Ok(Self::AndroidBoot),
            "4177c722-9e92-4aab-8644-43502bfd5506" => Ok(Self::AndroidRecovery),
            "ef32a33b-a409-486c-9141-9ffb711f6266" => Ok(Self::AndroidMisc),
            "20ac26be-20b7-11e3-84c5-6cfdb94711e9" => Ok(Self::AndroidMetadata),
            "38f428e6-d326-425d-9140-6e0ea133647c" => Ok(Self::AndroidSystem),
            "a893ef21-e428-470a-9e55-0668fd91a2d9" => Ok(Self::AndroidCache),
            "dc76dda9-5ac1-491c-af42-a82591580c0d" => Ok(Self::AndroidData),
            "ebc597d0-2053-4b15-8b64-e0aac75f4db1" => Ok(Self::AndroidPersistent),
            "c5a0aeec-13ea-11e5-a1b1-001e67ca0c3c" => Ok(Self::AndroidVendor),
            "bd59408b-4514-490d-bf12-9878d963f378" => Ok(Self::AndroidConfig),
            "8f68cc74-c5e5-48da-be91-a0c8c15e9c80" => Ok(Self::AndroidFactory),
            "9fdaa6ef-4b3f-40d2-ba8d-bff16bfb887b" => Ok(Self::AndroidFactoryAlt),
            "767941d0-2085-11e3-ad3b-6cfdb94711e9" => Ok(Self::AndroidFastboot),
            "ac6d7924-eb71-4df8-b48d-e267b27148ff" => Ok(Self::AndroidOEM),
            "19a710a2-b3ca-11e4-b026-10604b889dcf" => Ok(Self::Android6Meta),
            "193d1ea4-b3ca-11e4-b075-10604b889dcf" => Ok(Self::Android6Ext),
            "7412f7d5-a156-4b13-81dc-867174929325" => Ok(Self::ONIEBoot),
            "d4e6e2cd-4469-46f3-b5cb-1bff57afc149" => Ok(Self::ONIEConfig),
            "9e1a2d38-c612-4316-aa26-8b49521e5a8b" => Ok(Self::PPCPrePBoot),
            // "bc13c2ff-59e6-4262-a352-b275fd6f7172" => Ok(Self::FreeDesktopConfig),
            "734e5afe-f61a-11e6-bc64-92361f002671" => Ok(Self::AtariTOSBasicData),
            "35540011-b055-499f-842d-c69aeca357b7" => Ok(Self::AtariTOSRawData),
            "8c8f8eff-ac95-4770-814a-21994f2dbc8f" => Ok(Self::VeraCryptEncrypted),
            "90b6ff38-b98f-4358-a21f-48f35b4a8ad3" => Ok(Self::OS2ArcaOS),
            "7c5222bd-8f5d-4087-9c00-bf9843c7b58c" => Ok(Self::SPDK),
            "4778ed65-bf42-45fa-9c5b-287a1dc4aab1" => Ok(Self::BareboxState),
            "3de21764-95bd-54bd-a5c3-4abe786f38a8" => Ok(Self::UBootEnv),
            "b6fa30da-92d2-4a9a-96f1-871ec6486200" => Ok(Self::SoftRAIDStatus),
            "2e313465-19b9-463f-8126-8a7993773801" => Ok(Self::SoftRAIDScratch),
            "fa709c7e-65b1-4593-bfd5-e71d61de9b02" => Ok(Self::SoftRAIDVolume),
            "bbba6df5-f46f-4a89-8f59-8765b2727503" => Ok(Self::SoftRAIDCache),
            "fe8a2634-5e2e-46ba-99e3-3a192091a350" => Ok(Self::FuchsiaBoot),
            "d9fd4535-106c-4cec-8d37-dfc020ca87cb" => Ok(Self::FuchsiaSystemData),
            "a409e16b-78aa-4acc-995c-302352621a41" => Ok(Self::FuchsiaBootData),
            "f95d940e-caba-4578-9b93-bb6c90f29d3e" => Ok(Self::FuchsiaFactorySystemData),
            "10b8dbaa-d2bf-42a9-98c6-a7c5db3701e7" => Ok(Self::FuchsiaFactoryBootData),
            "49fd7cb8-df15-4e73-b9d9-992070127f0f" => Ok(Self::FuchsiaVolumeManager),
            "421a8bfc-85d9-4d85-acda-b64eec0133e9" => Ok(Self::FuchsiaVerifiedBoot),
            "9b37fff6-2e58-466a-983a-f7926d0b04e0" => Ok(Self::FuchsiaZirconBoot),
            // "c12a7328-f81f-11d2-ba4b-00a0c93ec93b" => Ok(Self::FuchsiaLegacyESP),
            "606b000b-b7c7-4653-a7d5-b737332c899d" => Ok(Self::FuchsiaLegacySystem),
            "08185f0c-892d-428a-a789-dbeec8f55e6a" => Ok(Self::FuchsiaLegacyData),
            "48435546-4953-2041-494e-5354414c4c52" => Ok(Self::FuchsiaLegacyInstall),
            "2967380e-134c-4cbb-b6da-17e7ce1ca45d" => Ok(Self::FuchsiaLegacyBlob),
            "41d0e340-57e3-954e-8c1e-17ecac44cff5" => Ok(Self::FuchsiaLegacyFVM),
            "de30cc86-1f4a-4a31-93c4-66f147d33e05" => Ok(Self::FuchsiaLegacyZirconBootSlotA),
            "23cc04df-c278-4ce7-8471-897d1a4bcdf7" => Ok(Self::FuchsiaLegacyZirconBootSlotB),
            "a0e5cf57-2def-46be-a80c-a2067c37cd49" => Ok(Self::FuchsiaLegacyZirconBootSlotR),
            "4e5e989e-4c86-11e8-a15b-480fcf35f8e6" => Ok(Self::FuchsiaLegacySysConfig),
            "5a3a90be-4c86-11e8-a15b-480fcf35f8e6" => Ok(Self::FuchsiaLegacyFactoryConfig),
            "5ece94fe-4c86-11e8-a15b-480fcf35f8e6" => Ok(Self::FuchsiaLegacyBoot),
            "8b94d043-30be-4871-9dfa-d69556e8c1f3" => Ok(Self::FuchsiaLegacyGuidTest),
            "a13b4d9a-ec5f-11e8-97d8-6c3be52705bf" => Ok(Self::FuchsiaLegacyVerifiedBootSlotA),
            "a288abf2-ec5f-11e8-97d8-6c3be52705bf" => Ok(Self::FuchsiaLegacyVerifiedBootSlotB),
            "6a2460c3-cd11-4e8b-80a8-12cce268ed0a" => Ok(Self::FuchsiaLegacyVerifiedBootSlotR),
            "1d75395d-f2c6-476b-a8b7-45cc1c97b476" => Ok(Self::FuchsiaLegacyMisc),
            "900b0fc5-90cd-4d4f-84f9-9f8ed579db88" => Ok(Self::FuchsiaLegacyEmmcBoot1),
            "b2b2e8d1-7c10-4ebc-a2d0-4614568260ad" => Ok(Self::FuchsiaLegacyEmmcBoot2),
            "481b2a38-0561-420b-b72a-f1c4988efc16" => Ok(Self::Minix),
            _unsupported => {
                let err_msg = format!("unsupported GUID: {:?}", s);

                Err(ParserError::Guid(err_msg))
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
    #[should_panic(expected = "missing closing double-quote")]
    fn guid_can_not_parse_a_guid_string_with_an_unclosed_double_quote() {
        let _: Guid = r#""082"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn guid_can_not_parse_a_guid_string_with_an_unclosed_quote() {
        let _: Guid = "'082".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "unsupported GUID")]
    fn guid_can_not_parse_an_invalid_guid_string() {
        let _: Guid = "DUMMY".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string slice conversion error")]
    fn guid_can_not_convert_invalid_bytes_into_a_guid() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = Guid::try_from(bytes).unwrap();
    }

    #[test]
    fn guid_can_convert_valid_bytes_into_a_guid() -> crate::Result<()> {
        let bytes: Vec<u8> = b"6a85cf4d-1dd2-11b2-99a6-080020736631".to_vec();
        let actual = Guid::try_from(bytes)?;
        let expected = Guid::SolarisRoot;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn guid_can_parse_a_valid_guid() -> crate::Result<()> {
        let guid_str = "00000000-0000-0000-0000-000000000000";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::EmptyPartition;
        assert_eq!(actual, expected);

        let guid_str = "024dee41-33e7-11d3-9d69-0008c781f39f";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MBRPartition;
        assert_eq!(actual, expected);

        let guid_str = "c12a7328-f81f-11d2-ba4b-00a0c93ec93b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::EfiSystem;
        assert_eq!(actual, expected);

        let guid_str = "21686148-6449-6e6f-744e-656564454649";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::BIOSBoot;
        assert_eq!(actual, expected);

        let guid_str = "d3bfe2de-3daf-11df-ba40-e3a556d89593";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::IntelFastFlash;
        assert_eq!(actual, expected);

        let guid_str = "f4019732-066e-4e12-8273-346c5641494f";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SonyBoot;
        assert_eq!(actual, expected);

        let guid_str = "bfbfafe7-a34f-448a-9a5b-6213eb736c22";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LenovoBoot;
        assert_eq!(actual, expected);

        let guid_str = "e3c9e316-0b5c-4db8-817d-f92df00215ae";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MicrosoftReserved;
        assert_eq!(actual, expected);

        let guid_str = "ebd0a0a2-b9e5-4433-87c0-68b6b72699c7";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::WindowsBasicData;
        assert_eq!(actual, expected);

        let guid_str = "5808c8aa-7e8f-42e0-85d2-e1e90434cfb3";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::WindowsLDM;
        assert_eq!(actual, expected);

        let guid_str = "af9b60a0-1431-4f62-bc68-3311714a69ad";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::WindowsLDMData;
        assert_eq!(actual, expected);

        let guid_str = "de94bba4-06d1-4d40-a16a-bfd50179d6ac";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::WindowsRecovery;
        assert_eq!(actual, expected);

        let guid_str = "37affc90-ef7d-4e96-91c3-2d7ae055b174";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::IBMGPFS;
        assert_eq!(actual, expected);

        let guid_str = "e75caf8f-f680-4cee-afa3-b001e56efc2d";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::WindowsStorageSpaces;
        assert_eq!(actual, expected);

        let guid_str = "558d43c5-a1ac-43c0-aac8-d1472b2923d1";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::WindowsStorageReplica;
        assert_eq!(actual, expected);

        let guid_str = "75894c1e-3aeb-11d3-b7c1-7b03a0000000";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::HPUXData;
        assert_eq!(actual, expected);

        let guid_str = "e2a1e728-32e3-11d6-a682-7b03a0000000";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::HPUXService;
        assert_eq!(actual, expected);

        let guid_str = "0fc63daf-8483-4772-8e79-3d69d8477de4";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxData;
        assert_eq!(actual, expected);

        let guid_str = "a19d880f-05fc-4d3b-a006-743f0f84911e";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRaid;
        assert_eq!(actual, expected);

        let guid_str = "6523f8ae-3eb1-4e2a-a05a-18b695ae656f";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootAlpha;
        assert_eq!(actual, expected);

        let guid_str = "d27f46ed-2919-4cb8-bd25-9531f3c16534";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootARC;
        assert_eq!(actual, expected);

        let guid_str = "69dad710-2ce4-4e3c-b16c-21a1d49abed3";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootARM32;
        assert_eq!(actual, expected);

        let guid_str = "b921b045-1df0-41c3-af44-4c6f280d3fae";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootARM64;
        assert_eq!(actual, expected);

        let guid_str = "993d8d3d-f80e-4225-855a-9daf8ed7ea97";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootIA64;
        assert_eq!(actual, expected);

        let guid_str = "77055800-792c-4f94-b39a-98c91b762bb6";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootLoongArch64;
        assert_eq!(actual, expected);

        let guid_str = "e9434544-6e2c-47cc-bae2-12d6deafb44c";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootMIPS;
        assert_eq!(actual, expected);

        let guid_str = "d113af76-80ef-41b4-bdb6-0cff4d3d4a25";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootMIPS64;
        assert_eq!(actual, expected);

        let guid_str = "37c58c8a-d913-4156-a25f-48b1b64e07f0";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootMIPSEL;
        assert_eq!(actual, expected);

        let guid_str = "700bda43-7a34-4507-b179-eeb93d7a7ca3";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootMIPS64EL;
        assert_eq!(actual, expected);

        let guid_str = "1aacdb3b-5444-4138-bd9e-e5c2239b2346";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootPARISC;
        assert_eq!(actual, expected);

        let guid_str = "1de3f1ef-fa98-47b5-8dcd-4a860a654d78";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootPPC32;
        assert_eq!(actual, expected);

        let guid_str = "912ade1d-a839-4913-8964-a10eee08fbd2";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootPPC64;
        assert_eq!(actual, expected);

        let guid_str = "c31c45e6-3f39-412e-80fb-4809c4980599";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootPPC64LE;
        assert_eq!(actual, expected);

        let guid_str = "60d5a7fe-8e7d-435c-b714-3dd8162144e1";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootRISCV32;
        assert_eq!(actual, expected);

        let guid_str = "72ec70a6-cf74-40e6-bd49-4bda08e8f224";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootRISCV64;
        assert_eq!(actual, expected);

        let guid_str = "08a7acea-624c-4a20-91e8-6e0fa67d23f9";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootS390;
        assert_eq!(actual, expected);

        let guid_str = "5eead9a9-fe09-4a1e-a1d7-520d00531306";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootS390x;
        assert_eq!(actual, expected);

        let guid_str = "c50cdd70-3862-4cc3-90e1-809a8c93ee2c";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootTILEGx;
        assert_eq!(actual, expected);

        let guid_str = "44479540-f297-41b2-9af7-d131d5f0458a";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootx86;
        assert_eq!(actual, expected);

        let guid_str = "4f68bce3-e8cd-4db1-96e7-fbcaf984b709";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootx86_64;
        assert_eq!(actual, expected);

        let guid_str = "e18cf08c-33ec-4c0d-8246-c6c6fb3da024";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrAlpha;
        assert_eq!(actual, expected);

        let guid_str = "7978a683-6316-4922-bbee-38bff5a2fecc";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrARC;
        assert_eq!(actual, expected);

        let guid_str = "7d0359a3-02b3-4f0a-865c-654403e70625";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrARM32;
        assert_eq!(actual, expected);

        let guid_str = "b0e01050-ee5f-4390-949a-9101b17104e9";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrARM64;
        assert_eq!(actual, expected);

        let guid_str = "4301d2a6-4e3b-4b2a-bb94-9e0b2c4225ea";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrIA64;
        assert_eq!(actual, expected);

        let guid_str = "e611c702-575c-4cbe-9a46-434fa0bf7e3f";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrLoongArch64;
        assert_eq!(actual, expected);

        let guid_str = "773b2abc-2a99-4398-8bf5-03baac40d02b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrMIPS;
        assert_eq!(actual, expected);

        let guid_str = "57e13958-7331-4365-8e6e-35eeee17c61b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrMIPS64;
        assert_eq!(actual, expected);

        let guid_str = "0f4868e9-9952-4706-979f-3ed3a473e947";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrMIPSEL;
        assert_eq!(actual, expected);

        let guid_str = "c97c1f32-ba06-40b4-9f22-236061b08aa8";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrMIPS64EL;
        assert_eq!(actual, expected);

        let guid_str = "dc4a4480-6917-4262-a4ec-db9384949f25";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrPARISC;
        assert_eq!(actual, expected);

        let guid_str = "7d14fec5-cc71-415d-9d6c-06bf0b3c3eaf";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrPPC32;
        assert_eq!(actual, expected);

        let guid_str = "2c9739e2-f068-46b3-9fd0-01c5a9afbcca";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrPPC64;
        assert_eq!(actual, expected);

        let guid_str = "15bb03af-77e7-4d4a-b12b-c0d084f7491c";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrPPC64LE;
        assert_eq!(actual, expected);

        let guid_str = "b933fb22-5c3f-4f91-af90-e2bb0fa50702";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrRISCV32;
        assert_eq!(actual, expected);

        let guid_str = "beaec34b-8442-439b-a40b-984381ed097d";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrRISCV64;
        assert_eq!(actual, expected);

        let guid_str = "cd0f869b-d0fb-4ca0-b141-9ea87cc78d66";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrS390;
        assert_eq!(actual, expected);

        let guid_str = "8a4f5770-50aa-4ed3-874a-99b710db6fea";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrS390x;
        assert_eq!(actual, expected);

        let guid_str = "55497029-c7c1-44cc-aa39-815ed1558630";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrTILEGx;
        assert_eq!(actual, expected);

        let guid_str = "75250d76-8cc6-458e-bd66-bd47cc81a812";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrx86;
        assert_eq!(actual, expected);

        let guid_str = "8484680c-9521-48c6-9c11-b0720656f69e";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrx86_64;
        assert_eq!(actual, expected);

        let guid_str = "fc56d9e9-e6e5-4c06-be32-e74407ce09a5";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityAlpha;
        assert_eq!(actual, expected);

        let guid_str = "24b2d975-0f97-4521-afa1-cd531e421b8d";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityARC;
        assert_eq!(actual, expected);

        let guid_str = "7386cdf2-203c-47a9-a498-f2ecce45a2d6";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityARM32;
        assert_eq!(actual, expected);

        let guid_str = "df3300ce-d69f-4c92-978c-9bfb0f38d820";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityARM64;
        assert_eq!(actual, expected);

        let guid_str = "86ed10d5-b607-45bb-8957-d350f23d0571";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityIA64;
        assert_eq!(actual, expected);

        let guid_str = "f3393b22-e9af-4613-a948-9d3bfbd0c535";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityLoongArch64;
        assert_eq!(actual, expected);

        let guid_str = "7a430799-f711-4c7e-8e5b-1d685bd48607";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityMIPS;
        assert_eq!(actual, expected);

        let guid_str = "579536f8-6a33-4055-a95a-df2d5e2c42a8";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityMIPS64;
        assert_eq!(actual, expected);

        let guid_str = "d7d150d2-2a04-4a33-8f12-16651205ff7b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityMIPSEL;
        assert_eq!(actual, expected);

        let guid_str = "16b417f8-3e06-4f57-8dd2-9b5232f41aa6";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityMIPS64EL;
        assert_eq!(actual, expected);

        let guid_str = "d212a430-fbc5-49f9-a983-a7feef2b8d0e";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityPARISC;
        assert_eq!(actual, expected);

        let guid_str = "98cfe649-1588-46dc-b2f0-add147424925";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityPPC32;
        assert_eq!(actual, expected);

        let guid_str = "9225a9a3-3c19-4d89-b4f6-eeff88f17631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityPPC64;
        assert_eq!(actual, expected);

        let guid_str = "906bd944-4589-4aae-a4e4-dd983917446a";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityPPC64LE;
        assert_eq!(actual, expected);

        let guid_str = "ae0253be-1167-4007-ac68-43926c14c5de";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityRISCV32;
        assert_eq!(actual, expected);

        let guid_str = "b6ed5582-440b-4209-b8da-5ff7c419ea3d";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityRISCV64;
        assert_eq!(actual, expected);

        let guid_str = "7ac63b47-b25c-463b-8df8-b4a94e6c90e1";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityS390;
        assert_eq!(actual, expected);

        let guid_str = "b325bfbe-c7be-4ab8-8357-139e652d2f6b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityS390x;
        assert_eq!(actual, expected);

        let guid_str = "966061ec-28e4-4b2e-b4a5-1f0a825a1d84";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityTILEGx;
        assert_eq!(actual, expected);

        let guid_str = "d13c5d3b-b5d1-422a-b29f-9454fdc89d76";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityx86;
        assert_eq!(actual, expected);

        let guid_str = "2c7357ed-ebd2-46d9-aec1-23d437ec2bf5";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVerityx86_64;
        assert_eq!(actual, expected);

        let guid_str = "8cce0d25-c0d0-4a44-bd87-46331bf1df67";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityAlpha;
        assert_eq!(actual, expected);

        let guid_str = "fca0598c-d880-4591-8c16-4eda05c7347c";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityARC;
        assert_eq!(actual, expected);

        let guid_str = "c215d751-7bcd-4649-be90-6627490a4c05";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityARM32;
        assert_eq!(actual, expected);

        let guid_str = "6e11a4e7-fbca-4ded-b9e9-e1a512bb664e";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityARM64;
        assert_eq!(actual, expected);

        let guid_str = "6a491e03-3be7-4545-8e38-83320e0ea880";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityIA64;
        assert_eq!(actual, expected);

        let guid_str = "f46b2c26-59ae-48f0-9106-c50ed47f673d";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityLoongArch64;
        assert_eq!(actual, expected);

        let guid_str = "6e5a1bc8-d223-49b7-bca8-37a5fcceb996";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityMIPS;
        assert_eq!(actual, expected);

        let guid_str = "81cf9d90-7458-4df4-8dcf-c8a3a404f09b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityMIPS64;
        assert_eq!(actual, expected);

        let guid_str = "46b98d8d-b55c-4e8f-aab3-37fca7f80752";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityMIPSEL;
        assert_eq!(actual, expected);

        let guid_str = "3c3d61fe-b5f3-414d-bb71-8739a694a4ef";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityMIPS64EL;
        assert_eq!(actual, expected);

        let guid_str = "5843d618-ec37-48d7-9f12-cea8e08768b2";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityPARISC;
        assert_eq!(actual, expected);

        let guid_str = "df765d00-270e-49e5-bc75-f47bb2118b09";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityPPC32;
        assert_eq!(actual, expected);

        let guid_str = "bdb528a5-a259-475f-a87d-da53fa736a07";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityPPC64;
        assert_eq!(actual, expected);

        let guid_str = "ee2b9983-21e8-4153-86d9-b6901a54d1ce";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityPPC64LE;
        assert_eq!(actual, expected);

        let guid_str = "cb1ee4e3-8cd0-4136-a0a4-aa61a32e8730";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityRISCV32;
        assert_eq!(actual, expected);

        let guid_str = "8f1056be-9b05-47c4-81d6-be53128e5b54";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityRISCV64;
        assert_eq!(actual, expected);

        let guid_str = "b663c618-e7bc-4d6d-90aa-11b756bb1797";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityS390;
        assert_eq!(actual, expected);

        let guid_str = "31741cc4-1a2a-4111-a581-e00b447d2d06";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityS390x;
        assert_eq!(actual, expected);

        let guid_str = "2fb4bf56-07fa-42da-8132-6b139f2026ae";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityTILEGx;
        assert_eq!(actual, expected);

        let guid_str = "8f461b0d-14ee-4e81-9aa9-049b6fb97abd";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityx86;
        assert_eq!(actual, expected);

        let guid_str = "77ff5f63-e7b6-4633-acf4-1565b864c0e6";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVerityx86_64;
        assert_eq!(actual, expected);

        let guid_str = "d46495b7-a053-414f-80f7-700c99921ef8";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigAlpha;
        assert_eq!(actual, expected);

        let guid_str = "143a70ba-cbd3-4f06-919f-6c05683a78bc";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigARC;
        assert_eq!(actual, expected);

        let guid_str = "42b0455f-eb11-491d-98d3-56145ba9d037";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigARM32;
        assert_eq!(actual, expected);

        let guid_str = "6db69de6-29f4-4758-a7a5-962190f00ce3";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigARM64;
        assert_eq!(actual, expected);

        let guid_str = "e98b36ee-32ba-4882-9b12-0ce14655f46a";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigIA64;
        assert_eq!(actual, expected);

        let guid_str = "5afb67eb-ecc8-4f85-ae8e-ac1e7c50e7d0";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigLoongArch64;
        assert_eq!(actual, expected);

        let guid_str = "bba210a2-9c5d-45ee-9e87-ff2ccbd002d0";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigMIPS;
        assert_eq!(actual, expected);

        let guid_str = "43ce94d4-0f3d-4999-8250-b9deafd98e6e";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigMIPS64;
        assert_eq!(actual, expected);

        let guid_str = "c919cc1f-4456-4eff-918c-f75e94525ca5";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigMIPSEL;
        assert_eq!(actual, expected);

        let guid_str = "904e58ef-5c65-4a31-9c57-6af5fc7c5de7";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigMIPS64EL;
        assert_eq!(actual, expected);

        let guid_str = "15de6170-65d3-431c-916e-b0dcd8393f25";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigPARISC;
        assert_eq!(actual, expected);

        let guid_str = "1b31b5aa-add9-463a-b2ed-bd467fc857e7";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigPPC32;
        assert_eq!(actual, expected);

        let guid_str = "f5e2c20c-45b2-4ffa-bce9-2a60737e1aaf";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigPPC64;
        assert_eq!(actual, expected);

        let guid_str = "d4a236e7-e873-4c07-bf1d-bf6cf7f1c3c6";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigPPC64LE;
        assert_eq!(actual, expected);

        let guid_str = "3a112a75-8729-4380-b4cf-764d79934448";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigRISCV32;
        assert_eq!(actual, expected);

        let guid_str = "efe0f087-ea8d-4469-821a-4c2a96a8386a";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigRISCV64;
        assert_eq!(actual, expected);

        let guid_str = "3482388e-4254-435a-a241-766a065f9960";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigS390;
        assert_eq!(actual, expected);

        let guid_str = "c80187a5-73a3-491a-901a-017c3fa953e9";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigS390x;
        assert_eq!(actual, expected);

        let guid_str = "b3671439-97b0-4a53-90f7-2d5a8f3ad47b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigTILEGx;
        assert_eq!(actual, expected);

        let guid_str = "5996fc05-109c-48de-808b-23fa0830b676";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigx86;
        assert_eq!(actual, expected);

        let guid_str = "41092b05-9fc8-4523-994f-2def0408b176";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxRootVeritySigx86_64;
        assert_eq!(actual, expected);

        let guid_str = "5c6e1c76-076a-457a-a0fe-f3b4cd21ce6e";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigAlpha;
        assert_eq!(actual, expected);

        let guid_str = "94f9a9a1-9971-427a-a400-50cb297f0f35";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigARC;
        assert_eq!(actual, expected);

        let guid_str = "d7ff812f-37d1-4902-a810-d76ba57b975a";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigARM32;
        assert_eq!(actual, expected);

        let guid_str = "c23ce4ff-44bd-4b00-b2d4-b41b3419e02a";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigARM64;
        assert_eq!(actual, expected);

        let guid_str = "8de58bc2-2a43-460d-b14e-a76e4a17b47f";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigIA64;
        assert_eq!(actual, expected);

        let guid_str = "b024f315-d330-444c-8461-44bbde524e99";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigLoongArch64;
        assert_eq!(actual, expected);

        let guid_str = "97ae158d-f216-497b-8057-f7f905770f54";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigMIPS;
        assert_eq!(actual, expected);

        let guid_str = "05816ce2-dd40-4ac6-a61d-37d32dc1ba7d";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigMIPS64;
        assert_eq!(actual, expected);

        let guid_str = "3e23ca0b-a4bc-4b4e-8087-5ab6a26aa8a9";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigMIPSEL;
        assert_eq!(actual, expected);

        let guid_str = "f2c2c7ee-adcc-4351-b5c6-ee9816b66e16";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigMIPS64EL;
        assert_eq!(actual, expected);

        let guid_str = "450dd7d1-3224-45ec-9cf2-a43a346d71ee";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigPARISC;
        assert_eq!(actual, expected);

        let guid_str = "7007891d-d371-4a80-86a4-5cb875b9302e";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigPPC32;
        assert_eq!(actual, expected);

        let guid_str = "0b888863-d7f8-4d9e-9766-239fce4d58af";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigPPC64;
        assert_eq!(actual, expected);

        let guid_str = "c8bfbd1e-268e-4521-8bba-bf314c399557";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigPPC64LE;
        assert_eq!(actual, expected);

        let guid_str = "c3836a13-3137-45ba-b583-b16c50fe5eb4";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigRISCV32;
        assert_eq!(actual, expected);

        let guid_str = "d2f9000a-7a18-453f-b5cd-4d32f77a7b32";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigRISCV64;
        assert_eq!(actual, expected);

        let guid_str = "17440e4f-a8d0-467f-a46e-3912ae6ef2c5";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigS390;
        assert_eq!(actual, expected);

        let guid_str = "3f324816-667b-46ae-86ee-9b0c0c6c11b4";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigS390x;
        assert_eq!(actual, expected);

        let guid_str = "4ede75e2-6ccc-4cc8-b9c7-70334b087510";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigTILEGx;
        assert_eq!(actual, expected);

        let guid_str = "974a71c0-de41-43c3-be5d-5c5ccd1ad2c0";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigx86;
        assert_eq!(actual, expected);

        let guid_str = "e7bb33fb-06cf-4e81-8273-e543b413e2e2";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxUsrVeritySigx86_64;
        assert_eq!(actual, expected);

        let guid_str = "bc13c2ff-59e6-4262-a352-b275fd6f7172";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxXBOOTLDR;
        assert_eq!(actual, expected);

        let guid_str = "0657fd6d-a4ab-43c4-84e5-0933c84b4f4f";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxSwap;
        assert_eq!(actual, expected);

        let guid_str = "e6d6d379-f507-44c2-a23c-238f2a3df928";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxLVM;
        assert_eq!(actual, expected);

        let guid_str = "933ac7e1-2eb4-4f13-b844-0e14e2aef915";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxHome;
        assert_eq!(actual, expected);

        let guid_str = "3b8f8425-20e0-4f3b-907f-1a25a76f98e8";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxServerData;
        assert_eq!(actual, expected);

        let guid_str = "773f91ef-66d4-49b5-bd83-d683bf40ad16";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxPerUserHome;
        assert_eq!(actual, expected);

        let guid_str = "7ffec5c9-2d00-49b7-8941-3ea10a5586b7";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxPlain;
        assert_eq!(actual, expected);

        let guid_str = "ca7d7ccb-63ed-4c53-861c-1742536059cc";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxLUKS;
        assert_eq!(actual, expected);

        let guid_str = "8da63339-0007-60c0-c436-083ac8230908";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::LinuxReserved;
        assert_eq!(actual, expected);

        let guid_str = "83bd6b9d-7f41-11dc-be0b-001560b84f0f";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FreeBSDBoot;
        assert_eq!(actual, expected);

        let guid_str = "516e7cb4-6ecf-11d6-8ff8-00022d09712b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FreeBSDDisklabel;
        assert_eq!(actual, expected);

        let guid_str = "516e7cb5-6ecf-11d6-8ff8-00022d09712b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FreeBSDSwap;
        assert_eq!(actual, expected);

        let guid_str = "516e7cb6-6ecf-11d6-8ff8-00022d09712b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FreeBSDUFS;
        assert_eq!(actual, expected);

        let guid_str = "516e7cb8-6ecf-11d6-8ff8-00022d09712b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FreeBSDVinum;
        assert_eq!(actual, expected);

        let guid_str = "516e7cba-6ecf-11d6-8ff8-00022d09712b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FreeBSDZFS;
        assert_eq!(actual, expected);

        let guid_str = "74ba7dd9-a689-11e1-bd04-00e081286acf";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FreeBSDnandfs;
        assert_eq!(actual, expected);

        let guid_str = "48465300-0000-11aa-aa11-00306543ecac";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MacOSHFSPlus;
        assert_eq!(actual, expected);

        let guid_str = "7c3457ef-0000-11aa-aa11-00306543ecac";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MacOSAPFS;
        assert_eq!(actual, expected);

        let guid_str = "55465300-0000-11aa-aa11-00306543ecac";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MacOSUFS;
        assert_eq!(actual, expected);

        let guid_str = "52414944-0000-11aa-aa11-00306543ecac";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MacOSRAID;
        assert_eq!(actual, expected);

        let guid_str = "52414944-5f4f-11aa-aa11-00306543ecac";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MacOSRAIDOffline;
        assert_eq!(actual, expected);

        let guid_str = "426f6f74-0000-11aa-aa11-00306543ecac";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MacOSBootRecovery;
        assert_eq!(actual, expected);

        let guid_str = "4c616265-6c00-11aa-aa11-00306543ecac";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MacOSLabel;
        assert_eq!(actual, expected);

        let guid_str = "5265636f-7665-11aa-aa11-00306543ecac";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MacOSAppleTVRecovery;
        assert_eq!(actual, expected);

        let guid_str = "53746f72-6167-11aa-aa11-00306543ecac";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MacOSHFSPlusFileVault;
        assert_eq!(actual, expected);

        let guid_str = "69646961-6700-11aa-aa11-00306543ecac";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MacOSAPFSPreboot;
        assert_eq!(actual, expected);

        let guid_str = "52637672-7900-11aa-aa11-00306543ecac";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MacOSAPFSRecovery;
        assert_eq!(actual, expected);

        let guid_str = "6a82cb45-1dd2-11b2-99a6-080020736631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SolarisBoot;
        assert_eq!(actual, expected);

        let guid_str = "6a85cf4d-1dd2-11b2-99a6-080020736631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SolarisRoot;
        assert_eq!(actual, expected);

        let guid_str = "6a87c46f-1dd2-11b2-99a6-080020736631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SolarisSwap;
        assert_eq!(actual, expected);

        let guid_str = "6a8b642b-1dd2-11b2-99a6-080020736631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SolarisBackup;
        assert_eq!(actual, expected);

        let guid_str = "6a898cc3-1dd2-11b2-99a6-080020736631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SolarisUsr;
        assert_eq!(actual, expected);

        let guid_str = "6a8ef2e9-1dd2-11b2-99a6-080020736631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SolarisVar;
        assert_eq!(actual, expected);

        let guid_str = "6a90ba39-1dd2-11b2-99a6-080020736631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SolarisHome;
        assert_eq!(actual, expected);

        let guid_str = "6a9283a5-1dd2-11b2-99a6-080020736631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SolarisAlternateSector;
        assert_eq!(actual, expected);

        let guid_str = "6a945a3b-1dd2-11b2-99a6-080020736631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SolarisReserved1;
        assert_eq!(actual, expected);

        let guid_str = "6a9630d1-1dd2-11b2-99a6-080020736631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SolarisReserved2;
        assert_eq!(actual, expected);

        let guid_str = "6a980767-1dd2-11b2-99a6-080020736631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SolarisReserved3;
        assert_eq!(actual, expected);

        let guid_str = "6a96237f-1dd2-11b2-99a6-080020736631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SolarisReserved4;
        assert_eq!(actual, expected);

        let guid_str = "6a8d2ac7-1dd2-11b2-99a6-080020736631";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SolarisReserved5;
        assert_eq!(actual, expected);

        let guid_str = "49f48d32-b10e-11dc-b99b-0019d1879648";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::NetBSDSwap;
        assert_eq!(actual, expected);

        let guid_str = "49f48d5a-b10e-11dc-b99b-0019d1879648";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::NetBSDFFS;
        assert_eq!(actual, expected);

        let guid_str = "49f48d82-b10e-11dc-b99b-0019d1879648";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::NetBSDLFS;
        assert_eq!(actual, expected);

        let guid_str = "49f48daa-b10e-11dc-b99b-0019d1879648";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::NetBSDRAID;
        assert_eq!(actual, expected);

        let guid_str = "2db519c4-b10f-11dc-b99b-0019d1879648";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::NetBSDConcatenated;
        assert_eq!(actual, expected);

        let guid_str = "2db519ec-b10f-11dc-b99b-0019d1879648";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::NetBSDEncrypted;
        assert_eq!(actual, expected);

        let guid_str = "fe3a2a5d-4f32-41a7-b725-accc3285a309";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::ChromeOSKernel;
        assert_eq!(actual, expected);

        let guid_str = "3cb8e202-3b7e-47dd-8a3c-7ff2a13cfcec";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::ChromeOSRootFs;
        assert_eq!(actual, expected);

        let guid_str = "cab6e88e-abf3-4102-a07a-d4bb9be3c1d3";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::ChromeOSFirmware;
        assert_eq!(actual, expected);

        let guid_str = "2e0a753d-9e48-43b0-8337-b15192cb1b5e";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::ChromeOSFuture;
        assert_eq!(actual, expected);

        let guid_str = "09845860-705f-4bb5-b16c-8a8a099caf52";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::ChromeOSMiniOS;
        assert_eq!(actual, expected);

        let guid_str = "3f0f8318-f146-4e6b-8222-c28c8f02e0d5";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::ChromeOSHibernate;
        assert_eq!(actual, expected);

        let guid_str = "5dfbf5f4-2848-4bac-aa5e-0d9a20b745a6";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CoreOSUsr;
        assert_eq!(actual, expected);

        let guid_str = "3884dd41-8582-4404-b9a8-e9b84f2df50e";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CoreOSResize;
        assert_eq!(actual, expected);

        let guid_str = "c95dc21a-df0e-4340-8d7b-26cbfa9a03e0";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CoreOSReserved;
        assert_eq!(actual, expected);

        let guid_str = "be9067b9-ea49-4f15-b4f6-f36f8c9e1818";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CoreOSRootRAID;
        assert_eq!(actual, expected);

        let guid_str = "42465331-3ba3-10f1-802a-4861696b7521";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::HaikuBFS;
        assert_eq!(actual, expected);

        let guid_str = "85d5e45e-237c-11e1-b4b3-e89a8f7fc3a7";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MidnightBSDBoot;
        assert_eq!(actual, expected);

        let guid_str = "85d5e45a-237c-11e1-b4b3-e89a8f7fc3a7";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MidnightBSDData;
        assert_eq!(actual, expected);

        let guid_str = "85d5e45b-237c-11e1-b4b3-e89a8f7fc3a7";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MidnightBSDSwap;
        assert_eq!(actual, expected);

        let guid_str = "0394ef8b-237e-11e1-b4b3-e89a8f7fc3a7";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MidnightBSDUFS;
        assert_eq!(actual, expected);

        let guid_str = "85d5e45c-237c-11e1-b4b3-e89a8f7fc3a7";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MidnightBSDVinum;
        assert_eq!(actual, expected);

        let guid_str = "85d5e45d-237c-11e1-b4b3-e89a8f7fc3a7";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::MidnightBSDZFS;
        assert_eq!(actual, expected);

        let guid_str = "45b0969e-9b03-4f30-b4c6-b4b80ceff106";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephJournal;
        assert_eq!(actual, expected);

        let guid_str = "45b0969e-9b03-4f30-b4c6-5ec00ceff106";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephDMCryptJournal;
        assert_eq!(actual, expected);

        let guid_str = "4fbd7e29-9d25-41b8-afd0-062c0ceff05d";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephOSD;
        assert_eq!(actual, expected);

        let guid_str = "4fbd7e29-9d25-41b8-afd0-5ec00ceff05d";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephDMCryptOSD;
        assert_eq!(actual, expected);

        let guid_str = "89c57f98-2fe5-4dc0-89c1-f3ad0ceff2be";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephDisk;
        assert_eq!(actual, expected);

        let guid_str = "89c57f98-2fe5-4dc0-89c1-5ec00ceff2be";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephDMCryptDisk;
        assert_eq!(actual, expected);

        let guid_str = "cafecafe-9b03-4f30-b4c6-b4b80ceff106";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephBlock;
        assert_eq!(actual, expected);

        let guid_str = "30cd0809-c2b2-499c-8879-2d6b78529876";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephBlockDB;
        assert_eq!(actual, expected);

        let guid_str = "5ce17fce-4087-4169-b7ff-056cc58473f9";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephBlockLog;
        assert_eq!(actual, expected);

        let guid_str = "fb3aabf9-d25f-47cc-bf5e-721d1816496b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephLockbox;
        assert_eq!(actual, expected);

        let guid_str = "4fbd7e29-8ae0-4982-bf9d-5a8d867af560";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephMultipathOSD;
        assert_eq!(actual, expected);

        let guid_str = "45b0969e-8ae0-4982-bf9d-5a8d867af560";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephMultipathJournal;
        assert_eq!(actual, expected);

        let guid_str = "cafecafe-8ae0-4982-bf9d-5a8d867af560";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephMultipathBlock1;
        assert_eq!(actual, expected);

        let guid_str = "7f4a666a-16f3-47a2-8445-152ef4d03f6c";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephMultipathBlock2;
        assert_eq!(actual, expected);

        let guid_str = "ec6d6385-e346-45dc-be91-da2a7c8b3261";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephMultipathBlockDB;
        assert_eq!(actual, expected);

        let guid_str = "01b41e1b-002a-453c-9f17-88793989ff8f";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephMultipathLog;
        assert_eq!(actual, expected);

        let guid_str = "cafecafe-9b03-4f30-b4c6-5ec00ceff106";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephDMCryptBlock;
        assert_eq!(actual, expected);

        let guid_str = "93b0052d-02d9-4d8a-a43b-33a3ee4dfbc3";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephDMCryptBlockDB;
        assert_eq!(actual, expected);

        let guid_str = "306e8683-4fe2-4330-b7c0-00a917c16966";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephDMCryptBlockLog;
        assert_eq!(actual, expected);

        let guid_str = "4fbd7e29-9d25-41b8-afd0-35865ceff05d";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephLUKSOSD;
        assert_eq!(actual, expected);

        let guid_str = "45b0969e-9b03-4f30-b4c6-35865ceff106";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephLUKSJournal;
        assert_eq!(actual, expected);

        let guid_str = "cafecafe-9b03-4f30-b4c6-35865ceff106";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephLUKSBlock;
        assert_eq!(actual, expected);

        let guid_str = "166418da-c469-4022-adf4-b30afd37f176";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephLUKSBlockDB;
        assert_eq!(actual, expected);

        let guid_str = "86a32090-3647-40b9-bbbd-38d8c573aa86";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::CephLUKSBlockLog;
        assert_eq!(actual, expected);

        let guid_str = "824cc7a0-36a8-11e3-890a-952519ad3f61";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::OpenBSDData;
        assert_eq!(actual, expected);

        let guid_str = "cef5a9ad-73bc-4601-89f3-cdeeeee321a1";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::QNX6Fs;
        assert_eq!(actual, expected);

        let guid_str = "c91818f9-8025-47af-89d2-f030d7000c2c";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::Plan9;
        assert_eq!(actual, expected);

        let guid_str = "9d275380-40ad-11db-bf97-000c2911d1b8";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::VMWareVMKCORE;
        assert_eq!(actual, expected);

        let guid_str = "aa31e02a-400f-11db-9590-000c2911d1b8";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::VMWareVMFS;
        assert_eq!(actual, expected);

        let guid_str = "9198effc-31c0-11db-8f78-000c2911d1b8";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::VMWareWmkReserved;
        assert_eq!(actual, expected);

        let guid_str = "2568845d-2332-4675-bc39-8fa5a4748d15";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidBootloader;
        assert_eq!(actual, expected);

        let guid_str = "114eaffe-1552-4022-b26e-9b053604cf84";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidBootloader2;
        assert_eq!(actual, expected);

        let guid_str = "49a4d17f-93a3-45c1-a0de-f50b2ebe2599";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidBoot;
        assert_eq!(actual, expected);

        let guid_str = "4177c722-9e92-4aab-8644-43502bfd5506";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidRecovery;
        assert_eq!(actual, expected);

        let guid_str = "ef32a33b-a409-486c-9141-9ffb711f6266";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidMisc;
        assert_eq!(actual, expected);

        let guid_str = "20ac26be-20b7-11e3-84c5-6cfdb94711e9";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidMetadata;
        assert_eq!(actual, expected);

        let guid_str = "38f428e6-d326-425d-9140-6e0ea133647c";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidSystem;
        assert_eq!(actual, expected);

        let guid_str = "a893ef21-e428-470a-9e55-0668fd91a2d9";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidCache;
        assert_eq!(actual, expected);

        let guid_str = "dc76dda9-5ac1-491c-af42-a82591580c0d";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidData;
        assert_eq!(actual, expected);

        let guid_str = "ebc597d0-2053-4b15-8b64-e0aac75f4db1";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidPersistent;
        assert_eq!(actual, expected);

        let guid_str = "c5a0aeec-13ea-11e5-a1b1-001e67ca0c3c";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidVendor;
        assert_eq!(actual, expected);

        let guid_str = "bd59408b-4514-490d-bf12-9878d963f378";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidConfig;
        assert_eq!(actual, expected);

        let guid_str = "8f68cc74-c5e5-48da-be91-a0c8c15e9c80";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidFactory;
        assert_eq!(actual, expected);

        let guid_str = "9fdaa6ef-4b3f-40d2-ba8d-bff16bfb887b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidFactoryAlt;
        assert_eq!(actual, expected);

        let guid_str = "767941d0-2085-11e3-ad3b-6cfdb94711e9";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidFastboot;
        assert_eq!(actual, expected);

        let guid_str = "ac6d7924-eb71-4df8-b48d-e267b27148ff";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AndroidOEM;
        assert_eq!(actual, expected);

        let guid_str = "19a710a2-b3ca-11e4-b026-10604b889dcf";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::Android6Meta;
        assert_eq!(actual, expected);

        let guid_str = "193d1ea4-b3ca-11e4-b075-10604b889dcf";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::Android6Ext;
        assert_eq!(actual, expected);

        let guid_str = "7412f7d5-a156-4b13-81dc-867174929325";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::ONIEBoot;
        assert_eq!(actual, expected);

        let guid_str = "d4e6e2cd-4469-46f3-b5cb-1bff57afc149";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::ONIEConfig;
        assert_eq!(actual, expected);

        let guid_str = "9e1a2d38-c612-4316-aa26-8b49521e5a8b";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::PPCPrePBoot;
        assert_eq!(actual, expected);

        let guid_str = "734e5afe-f61a-11e6-bc64-92361f002671";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AtariTOSBasicData;
        assert_eq!(actual, expected);

        let guid_str = "35540011-b055-499f-842d-c69aeca357b7";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::AtariTOSRawData;
        assert_eq!(actual, expected);

        let guid_str = "8c8f8eff-ac95-4770-814a-21994f2dbc8f";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::VeraCryptEncrypted;
        assert_eq!(actual, expected);

        let guid_str = "90b6ff38-b98f-4358-a21f-48f35b4a8ad3";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::OS2ArcaOS;
        assert_eq!(actual, expected);

        let guid_str = "7c5222bd-8f5d-4087-9c00-bf9843c7b58c";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SPDK;
        assert_eq!(actual, expected);

        let guid_str = "4778ed65-bf42-45fa-9c5b-287a1dc4aab1";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::BareboxState;
        assert_eq!(actual, expected);

        let guid_str = "3de21764-95bd-54bd-a5c3-4abe786f38a8";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::UBootEnv;
        assert_eq!(actual, expected);

        let guid_str = "b6fa30da-92d2-4a9a-96f1-871ec6486200";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SoftRAIDStatus;
        assert_eq!(actual, expected);

        let guid_str = "2e313465-19b9-463f-8126-8a7993773801";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SoftRAIDScratch;
        assert_eq!(actual, expected);

        let guid_str = "fa709c7e-65b1-4593-bfd5-e71d61de9b02";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SoftRAIDVolume;
        assert_eq!(actual, expected);

        let guid_str = "bbba6df5-f46f-4a89-8f59-8765b2727503";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::SoftRAIDCache;
        assert_eq!(actual, expected);

        let guid_str = "fe8a2634-5e2e-46ba-99e3-3a192091a350";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaBoot;
        assert_eq!(actual, expected);

        let guid_str = "d9fd4535-106c-4cec-8d37-dfc020ca87cb";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaSystemData;
        assert_eq!(actual, expected);

        let guid_str = "a409e16b-78aa-4acc-995c-302352621a41";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaBootData;
        assert_eq!(actual, expected);

        let guid_str = "f95d940e-caba-4578-9b93-bb6c90f29d3e";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaFactorySystemData;
        assert_eq!(actual, expected);

        let guid_str = "10b8dbaa-d2bf-42a9-98c6-a7c5db3701e7";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaFactoryBootData;
        assert_eq!(actual, expected);

        let guid_str = "49fd7cb8-df15-4e73-b9d9-992070127f0f";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaVolumeManager;
        assert_eq!(actual, expected);

        let guid_str = "421a8bfc-85d9-4d85-acda-b64eec0133e9";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaVerifiedBoot;
        assert_eq!(actual, expected);

        let guid_str = "9b37fff6-2e58-466a-983a-f7926d0b04e0";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaZirconBoot;
        assert_eq!(actual, expected);

        let guid_str = "606b000b-b7c7-4653-a7d5-b737332c899d";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacySystem;
        assert_eq!(actual, expected);

        let guid_str = "08185f0c-892d-428a-a789-dbeec8f55e6a";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyData;
        assert_eq!(actual, expected);

        let guid_str = "48435546-4953-2041-494e-5354414c4c52";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyInstall;
        assert_eq!(actual, expected);

        let guid_str = "2967380e-134c-4cbb-b6da-17e7ce1ca45d";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyBlob;
        assert_eq!(actual, expected);

        let guid_str = "41d0e340-57e3-954e-8c1e-17ecac44cff5";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyFVM;
        assert_eq!(actual, expected);

        let guid_str = "de30cc86-1f4a-4a31-93c4-66f147d33e05";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyZirconBootSlotA;
        assert_eq!(actual, expected);

        let guid_str = "23cc04df-c278-4ce7-8471-897d1a4bcdf7";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyZirconBootSlotB;
        assert_eq!(actual, expected);

        let guid_str = "a0e5cf57-2def-46be-a80c-a2067c37cd49";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyZirconBootSlotR;
        assert_eq!(actual, expected);

        let guid_str = "4e5e989e-4c86-11e8-a15b-480fcf35f8e6";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacySysConfig;
        assert_eq!(actual, expected);

        let guid_str = "5a3a90be-4c86-11e8-a15b-480fcf35f8e6";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyFactoryConfig;
        assert_eq!(actual, expected);

        let guid_str = "5ece94fe-4c86-11e8-a15b-480fcf35f8e6";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyBoot;
        assert_eq!(actual, expected);

        let guid_str = "8b94d043-30be-4871-9dfa-d69556e8c1f3";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyGuidTest;
        assert_eq!(actual, expected);

        let guid_str = "a13b4d9a-ec5f-11e8-97d8-6c3be52705bf";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyVerifiedBootSlotA;
        assert_eq!(actual, expected);

        let guid_str = "a288abf2-ec5f-11e8-97d8-6c3be52705bf";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyVerifiedBootSlotB;
        assert_eq!(actual, expected);

        let guid_str = "6a2460c3-cd11-4e8b-80a8-12cce268ed0a";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyVerifiedBootSlotR;
        assert_eq!(actual, expected);

        let guid_str = "1d75395d-f2c6-476b-a8b7-45cc1c97b476";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyMisc;
        assert_eq!(actual, expected);

        let guid_str = "900b0fc5-90cd-4d4f-84f9-9f8ed579db88";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyEmmcBoot1;
        assert_eq!(actual, expected);

        let guid_str = "b2b2e8d1-7c10-4ebc-a2d0-4614568260ad";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::FuchsiaLegacyEmmcBoot2;
        assert_eq!(actual, expected);

        let guid_str = "481b2a38-0561-420b-b72a-f1c4988efc16";
        let actual: Guid = guid_str.parse()?;
        let expected = Guid::Minix;
        assert_eq!(actual, expected);

        Ok(())
    }
}
