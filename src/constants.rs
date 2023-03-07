use scrypto::math::BnumI256;
use scrypto::prelude::Decimal;

pub const NB_STEP: u16 = 65535;

pub const LP_FEE: Decimal = Decimal(BnumI256::from_digits([2500000000000000, 0, 0, 0]));

pub const PROTOCOL_FEE: Decimal = Decimal(BnumI256::from_digits([500000000000000, 0, 0, 0]));

pub const TRADED: Decimal = Decimal(BnumI256::from_digits([997500000000000000, 0, 0, 0]));
