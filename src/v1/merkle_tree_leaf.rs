use deku::prelude::*;

use super::{TimestampedEntry, Version};

#[derive(Clone, Debug, DekuRead, DekuWrite)]
pub struct MerkleTreeLeaf {
	pub version: Version,
	pub entry: TreeLeafEntry,
}

#[derive(Clone, Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
#[non_exhaustive]
pub enum TreeLeafEntry {
	#[deku(id = "0")]
	TimestampedEntry(TimestampedEntry),
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::utils::test::setup;

	mod merkle_tree_leaf {
		use super::*;
		use crate::v1::*;

		#[test]
		fn parse_an_x509_leaf() {
			setup();

			let mtl = MerkleTreeLeaf::from_bytes((
				include_bytes!("test_vectors/x509_merkle_tree_leaf"),
				0,
			))
			.unwrap()
			.1;

			assert_eq!(Version::v1, mtl.version);

			#[allow(irrefutable_let_patterns)]
			if let TreeLeafEntry::TimestampedEntry(ts_entry) = &mtl.entry {
				assert_eq!(1666198004098, ts_entry.timestamp);

				if let SignedEntry::X509Entry(_) = ts_entry.signed_entry {
					// All good
				} else {
					unreachable!();
				}
			} else {
				unreachable!();
			}
		}

		#[test]
		fn parse_a_precert_leaf() {
			setup();

			let mtl = MerkleTreeLeaf::from_bytes((
				include_bytes!("test_vectors/precert_merkle_tree_leaf"),
				0,
			))
			.unwrap()
			.1;

			assert_eq!(Version::v1, mtl.version);

			#[allow(irrefutable_let_patterns)]
			if let TreeLeafEntry::TimestampedEntry(ts_entry) = &mtl.entry {
				assert_eq!(1532471986235, ts_entry.timestamp);

				if let SignedEntry::PrecertEntry(_) = ts_entry.signed_entry {
					// All good
				} else {
					unreachable!();
				}
			} else {
				unreachable!();
			}
		}
	}
}
