use scrypto::prelude::*;

#[derive(
    NonFungibleData, ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone,
)]
pub struct Loan {
    /// Token used as collateral
    pub collateral_token: ResourceAddress,

    /// Amount of tokens given as collateral
    #[mutable]
    pub collateral_amount: Decimal,

    /// Amount of stablecoins lent
    #[mutable]
    pub amount_lent: Decimal,

    /// Time at the moment of loan,
    pub loan_date: i64,

    /// Loan To Value at the moment of loan
    pub loan_to_value: Decimal,

    /// Daily interest rate at the moment of loan
    pub interest_rate: Decimal,
}

impl Loan {
    pub fn from(
        collateral_token: ResourceAddress,
        collateral_amount: Decimal,
        amount_lent: Decimal,
        loan_date: i64,
        loan_to_value: Decimal,
        interest_rate: Decimal,
    ) -> Self {
        Self {
            collateral_token,
            collateral_amount,
            amount_lent,
            loan_date,
            loan_to_value,
            interest_rate,
        }
    }
}
