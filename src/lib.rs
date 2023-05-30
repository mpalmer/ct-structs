//! Data structures used in the [Certificate Transparency](https://certificate-transparency.org)
//! protocol.
//!
//! If you're working with CT logs, then you know the many and varied set of JSON-based data
//! structures that it uses.  This crate contains Rust structs that represent these structures, and
//! allow serde-driven serialization and deserialization.
//!
//! The top-level namespace contains modules representing each version of the CT protocol
//! (currently only [`v1`], because nobody seems to be bothered by CT v2).  Under that are the
//! structures for each available data type, with names that are, as far as possible, direct copies
//! of the names from [The Fine RFC](https://datatracker.ietf.org/doc/html/rfc6962).
//!

pub mod v1;

pub(crate) mod utils;
