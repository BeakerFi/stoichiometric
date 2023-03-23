//! Definition of [`StepPosition`] and [`Position`]

use sbor::{CustomValueKind, Decoder, EncodeError, Encoder, ValueKind};
use scrypto::prelude::*;

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub struct StepPosition {

    liquidity: Decimal,
    last_stable_fees_per_liq: Decimal,
    last_other_fees_per_liq: Decimal
}