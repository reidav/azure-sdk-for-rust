#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2020-06-01")]
pub mod package_2020_06_01;
#[cfg(all(feature = "package-2020-06-01", not(feature = "no-default-tag")))]
pub use package_2020_06_01::{models, Client, ClientBuilder};
