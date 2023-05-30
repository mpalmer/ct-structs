use super::SignedTreeHead;

mod get_entries;

pub type GetSth = SignedTreeHead;

pub use get_entries::{GetEntries, ResponseEntry};
