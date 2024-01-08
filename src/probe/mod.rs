// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg_attr(doc,
    cfg_attr(all(),
        doc = ::embed_doc_image::embed_image!( "fig-01", "assets/diagrams/svg/fig01-probe-metadata-scanning-process.svg"),))]
//! Low-level API to probe block devices.
//!
//! ----
//! # Table of Contents
//! 1. [Description](#description)
//! 2. [Overview](#overview)
//!
//! ## Description
//!
//! The `probe` module offers fine-grained tools from three categories to collect, analyse, and
//! eventually alter data about block devices:
//! - `superblocks`: for file system properties,
//! - `partitions`: for partition description,
//! - `topology`: for sector size, optimal I/O size, device capabilities, etc.
//!
//! The `Probe` struct is the main entry-point of this module. It centralizes all module functionalities.
//!
//! ## Overview
//!
//! Unlike a [`Cache`](crate::cache::Cache), a low-level `Probe` reads data primarily from a
//! block device assigned to it at construction. This block device can be, for example:
//! - a whole disk (e.g.  `/dev/sda`)
//! - a disk partition (e.g. `/dev/sda1`)
//! - or an image file.
//!
//! To gather information, a `Probe` tries to identify any disk topology, file system, or partition
//! present on a block device. For each category mentioned, a `Probe` uses a chain of search
//! functions to detect and collect relevant data; search functions are tried in succession until
//! one matches, as described in the flowchart below.
//!
//! For example, to determine which file system a disk uses, a `Probe` will try to find a unique
//! identifier (magic number) in the device `superblocks`.
//!
//! > "The **superblock** is essentially file system metadata and defines the file system type, size, status,
//! > and information about other metadata structures (metadata of metadata). The superblock is very
//! > critical to the file system and therefore is stored in multiple redundant copies for each file
//! > system. The superblock is a very "high level" metadata structure for the file system. For example,
//! > if the superblock of a partition, `/var`, becomes corrupt then the file system in question
//! > (`/var`) cannot be mounted by the operating system. Commonly in this event, you need to run
//! > `fsck` which will automatically select an alternate, backup copy of the superblock and attempt
//! > to recover the file system".
//!
//! Source: [StackExchange - What is a Superblock, Inode, Dentry and a File?](https://unix.stackexchange.com/a/4403)
//!
//! If a magic number matches one in the list of supported file systems, the `Probe` will use a
//! specialised function to extract file system properties requested by the user (e.g. `LABEL`,
//! `UUID`, etc.). If asked, the `Probe` will then automatically switch to searching data for other
//! categories, i.e. `partitions` and `topology`, applying the same process.
//!
//! ![Flowchart of a Probe's data gathering process][fig-01]
//!
//! In the flowchart above, going from the starting point at the top, the first step in the process
//! is to determine whether the user requested a file system scan at the *Scan for file systems?*
//! node.
//!
//! If the answer is yes, the program enters the box titled `File system scanner` proceeding down
//! a decision tree. At each node, the program tests for the presence of a particular file system
//! on the device associated with the `Probe`.
//!
//! In the example flowchart, it will check for the presence of an `APFS` file system:
//! - if the test is successful, we transition to the node titled `Collect file system properties`
//! to gather data on the file system. We then exit the `File system scanner`, and move to the
//! decision node titled `Scan for partitions?` and check whether the user asked the program to
//! scan the device for partitions.
//! - if however the first file system test fails, we move to the next test, this time for a `BFS`
//! file system.
//! - if this second test succeeds we proceed to the `Collect file system properties` node,
//! followed by exiting the `File system scanner` and going to the `Scan for partitions?` node.
//! - this routine is repeated for every file system test in the `File system scanner`. If none
//! matches we transition to the decision node on partition scanning.
//!
//! Once the scan for file systems is concluded, we determine whether the program need to scan for
//! partitions.
//!
//! Going from the `Scan for partitions?` node:
//! - if the check succeeds we transition to the decision tree in the `Partitions scanner` box. We
//! first try to identify the type of partition table used on the device at the `Has an AIX
//! partition table?` decision node.
//! - if that is the case, we go to the `Collect per-partition properties` node, then exit the
//! `Partition scanner?` box to head to the `Extract device topology?` decision node.
//! - if an `AIX` partition table is not found, we move to the next test `Has a DOS partition
//! table?`, and so on, and so forth until one matches.
//! - if no test succeeds, we exit the test chain and go to the `Extract device topology?` node.
//!
//! Finally, at the `Extract device topology?` decision node, if the `Probe` was configured to do
//! so, we move to the `Topology scanner` box which only contains a `Collect device topology data`
//! node. After gathering all the information available, we reach the end of the collection process.
//!
//! If however, the user does not want data on the device's topology we go to the `End` node.
