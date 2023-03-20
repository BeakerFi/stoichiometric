use scrypto::prelude::*;

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub enum ProposalStatus
{
    SuggestionPhase,
    SuggestionRejected,
    VotingPhase,
    ProposalRejected,
    ProposalAccepted
}