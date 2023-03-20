use scrypto::prelude::*;

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub enum ProposedChange
{
    /// Changes the supporting period of proposals
    ChangeSupportPeriod(u64),

    /// Changes the vote period of proposals
    ChangeVotePeriod(u64),

    /// Changes the suggestion approval threshold of proposals
    ChangeSuggestionApprovalThreshold(Decimal),

    /// Changes the minimum amount of votes that have to be casted to consider a vote valid
    ChangeMinimumVoteThreshold(Decimal),

    /// Allows claiming of a certain amount of resource by a voter id
    AllowClaim(ResourceAddress, Decimal, u64),

    /// Changes the parameters of a given stablecoin lender
    ChangeLenderParameters(ResourceAddress, Decimal, Decimal, Decimal, Decimal),

    /// Changes the oracle of a given stablecoin lender
    ChangeLenderOracle(ResourceAddress, ComponentAddress),

    /// Adds a new token as possible collateral. Taking this decision will also create a pool for the given token
    AddNewCollateralToken(ResourceAddress),

    /// Adds given tokens to the stablecoin issuer reserves
    AddTokensToIssuerReserves(Vec<(ResourceAddress, Decimal)>)
}

