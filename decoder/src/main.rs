use std::collections::HashMap;
use std::num::ParseIntError;
use radix_engine_interface::api::types::{DecodeError, Decoder};
use radix_engine_interface::data::ScryptoDecoder;
use radix_engine_interface::math::Decimal;

fn main() {

    //let args: Vec<String> = env::args().collect();
    //let hex_str = args.get(1).unwrap();
    let hex_str = "5c210182000000000000000000000000000000000000000000000000000000";
    let vec_bytes = decode_hex(hex_str).expect("The input string could not be parsed correctly");
    let bytes = vec_bytes.as_slice();

    let step_positions: Result<(u16,(Decimal, Decimal, Decimal)), DecodeError> = ScryptoDecoder::new(bytes).decode_payload(92);
    let ok = 5;
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}