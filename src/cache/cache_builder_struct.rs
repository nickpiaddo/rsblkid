// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use typed_builder::TypedBuilder;

// From standard library
use std::path::PathBuf;

// From this library
use crate::cache::{Cache, CacheBuilderError};

#[derive(Debug, TypedBuilder)]
#[builder(builder_type(name = CacheBuilder, vis = "pub", doc ="Configure and instantiate a [`Cache`].\n\nFor usage, see [`CacheBuilder::build`]."),
    build_method(vis = "", name = __build))]
pub(crate) struct Builder {
    #[builder(setter(
        strip_bool,
        doc = r"Discards changes when a [`Cache`] instance goes out of scope.

- **Note:** `discard_changes_on_drop` and `auto_save_changes_to` are **mutually exclusive**."
    ))]
    discard_changes_on_drop: bool,

    #[builder(
        default,
        setter(
            into,
            strip_option,
            doc = r"Saves changes to disk, automatically, when [`Cache`] goes out of scope.

# Argument

`destination` -- name of the file to save the cache's content into.

- **Note:** `discard_changes_on_drop` and `auto_save_changes_to` are **mutually exclusive**.
"
        )
    )]
    // FIXME find a way to have a custom setter name different from the field name
    // i.e. how to have
    //
    // pub fn auto_save_changes_to( self, dest_file: P)
    //
    // instead of the current default
    //
    // pub fn auto_save_changes_to( self, auto_save_changes_to: P)
    //
    // Ask question with example
    //
    // struct Fraction {
    // n: i64,
    // d: i64,
    // }
    //
    // Can we have FractionBuilder with setters with custom names?:
    //
    // numerator(n: i64)
    // denominator(d: i64(
    auto_save_changes_to: Option<PathBuf>,
}

#[allow(non_camel_case_types)]
impl<
        __discard_changes_on_drop: ::typed_builder::Optional<bool>,
        __auto_save_changes_to: ::typed_builder::Optional<Option<PathBuf>>,
    > CacheBuilder<(__discard_changes_on_drop, __auto_save_changes_to)>
{
    /// Builds a new [`Cache`] instance.
    ///
    /// # Examples
    /// ----
    ///
    /// ```
    /// use rsblkid::cache::Cache;
    /// use std::error::Error;
    /// use std::path::Path;
    /// use tempfile::NamedTempFile;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///
    ///     // Create a cache that automatically saves changes to the default cache file
    ///     // (`blkid.tab`) when dropped.
    ///     let result = Cache::builder().build();
    ///
    ///     assert!(result.is_ok());
    ///
    ///     // Create a cache that automatically saves changes to a custom cache file when dropped.
    ///     let temp_file = NamedTempFile::new()?; // Create a temporary destination file in `/tmp`.
    ///     let result = Cache::builder()
    ///         .auto_save_changes_to(temp_file.path())
    ///         .build();
    ///
    ///     assert!(result.is_ok());
    ///
    ///     // Create a cache that automatically discards changes when dropped.
    ///     let result = Cache::builder().discard_changes_on_drop().build();
    ///
    ///     assert!(result.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn build(self) -> Result<Cache, CacheBuilderError> {
        log::debug!("CacheBuilder::build configuring new `Cache` instance");

        let builder = self.__build();
        let discard_file = "/dev/null";

        match (
            builder.discard_changes_on_drop,
            builder.auto_save_changes_to,
        ) {
            // Default (i.e. save changes to `blkid.tab`.
            (false, None) => {
                log::debug!("CacheBuilder::build new default cache");

                Cache::new_default().map_err(CacheBuilderError::Cache)
            }
            // Can not save to empty path, defaults to `blkid.tab`.
            (false, Some(path)) if path.as_os_str().is_empty() => {
                log::debug!("CacheBuilder::build new default cache (given empty destination)");

                Cache::new_default().map_err(CacheBuilderError::Cache)
            }
            // Save cache to...
            (false, Some(path)) => {
                log::debug!(
                    "CacheBuilder::build new cache, saving data on drop to {}",
                    path.display()
                );

                Cache::new_auto_save_changes_to(path).map_err(CacheBuilderError::Cache)
            }
            // Discard changes.
            (true, None) => {
                log::debug!("CacheBuilder::build new cache, discarding data on drop");

                Cache::new_auto_save_changes_to(discard_file).map_err(CacheBuilderError::Cache)
            }
            // Can not both Discard AND Save.
            (true, Some(_)) => {
                let discard = "discard_changes_on_drop";
                let auto_save = "auto_save_changes_to";
                log::debug!(
                    "CacheBuilder::build called two mutually exclusive setters: `{}` and `{}`",
                    discard,
                    auto_save
                );

                let err_msg = format!(
                    "can not set `{}` and `{}` simultaneously",
                    discard, auto_save
                );

                Err(CacheBuilderError::MutuallyExclusive(err_msg))
            }
        }
    }
}
