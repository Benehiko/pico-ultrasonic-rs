#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![no_std]
// disable warning for already-stabilized features.
// Needed to pass CI, because we deny warnings.
// We don't immediately remove them to not immediately break older nightlies.
// When all features are stable, we'll remove them.
#![cfg_attr(nightly, allow(stable_features, unknown_lints))]
#![cfg_attr(nightly, feature(async_fn_in_trait, impl_trait_projections))]
#![allow(async_fn_in_trait)]

pub mod delay;
pub mod digital;
pub mod i2c;
pub mod spi;
