// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[macro_export]
macro_rules! generate_filesystem_tests {
    [ $($fs_name:literal),* ] => {
        $( gen_fs_test!($fs_name, $fs_name, "session_offset", 0); )*
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
