use deku::prelude::*;

#[derive(Clone, Debug, Default, DekuRead, DekuWrite)]
pub struct ASN1Cert {
	#[deku(update = "self.certificate.len()", bytes = 3, endian = "big")]
	len: usize,
	#[deku(count = "len")]
	pub certificate: Vec<u8>,
}

#[derive(Clone, Debug, Default, DekuRead, DekuWrite)]
pub struct PreCert {
	pub issuer_key_hash: [u8; 32],
	#[deku(update = "self.tbs_certificate.len()", bytes = 3, endian = "big")]
	tc_len: u32,
	#[deku(count = "tc_len")]
	pub tbs_certificate: Vec<u8>,
}
