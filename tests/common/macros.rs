// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[macro_export]
macro_rules! generate_filesystem_tests {
    [ $($fs_name:literal),* ] => {
        $( gen_fs_test!($fs_name, $fs_name, "session_offset", 0); )*
    };
}

#[macro_export]
macro_rules! generate_partition_table_tests {
    [ $($pt_name:literal),* ] => {
        $( gen_pt_test!($pt_name); )*
    };
}

#[macro_export]
macro_rules! gen_fs_test {
    ( $fs_name:literal, $image_file_name:literal, $hint_name:literal, $hint_value:expr ) => {
        paste::paste! {
            #[test]
            fn [< probe_file_system_ $fs_name:lower >]() {
                use std::io::Read;

                // Initialize debug output
                INIT.call_once(|| {
                    // rsblkid debug messages
                    env_logger::init();
                    // libblkid debug messages
                    rsblkid::debug::init_default_debug();
                });

                let base_dir: &'static str = env!("CARGO_MANIFEST_DIR");

                let mut blkid_dir = std::path::PathBuf::new();
                blkid_dir.push(base_dir);
                blkid_dir.push("third-party/vendor/util-linux/blkid/");

                // Construct device image path
                let mut compressed_image_file_path = blkid_dir.clone();
                let fragment = concat!("images/filesystems/", $image_file_name, ".img.xz");
                compressed_image_file_path.push(fragment);

                // Copy decompressed device image to temporary file
                let compressed_image_file = std::fs::File::open(compressed_image_file_path).unwrap();
                let mut decompressed = xz2::read::XzDecoder::new(compressed_image_file);

                let mut temp_image_file = tempfile::NamedTempFile::new().unwrap();
                std::io::copy(&mut decompressed, &mut temp_image_file).unwrap();

                // Probe device
                let image = temp_image_file.into_file();

                let mut probe = rsblkid::probe::Probe::builder()
                    .scan_file(image)
                    .scan_device_superblocks(true)
                    .collect_fs_properties(vec![
                        rsblkid::probe::FsProperty::Label,
                        rsblkid::probe::FsProperty::SecondType,
                        rsblkid::probe::FsProperty::Type,
                        rsblkid::probe::FsProperty::Usage,
                        rsblkid::probe::FsProperty::Uuid,
                        rsblkid::probe::FsProperty::Version,
                        rsblkid::probe::FsProperty::FsInfo,
                    ])
                    .build()
                    .unwrap();

                let io_hint = rsblkid::probe::IoHint::from(($hint_name, $hint_value));
                probe.set_hint(&io_hint).unwrap();
                probe.find_device_properties();

                // Collect device properties
                let kv = probe.iter_device_properties().collect::<Vec<_>>();

                let mut kv_str = kv
                    .iter()
                    .map(|kv| kv.to_udev_format())
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
                kv_str.sort();

                let mut output = kv_str.join("\n");
                output.push('\n');

                // Construct path to expected key/value pair output
                let mut expected_file = blkid_dir.clone();
                let fragment = concat!("expected/filesystems/", "low-probe-", $fs_name);
                expected_file.push(fragment);

                // Read expected output from file
                let mut f = std::fs::File::open(expected_file).unwrap();
                let mut expected = String::new();
                f.read_to_string(&mut expected).unwrap();

                pretty_assertions::assert_eq!(output, expected, "comparing scanned device properties to expected values");
            }
        }
    };
}

#[macro_export]
macro_rules! gen_pt_test {
    [ $pt_name:literal ] => {
        paste::paste! {
            #[test]
            fn [< probe_partition_table_ $pt_name:lower >]() {
                use std::io::Read;

                // Initialize debug output
                INIT.call_once(|| {
                    // rsblkid debug messages
                    env_logger::init();
                    // libblkid debug messages
                    rsblkid::debug::init_default_debug();
                });

                let base_dir: &'static str = env!("CARGO_MANIFEST_DIR");

                let mut blkid_dir = std::path::PathBuf::new();
                blkid_dir.push(base_dir);
                blkid_dir.push("third-party/vendor/util-linux/blkid/");

                // Construct device image path
                let mut compressed_image_file_path = blkid_dir.clone();
                let fragment = concat!("images/partition_tables/", $pt_name, ".img.xz");
                compressed_image_file_path.push(fragment);

                // Copy decompressed device image to temporary file
                let compressed_image_file = std::fs::File::open(&compressed_image_file_path).unwrap();
                let mut decompressed = xz2::read::XzDecoder::new(compressed_image_file);

                let mut temp_image_file = tempfile::NamedTempFile::new().unwrap();
                std::io::copy(&mut decompressed, &mut temp_image_file).unwrap();

                // Probe device
                let image = temp_image_file.into_file();

                let mut probe = rsblkid::probe::Probe::builder()
                    .scan_file(image)
                    .scan_device_superblocks(false)
                    .scan_device_partitions(true)
                    .build()
                    .unwrap();

                probe.find_device_properties();

                // Formatting output following example
                // https://github.com/util-linux/util-linux/blob/stable/v2.39/libblkid/samples/partitions.c#L34
                let partitions_iter = probe.iter_partitions();
                let err_msg = format!("PartitionIter::partition_table {:?} does not contain any known partition table", compressed_image_file_path);
                let partition_table = partitions_iter
                    .partition_table()
                    .expect(&err_msg);

                let size = probe.scanned_device_segment_size();
                let sector_size = probe.device_logical_sector_size();
                let pt = partition_table.partition_table_type().unwrap();
                let offset = partition_table.location_in_bytes().unwrap();
                let partition_table_id = partition_table.id();
                let id = if let Some(ref id) = partition_table_id { id.to_string()  } else { "(null)".to_string() } ;

                let mut output = String::new();
                let header = format!("size: {size}, sector size: {sector_size}, PT: {pt}, offset: {offset}, id={id}\n---\n");

                // List partitions
                let rows = partitions_iter.map(|partition| {
                    let number = partition.number();
                    let start = partition.location_in_sectors();
                    let size = partition.size_in_sectors();
                    let part_type = partition.partition_type();
                    let parent_table = partition.partition_table().unwrap();
                    let parent_table_id = parent_table.id();


                    let mut row = format!("#{}: {:>10} {:>10}  0x{:x}", number, start, size, part_type);

                    if parent_table_id != partition_table_id {
                        let parent_table_type = parent_table.partition_table_type().unwrap();

                        let table_type = format!(" ({})", parent_table_type);
                        row.push_str(&table_type);
                    }

                    if let Some(name) = partition.name() {
                        let name = format!(" name='{}'", name);
                        row.push_str(&name);
                    }

                    if let Some(uuid) = partition.uuid() {
                        let uuid = format!(" uuid='{}'", uuid);
                        row.push_str(&uuid);
                    }

                    if let Some(type_str) = partition.partition_type_string() {
                        let type_str = format!(" type='{}'", type_str);
                        row.push_str(&type_str);
                    }

                    row
                }).collect::<Vec<_>>().join("\n");

                output.push_str(&header);
                output.push_str(&rows);
                output.push('\n');

                // Construct path to expected partition table output
                let mut expected_file = blkid_dir.clone();
                let fragment = concat!("expected/partition_tables/lowprobe-pt-", $pt_name);
                expected_file.push(fragment);

                // Read expected output from file
                let mut f = std::fs::File::open(expected_file).unwrap();
                let mut expected = String::new();
                f.read_to_string(&mut expected).unwrap();

                pretty_assertions::assert_eq!(output, expected);
            }
        }
    };
}
