use crate::{
    phase0::compute_domain,
    primitives::{BlsPublicKey, BlsSignature, Domain, DomainType, ExecutionAddress},
    ssz::prelude::*,
    state_transition::{Context, Error},
};

#[derive(Debug, Clone, Default, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValidatorRegistration {
    pub fee_recipient: ExecutionAddress,
    #[serde(with = "crate::serde::as_string")]
    pub gas_limit: u64,
    #[serde(with = "crate::serde::as_string")]
    pub timestamp: u64,
    #[serde(rename = "pubkey")]
    pub public_key: BlsPublicKey,
}

#[derive(Debug, Clone, Default, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignedValidatorRegistration {
    pub message: ValidatorRegistration,
    pub signature: BlsSignature,
}

pub fn compute_builder_domain(context: &Context) -> Result<Domain, Error> {
    let domain_type = DomainType::ApplicationBuilder;
    compute_domain(domain_type, None, None, context)
}
