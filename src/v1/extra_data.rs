use deku::prelude::*;

use super::ASN1Cert;

#[derive(Clone, Debug, DekuRead, DekuWrite)]
#[deku(type = "()", bytes = "0")]
#[non_exhaustive]
pub enum ExtraData {
	#[deku(id = "()")]
	X509ExtraData(X509ExtraData),
	#[deku(id = "()")]
	PrecertExtraData(PrecertExtraData),
}

#[derive(Clone, Debug, DekuRead, DekuWrite)]
pub struct X509ExtraData {
	#[deku(update = "self.certificate_chain.len()", bytes = 3, endian = "big")]
	len: usize,
	#[deku(bytes_read = "len")]
	pub certificate_chain: Vec<ASN1Cert>,
}

#[derive(Clone, Debug, DekuRead, DekuWrite)]
pub struct PrecertExtraData {
	pub pre_certificate: ASN1Cert,
	#[deku(update = "self.precertificate_chain.len()", bytes = 3, endian = "big")]
	len: usize,
	#[deku(bytes_read = "len")]
	pub precertificate_chain: Vec<ASN1Cert>,
}
