pub(crate) mod b64_bytes {
	use base64::{engine::general_purpose::STANDARD as b64, Engine as _};
	use serde::{
		de::{self, DeserializeOwned},
		Deserialize, Deserializer, Serializer,
	};

	pub(crate) fn serialize<S>(src: &[u8], serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		if serializer.is_human_readable() {
			serializer.serialize_str(&b64.encode(src))
		} else {
			serializer.serialize_bytes(src)
		}
	}

	pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
	where
		D: Deserializer<'de>,
		T: DeserializeOwned + From<Vec<u8>>,
	{
		if deserializer.is_human_readable() {
			let s = <String>::deserialize(deserializer)?;
			Ok(b64.decode(s).map_err(de::Error::custom)?.into())
		} else {
			<T>::deserialize(deserializer)
		}
	}
}

pub(crate) mod b64_deku {
	use base64::{engine::general_purpose::STANDARD as b64, Engine as _};
	use deku::prelude::*;
	use serde::ser::{self, Serializer};

	pub(crate) fn serialize<S, T>(src: &T, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
		T: DekuContainerWrite,
	{
		serializer.serialize_str(&b64.encode(src.to_bytes().map_err(ser::Error::custom)?))
	}
	/*
		pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
			where D: Deserializer<'de>,
				  T: for<'a> DekuContainerRead<'a>
		{
			let s = <String>::deserialize(deserializer)?;
			let b = b64.decode(s).map_err(de::Error::custom)?;
			Ok(T::from_bytes((&b, 0)).map_err(de::Error::custom)?.1)
		}
	*/
}

#[cfg(test)]
pub(crate) mod test {
	#[cfg(not(feature = "parse-tracing"))]
	pub(crate) fn setup() {}

	#[cfg(feature = "parse-tracing")]
	use std::sync::Once;

	#[cfg(feature = "parse-tracing")]
	static BEFORE_TESTS: Once = Once::new();

	#[cfg(feature = "parse-tracing")]
	pub(crate) fn setup() {
		BEFORE_TESTS.call_once(|| {
			stderrlog::new()
				.verbosity(4)
				.show_level(false)
				.color(stderrlog::ColorChoice::Never)
				.timestamp(stderrlog::Timestamp::Off)
				.init()
				.unwrap();
		});
	}

	#[cfg(not(feature = "parse-tracing"))]
	use stderrlog as _;
}
