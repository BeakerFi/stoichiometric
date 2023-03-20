use scrypto::prelude::*;

#[derive(NonFungibleData, ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub struct ProposalReceipt {
    proposal_id: u64,
}