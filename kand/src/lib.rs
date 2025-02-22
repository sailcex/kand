pub mod ta;
pub use ta::*;

pub mod helper;

pub mod error;
pub use error::{KandError, Result};

#[cfg(all(feature = "int32", feature = "int64"))]
compile_error!("Cannot enable both 'int32' and 'int64' features simultaneously");

#[cfg(all(feature = "float32", feature = "float64"))]
compile_error!("Cannot enable both 'float32' and 'float64' features simultaneously");

/// Default floating-point precision type used across the library.
///
/// This type is determined by the enabled features:
/// - With feature "float64": Uses f64 (recommended for most cases)
/// - With feature "float32": Uses f32 (for memory-constrained environments)
/// - With no features enabled: Defaults to f64
///
/// The default configuration (feature "float64") is recommended as it provides:
/// - Higher precision calculations
/// - Better handling of large price values
/// - More accurate technical indicator results
#[cfg(all(feature = "float32", not(feature = "float64")))]
pub type TAFloat = f32;
#[cfg(all(feature = "float64", not(feature = "float32")))]
pub type TAFloat = f64;
#[cfg(not(any(feature = "float32", feature = "float64")))]
pub type TAFloat = f64; // Default to f64 when no features are enabled

/// Default integer type used for indicator outputs.
///
/// This type is determined by the enabled features:
/// - With feature "int64": Uses i64
/// - With feature "int32": Uses i32
/// - With no features enabled: Defaults to i32
///
/// The default configuration (feature "int64") is recommended for most use cases
/// as it provides better precision for large numbers.
#[cfg(all(feature = "int32", not(feature = "int64")))]
pub type TAInt = i32;
#[cfg(all(feature = "int64", not(feature = "int32")))]
pub type TAInt = i64;
#[cfg(not(any(feature = "int32", feature = "int64")))]
pub type TAInt = i32; // Default to i32 when no features are enabled
