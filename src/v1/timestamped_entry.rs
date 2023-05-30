use deku::prelude::*;

use super::{CtExtensions, SignedEntry};

#[derive(Clone, Debug, DekuRead, DekuWrite)]
pub struct TimestampedEntry {
	#[deku(endian = "big")]
	pub timestamp: u64,
	pub signed_entry: SignedEntry,
	pub extensions: CtExtensions,
}
