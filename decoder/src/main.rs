use std::env;
use std::num::ParseIntError;
use radix_engine_interface::address::Bech32Encoder;
use radix_engine_interface::api::types::{Decoder};
use radix_engine_interface::data::ScryptoDecoder;
use radix_engine_interface::math::Decimal;
use radix_engine_interface::model::ResourceAddress;
use radix_engine_interface::node::NetworkDefinition;

fn main() {

    let args: Vec<String> = env::args().collect();
    //let hex_str = args.get(1).unwrap();
    let mutable_hex = args.get(1).unwrap();
    let immutable_hex = args.get(2).unwrap();

    let mutable_vec_bytes = decode_hex(mutable_hex).expect("The input string could not be parsed correctly");
    let mutable_bytes = mutable_vec_bytes.as_slice();
    let (collateral_amount, amount_lent): (Decimal, Decimal) = ScryptoDecoder::new(mutable_bytes).decode_payload(92).unwrap();

    let immutable_vec_bytes = decode_hex(immutable_hex).expect("The input string could not be parsed correctly");
    let immutable_bytes = immutable_vec_bytes.as_slice();
    let (collateral_token_tmp, loan_date, loan_to_value, interest_rate): (ResourceAddress, i64, Decimal, Decimal) = ScryptoDecoder::new(immutable_bytes).decode_payload(92).unwrap();

    let bech = Bech32Encoder::new(&NetworkDefinition::nebunet());
    let collateral_token = bech.encode_resource_address_to_string(&collateral_token_tmp);

    println!("{} {} {} {} {} {}", collateral_token, collateral_amount, amount_lent, loan_date, loan_to_value, interest_rate);
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}