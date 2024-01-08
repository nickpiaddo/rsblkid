# Web snapshots

----

Snapshots of helpful website, blog, and documentation pages used while building
this project.

A bookmark is useful, but you never know when a site will go dark forever.
Having a local copy is a good remedy to having information disappear overnight.

These tools were essential for collecting copies:

- [Monolith](https://github.com/Y2Z/monolith)
- [SingleFile](https://addons.mozilla.org/en-US/firefox/addon/single-file/)

----

`[a]`link to the version saved in this repository.

## Alpine Linux

[Managing repositories][1] [[a]][2]
[How to enable automatic login in alpine linux for root][3] [[a]][4]

## Linux Kernel

[List available modules][5] [[a]][6]

## QEMU

[QEMU Machine Protocol][7] [[a]][8]

## NixOS

[C header includes in NixOS][9] [[a]][10]
[NixOS - Environment variables][11] [[a]][12]

## Rust

[Using C Libraries in Rust][13] [[a]][14]
[Wrapping Unsafe C Libraries in Rust][15] [[a]][16]
[Rust Conversions][37] [[a]][38]
[How to build a Rust API with the builder pattern][39] [[a]][40]
[Hand-Implementing PartialEq, Eq, Hash, PartialOrd and Ord in Rust][41] [[a]][42]
[How to combine two cmp conditions in `Ord::cmp()`][43] [[a]][44]

## File systems

[Partition type GUIDs][17] [[a]][18]
[Design of the FAT file system][19] [[a]][20]
[Journaling file system][21] [[a]][22]
[Logical block addressing][23] [[a]][24]
[What is a Superblock, Inode, Dentry and a File?][45] [[a]][46]
[What's a file system's "magic" number in a super block?][47] [[a]][48]

## Disks

[Cylinder-head-sector][25] [[a]][26]

## Standards

[GUID Partition Table (GPT) Disk Layout][27] [[a]][28]
[Volume and File Structure of CDROM for Information Interchange][29] [[a]][30]
[List of partition identifiers for PCs][31] [[a]][32]
[Stratis Software Design][33] [[a]][34]
[LUKS2 On-Disk Format Specification][35] [[a]][36]

[1]: https://wiki.alpinelinux.org/wiki/Repositories#Managing_repositories
[2]: alpine-linux/managing-repositories.html
[3]: https://unix.stackexchange.com/questions/751105/how-to-enable-automatic-login-in-alpine-linux-for-root
[4]: alpine-linux/automatic-login-in-alpine-linux-for-root.html
[5]: https://wiki.gentoo.org/wiki/Kernel_Modules#List_available_modules
[6]: linux-kernel/kernel-modules.html
[7]: https://wiki.qemu.org/Documentation/QMP#By_hand
[8]: qemu/qemu-machine-protocol.html
[9]: https://discourse.nixos.org/t/c-header-includes-in-nixos/17410
[10]: nixos/c-header-includes.html
[11]: https://nixos.wiki/wiki/Environment_variables
[12]: nixos/environment-variables.html
[13]: https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a
[14]: rust/using-c-libraries-in-rust.html
[15]: https://medium.com/dwelo-r-d/wrapping-unsafe-c-libraries-in-rust-d75aeb283c65
[16]: rust/wrapping-unsafe-c-libraries.html
[17]: https://en.wikipedia.org/wiki/GUID_Partition_Table#Partition_type_GUIDs
[18]: fs/GPT-partition-type-guid.html
[19]: https://en.wikipedia.org/wiki/Design_of_the_FAT_file_system#Boot_Sector
[20]: fs/design-of-the-fat-file-system.html
[21]: https://en.wikipedia.org/wiki/Journaling_file_system
[22]: web-snapshots/fs/journaling-file-system.html
[23]: https://en.wikipedia.org/wiki/Logical_block_addressing
[24]: web-snapshots/fs/logical-block-addressing.html
[25]: https://en.wikipedia.org/wiki/Cylinder-head-sector
[26]: disk/cylinder-head-sector-addressing.html
[27]: https://uefi.org/specs/UEFI/2.10/05_GUID_Partition_Table_Format.html
[28]: standards/GPT-MBR-partition-table-format.html
[29]: https://ecma-international.org/wp-content/uploads/ECMA-119_3rd_edition_december_2017.pdf
[30]: standards/ISO9660-ECMA-119-3rd-edition-december-2017.pdf
[31]: https://www.win.tue.nl/~aeb/partitions/partition_types-1.html
[32]: web-snapshots/standards/MBR-partition-types-list-of-partition-identifiers.html
[33]: https://stratis-storage.github.io/StratisSoftwareDesign.pdf
[34]: web-snapshots/standards/StratisSoftwareDesign.pdf
[35]: https://fossies.org/linux/cryptsetup/docs/on-disk-format-luks2.pdf
[36]: web-snapshots/standards/on-disk-format-luks2.pdf
[37]: https://nicholasbishop.github.io/rust-conversions/
[38]: rust/rust-conversions.html
[39]: https://blog.logrocket.com/build-rust-api-builder-pattern/
[40]: rust/build-a-rust-api-with-the-builder-pattern.html
[41]: https://www.philipdaniels.com/blog/2019/rust-equality-and-ordering/
[42]: rust/rust-equality-and-ordering.html
[43]: https://stackoverflow.com/questions/67335967/how-to-combine-two-cmp-conditions-in-ordcmp
[44]: rust/how-to-combine-two-orderings.html
[45]: https://unix.stackexchange.com/questions/4402/what-is-a-superblock-inode-dentry-and-a-file
[46]: filesystems/what-is-a-superblock.html
[47]: https://superuser.com/questions/239088/whats-a-file-systems-magic-number-in-a-super-block
[48]: filesystems/what-is-a-filesystem-s-magic-number.html
