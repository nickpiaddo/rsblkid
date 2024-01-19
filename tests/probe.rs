// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod common;

static INIT: std::sync::Once = std::sync::Once::new();

generate_filesystem_tests![
    "adaptec-raid",
    "apfs",
    "bcache-B",
    "bcache-C",
    "bcache-journal",
    "bcachefs-2",
    "bcachefs",
    "befs",
    "bfs",
    "bluestore",
    "btrfs",
    "cramfs-big",
    "cramfs",
    "cs_fvault2",
    "ddf-raid",
    "drbd-v08",
    "drbd-v09",
    "drbdmanage-control-volume",
    "erofs",
    "exfat",
    "ext2",
    "ext3",
    "ext4",
    "f2fs",
    "fat",
    "fat16_noheads",
    "fat32_cp850_O_tilde",
    "fat32_label_64MB",
    "fat32_mkdosfs_label1",
    "fat32_mkdosfs_label1_dosfslabel_empty",
    "fat32_mkdosfs_label1_dosfslabel_label2",
    "fat32_mkdosfs_label1_dosfslabel_NO_NAME",
    "fat32_mkdosfs_label1_mlabel_erase",
    "fat32_mkdosfs_label1_mlabel_NO_NAME",
    "fat32_mkdosfs_label1_xp_erase",
    "fat32_mkdosfs_label1_xp_label2",
    "fat32_mkdosfs_none",
    "fat32_mkdosfs_none_dosfslabel_label1",
    "fat32_mkdosfs_none_dosfslabel_label1_xp_label2",
    "fat32_mkdosfs_none_dosfslabel_NO_NAME",
    "fat32_mkdosfs_none_xp_label1",
    "fat32_mkdosfs_none_xp_label1_dosfslabel_label2",
    "fat32_xp_label1",
    "fat32_xp_none",
    "fat32_xp_none_dosfslabel_label1",
    "fat32_xp_none_mlabel_label1",
    "gfs2",
    "hfs",
    "hfsplus",
    "hpfs",
    "hpt37x-raid",
    "hpt45x-raid",
    "iso-different-iso-joliet-label",
    "iso-joliet",
    "iso-rr-joliet",
    "iso-unicode-long-label",
    "iso",
    "isw-raid",
    "jbd",
    "jfs",
    "jmicron-raid",
    "lsi-raid",
    "luks1",
    "luks2",
    "lvm2",
    "mdraid-1",
    "mdraid",
    "minix-BE",
    "minix-LE",
    "mpool",
    "netware",
    "nilfs2",
    "ntfs",
    "nvidia-raid",
    "ocfs2",
    "promise-raid",
    "reiser3",
    "reiser4",
    "romfs",
    "silicon-raid",
    "small-fat32",
    "squashfs3",
    "squashfs4",
    "swap0",
    "swap1-big",
    "swap1",
    "tuxonice",
    "ubi",
    "ubifs",
    "udf-bdr-2_60-nero",
    "udf-cd-mkudfiso-20100208",
    "udf-cd-nero-6",
    "udf-hdd-macosx-2_60-4096",
    "udf-hdd-mkudffs-1_0_0-1",
    "udf-hdd-mkudffs-1_0_0-2",
    "udf-hdd-mkudffs-1_3-1",
    "udf-hdd-mkudffs-1_3-2",
    "udf-hdd-mkudffs-1_3-3",
    "udf-hdd-mkudffs-1_3-4",
    "udf-hdd-mkudffs-1_3-5",
    "udf-hdd-mkudffs-1_3-6",
    "udf-hdd-mkudffs-1_3-7",
    "udf-hdd-mkudffs-1_3-8",
    "udf-hdd-mkudffs-2_2",
    "udf-hdd-udfclient-0_7_5",
    "udf-hdd-udfclient-0_7_7",
    "udf-hdd-win7",
    "udf",
    "ufs",
    "vdo",
    "via-raid",
    "vmfs",
    "vmfs_volume",
    "xfs-log",
    "xfs-v5",
    "xfs",
    "zfs",
    "zonefs"
];

// Tests for ISO, and Universal Disk Format (UDF) images with multi-session mastering.
// Providing a "session_offset" hint allows a Probe to determine which location to scan.
//
// From source file https://github.com/util-linux/util-linux/blob/stable/v2.39/tests/ts/blkid/low-probe
//    #
//    # multi session images, the image name contains "-multi-" and all
//    # -<numbers>- are interpreted as offset [in sectors] to the sessions. The offset is
//    # calculated in 2048[-byte] sectors.  For example: iso-multi-0-174-348-genisoimage.img
//    #

// iso-multi-0-174-348-genisoimage
gen_fs_test!(
    "iso-multi-genisoimage-0",
    "iso-multi-0-174-348-genisoimage",
    "session_offset",
    0 * 2048
);
gen_fs_test!(
    "iso-multi-genisoimage-174",
    "iso-multi-0-174-348-genisoimage",
    "session_offset",
    174 * 2048
);
gen_fs_test!(
    "iso-multi-genisoimage-348",
    "iso-multi-0-174-348-genisoimage",
    "session_offset",
    348 * 2048
);

//"udf-multi-0-417-834-genisoimage",
gen_fs_test!(
    "udf-multi-genisoimage-0",
    "udf-multi-0-417-834-genisoimage",
    "session_offset",
    0 * 2048
);
gen_fs_test!(
    "udf-multi-genisoimage-417",
    "udf-multi-0-417-834-genisoimage",
    "session_offset",
    417 * 2048
);
gen_fs_test!(
    "udf-multi-genisoimage-834",
    "udf-multi-0-417-834-genisoimage",
    "session_offset",
    834 * 2048
);

// udf-multi-0-320-640-mkudffs
gen_fs_test!(
    "udf-multi-mkudffs-0",
    "udf-multi-0-320-640-mkudffs",
    "session_offset",
    0 * 2048
);
gen_fs_test!(
    "udf-multi-mkudffs-320",
    "udf-multi-0-320-640-mkudffs",
    "session_offset",
    320 * 2048
);
gen_fs_test!(
    "udf-multi-mkudffs-640",
    "udf-multi-0-320-640-mkudffs",
    "session_offset",
    640 * 2048
);

generate_partition_table_tests![
    "atari-primary",
    "atari-xgm",
    "bsd",
    "dos_bsd",
    "gpt",
    "sgi",
    "sun"
];
