use scrypto::prelude::*;

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub struct Loan {
    /// Token used as collateral
    pub collateral_token: ResourceAddress,

    /// Amount of stablecoins loaned
    pub amount_loaned: Decimal,

    /// Time at the moment of loan,
    pub loan_date: i64
}