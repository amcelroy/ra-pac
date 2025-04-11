#![no_std]
#![allow(non_camel_case_types)]
#![doc = include_str!("../README.md")]
#![allow(unused_imports)]

// Define which module to use based on the chip feature flags
#[cfg_attr(feature = "ra0e1", path = "./pacs/ra0e1/pac.rs")]
#[cfg_attr(feature = "ra2a1", path = "./pacs/ra2a1/pac.rs")]
#[cfg_attr(feature = "ra2a2", path = "./pacs/ra2a2/pac.rs")]
#[cfg_attr(feature = "ra2e1", path = "./pacs/ra2e1/pac.rs")]
#[cfg_attr(feature = "ra2e2", path = "./pacs/ra2e2/pac.rs")]
#[cfg_attr(feature = "ra2e3", path = "./pacs/ra2e3/pac.rs")]
#[cfg_attr(feature = "ra2l1", path = "./pacs/ra2l1/pac.rs")]
#[cfg_attr(feature = "ra4e1", path = "./pacs/ra4e1/pac.rs")]
#[cfg_attr(feature = "ra4m1", path = "./pacs/ra4m1/pac.rs")]
#[cfg_attr(feature = "ra4m2", path = "./pacs/ra4m2/pac.rs")]
#[cfg_attr(feature = "ra4m3", path = "./pacs/ra4m3/pac.rs")]
#[cfg_attr(feature = "ra4w1", path = "./pacs/ra4w1/pac.rs")]
#[cfg_attr(feature = "ra6e1", path = "./pacs/ra6e1/pac.rs")]
#[cfg_attr(feature = "ra6m1", path = "./pacs/ra6m1/pac.rs")]
#[cfg_attr(feature = "ra6m2", path = "./pacs/ra6m2/pac.rs")]
#[cfg_attr(feature = "ra6m3", path = "./pacs/ra6m3/pac.rs")]
#[cfg_attr(feature = "ra6m4", path = "./pacs/ra6m4/pac.rs")]
#[cfg_attr(feature = "ra6m5", path = "./pacs/ra6m5/pac.rs")]
#[cfg_attr(feature = "ra6t1", path = "./pacs/ra6t1/pac.rs")]
mod inner;

// Re-export all items from the selected implementation
pub use inner::*;

// Define constants according to user instructions (uppercase)
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
