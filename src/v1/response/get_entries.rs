use base64::{engine::general_purpose::STANDARD as b64, Engine as _};
use deku::prelude::*;
use serde::{
	de::{self, Error as _, MapAccess, Visitor},
	Deserialize, Deserializer, Serialize,
};
use std::any::type_name;

use super::super::{ExtraData, MerkleTreeLeaf, SignedEntry, TreeLeafEntry};
use crate::utils::b64_deku;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetEntries {
	pub entries: Vec<ResponseEntry>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseEntry {
	#[serde(serialize_with = "b64_deku::serialize")]
	pub leaf_input: MerkleTreeLeaf,
	#[serde(serialize_with = "b64_deku::serialize")]
	pub extra_data: ExtraData,
}

struct ResponseEntryVisitor {
	leaf_input: Option<MerkleTreeLeaf>,
	extra_data: Option<Vec<u8>>,
}

impl ResponseEntryVisitor {
	fn new() -> Self {
		ResponseEntryVisitor {
			leaf_input: None,
			extra_data: None,
		}
	}
}

impl<'de> Visitor<'de> for ResponseEntryVisitor {
	type Value = ResponseEntry;

	fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		formatter.write_str("a get-entries response entry")
	}

	fn visit_map<M>(mut self, mut access: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		fn parse_extra_data<'a, T, E>(extra_data: &'a [u8]) -> Result<T, E>
		where
			T: DekuContainerRead<'a>,
			E: de::Error,
		{
			match T::from_bytes((extra_data, 0)) {
				Ok(((&[], 0), xd)) => Ok(xd),
				Ok(_) => Err(E::custom(format!(
					"malformed {}: trailing bytes",
					type_name::<T>()
				))),
				Err(e) => Err(E::custom(format!("malformed {}: {e}", type_name::<T>()))),
			}
		}

		while let Some((key, value)) = access.next_entry::<String, _>()? {
			if key == "leaf_input" {
				self.leaf_input = match MerkleTreeLeaf::from_bytes((
					&b64.decode::<String>(value).map_err(M::Error::custom)?,
					0,
				)) {
					Ok(((&[], 0), leaf)) => Ok(Some(leaf)),
					Ok(_) => Err(M::Error::custom(
						"malformed merkle tree leaf: trailing data",
					)),
					Err(e) => Err(M::Error::custom(format!("malformed merkle tree leaf: {e}"))),
				}?;
			} else if key == "extra_data" {
				self.extra_data = Some(b64.decode(value).map_err(M::Error::custom)?);
			} else {
				return Err(M::Error::unknown_field(&key, &["leaf_input", "extra_data"]));
			}
		}

		let (leaf_input, extra_data) = if let Some(leaf_input) = self.leaf_input {
			if let Some(extra_data) = self.extra_data {
				(leaf_input, extra_data)
			} else {
				return Err(M::Error::missing_field("extra_data"));
			}
		} else {
			return Err(M::Error::missing_field("leaf_input"));
		};

		let extra_data = match &leaf_input.entry {
			TreeLeafEntry::TimestampedEntry(ts_entry) => match ts_entry.signed_entry {
				SignedEntry::X509Entry(_) => {
					ExtraData::X509ExtraData(parse_extra_data(&extra_data)?)
				}
				SignedEntry::PrecertEntry(_) => {
					ExtraData::PrecertExtraData(parse_extra_data(&extra_data)?)
				}
			},
		};

		Ok(ResponseEntry {
			leaf_input,
			extra_data,
		})
	}
}

impl<'de> Deserialize<'de> for ResponseEntry {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_map(ResponseEntryVisitor::new())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::test::setup;
	use base64::engine::general_purpose::STANDARD as b64;
	use x509_parser::{certificate::X509Certificate, prelude::FromDer as _};

	#[test]
	fn correctly_decodes_x509_entry() {
		setup();

		let json_entries = format!(
			r#"{{"entries":[{{"leaf_input":"{}","extra_data":"{}"}}]}}"#,
			b64.encode(include_bytes!("../test_vectors/x509_merkle_tree_leaf")),
			b64.encode(include_bytes!("../test_vectors/x509_extra_data"))
		);

		let decoded_entries: GetEntries = serde_json::from_str(&json_entries).unwrap();
		let TreeLeafEntry::TimestampedEntry(te_entry) =
			&decoded_entries.entries[0].leaf_input.entry;
		assert_eq!(1666198004098, te_entry.timestamp);
		if let SignedEntry::X509Entry(signed_entry) = &te_entry.signed_entry {
			let (_rest, cert) = X509Certificate::from_der(&signed_entry.certificate).unwrap();
			assert_eq!("CN=crt.sh", cert.subject().to_string());
		} else {
			panic!("signed_entry was not X509Entry");
		}
	}

	#[test]
	fn correctly_decodes_a_precert_entry() {
		setup();

		let json_entries = format!(
			r#"{{"entries":[{{"leaf_input":"{}","extra_data":"{}"}}]}}"#,
			b64.encode(include_bytes!("../test_vectors/precert_merkle_tree_leaf")),
			b64.encode(include_bytes!("../test_vectors/precert_extra_data"))
		);

		let decoded_entries: GetEntries = serde_json::from_str(&json_entries).unwrap();
		let TreeLeafEntry::TimestampedEntry(te_entry) =
			&decoded_entries.entries[0].leaf_input.entry;
		assert_eq!(1532471986235, te_entry.timestamp);
	}

	#[test]
	fn correctly_decodes_x509_entry_with_empty_extra_data() {
		setup();

		let json_entries = format!(
			r#"{{"entries":[{{"leaf_input":"{}","extra_data":"{}"}}]}}"#,
			b64.encode(include_bytes!("../test_vectors/x509_merkle_tree_leaf")),
			b64.encode(b"\0\0\0")
		);

		let decoded_entries: GetEntries = serde_json::from_str(&json_entries).unwrap();
		let TreeLeafEntry::TimestampedEntry(te_entry) =
			&decoded_entries.entries[0].leaf_input.entry;
		assert_eq!(1666198004098, te_entry.timestamp);
		if let ExtraData::X509ExtraData(extra_data) = &decoded_entries.entries[0].extra_data {
			assert!(extra_data.certificate_chain.is_empty());
		} else {
			panic!("extra_data was not X509ExtraData");
		}
	}
}
