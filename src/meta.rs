//! Metadata about the `nahsql` crate
//! such as its version
//! and other miscellaneous
//! goodies.

/// The version of the `nahsql` crate,
/// fetched at compile-time using the
/// `CARGO_PKG_VERSION` environment variable.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The type to be used
/// for representing the
/// version of a database schema.
///
/// This type definition
/// makes the project easier to
/// maintain as the schema version
/// type can now easily be changed
/// to a different integer type if needed.
pub type SchemaVersion = u64;
