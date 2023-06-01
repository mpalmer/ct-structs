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
	// TODO: This update is wrong, need to figure out how to calculate the
	// serialized length of the chain and put it into here
	#[deku(update = "self.certificate_chain.len()", bytes = 3, endian = "big")]
	len: usize,
	#[deku]
	#[deku(bytes_read = "len", cond = "*len > 0", default = "vec![]")]
	pub certificate_chain: Vec<ASN1Cert>,
}

#[derive(Clone, Debug, DekuRead, DekuWrite)]
pub struct PrecertExtraData {
	// TODO: This update is wrong, need to figure out how to calculate the
	// serialized length of the chain and put it into here
	pub pre_certificate: ASN1Cert,
	#[deku(update = "self.precertificate_chain.len()", bytes = 3, endian = "big")]
	len: usize,
	#[deku(bytes_read = "len", cond = "*len > 0", default = "vec![]")]
	pub precertificate_chain: Vec<ASN1Cert>,
}
