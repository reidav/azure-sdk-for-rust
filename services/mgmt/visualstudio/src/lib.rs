#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2014-04-preview")]
pub mod package_2014_04_preview;
#[cfg(all(feature = "package-2014-04-preview", not(feature = "no-default-tag")))]
pub use package_2014_04_preview::{models, Client, ClientBuilder};
