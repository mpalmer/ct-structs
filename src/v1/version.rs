use deku::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, DekuRead, DekuWrite, PartialEq)]
#[deku(type = "u8")]
#[non_exhaustive]
pub enum Version {
	v1 = 0,
}
