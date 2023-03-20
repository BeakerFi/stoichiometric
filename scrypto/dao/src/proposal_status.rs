use scrypto::prelude::*;

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub enum ProposalStatus {
    VotingStage,
    Accepted,
    Rejected,
    Executed
}

impl ProposalStatus {

    pub fn is_accepted(&self) -> bool
    {
        match self {
            ProposalStatus::Accepted => true,
            _ => false
        }
    }

    pub fn is_voting_stage(&self) -> bool
    {
        match self {
            ProposalStatus::VotingStage => true,
            _ => false
        }
    }
}