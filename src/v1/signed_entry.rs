use deku::prelude::*;

use super::{ASN1Cert, PreCert};

#[derive(Clone, Debug, DekuRead, DekuWrite)]
#[deku(type = "u16", id_endian = "big")]
#[non_exhaustive]
pub enum SignedEntry {
	#[deku(id = "0")]
	X509Entry(ASN1Cert),
	#[deku(id = "1")]
	PrecertEntry(PreCert),
}
