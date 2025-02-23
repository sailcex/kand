#![allow(clippy::similar_names, clippy::too_many_lines)]

pub mod ta;
pub use ta::*;

pub mod helper;

pub mod error;
pub use error::{KandError, Result};

#[cfg(all(feature = "f32", feature = "f64"))]
pub type TAFloat = f64;

#[cfg(all(feature = "i32", feature = "i64"))]
pub type TAInt = i64;

/// Default floating-point precision type used across the library.
///
/// This type is determined by the enabled features:
/// - With feature "f64": Uses f64 (recommended for most cases)
/// - With feature "f32": Uses f32 (for memory-constrained environments)
/// - With no features enabled: Defaults to f64
///
/// The default configuration (feature "extended") provides f64 for:
/// - Higher precision calculations (15-17 decimal digits)
/// - Better handling of large price values
/// - More accurate technical indicator results
#[cfg(all(feature = "f32", not(feature = "f64")))]
pub type TAFloat = f32;

#[cfg(all(feature = "f64", not(feature = "f32")))]
pub type TAFloat = f64;

#[cfg(not(any(feature = "f32", feature = "f64")))]
pub type TAFloat = f64; // Default to f64 when no features are enabled

/// Default integer type used for indicator outputs.
///
/// This type is determined by the enabled features:
/// - With feature "i64": Uses i64 (recommended for most cases)
/// - With feature "i32": Uses i32 (for memory-constrained environments)
/// - With no features enabled: Defaults to i32
///
/// The default configuration (feature "extended") provides i64 for:
/// - Larger value range (-2^63 to 2^63-1)
/// - Better precision in accumulation operations
/// - Future-proof for high-frequency data analysis
#[cfg(all(feature = "i32", not(feature = "i64")))]
pub type TAInt = i32;

#[cfg(all(feature = "i64", not(feature = "i32")))]
pub type TAInt = i64;

#[cfg(not(any(feature = "i32", feature = "i64")))]
pub type TAInt = i32; // Default to i32 when no features are enabled
