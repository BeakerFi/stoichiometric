//! Definition of [`StepPosition`] and [`Position`]

use scrypto::prelude::*;

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub struct StepPosition {
    /// Liquidity of the position
    pub liquidity: Decimal,

    /// Value of the `stable_fees_per_liq` variable of the PoolStep last time that the
    /// associated [`Position`] collected fees.
    pub last_stable_fees_per_liq: Decimal,

    /// Value of the `other_fees_per_liq` variable of the PoolStep last time that the associated
    /// Position collected fees
    pub last_other_fees_per_liq: Decimal,
}