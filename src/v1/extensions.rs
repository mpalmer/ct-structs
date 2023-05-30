use deku::prelude::*;

#[derive(Clone, Debug, Default, DekuRead, DekuWrite)]
pub struct CtExtensions {
	#[deku(update = "self.data.len()", endian = "big")]
	len: u16,
	#[deku(count = "len")]
	pub data: Vec<u8>,
}
