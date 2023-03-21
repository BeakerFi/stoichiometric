use scrypto::prelude::*;

#[derive(
    NonFungibleData, ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone,
)]
pub struct ProposalReceipt {
    pub proposal_id: u64,
}
