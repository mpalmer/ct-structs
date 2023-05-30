use serde::{Deserialize, Serialize};

use crate::utils::b64_bytes;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SignedTreeHead {
	pub tree_size: u64,
	pub timestamp: u64,
	#[serde(with = "b64_bytes")]
	pub sha256_root_hash: Vec<u8>,
	#[serde(with = "b64_bytes")]
	pub tree_head_signature: Vec<u8>,
}
