mod positions;
use std::num::ParseIntError;
use radix_engine_interface::address::Bech32Encoder;
use radix_engine_interface::api::types::{Decoder};
use radix_engine_interface::data::ScryptoDecoder;
use radix_engine_interface::math::Decimal;
use radix_engine_interface::model::{ResourceAddress, NonFungibleLocalId};
use radix_engine_interface::node::NetworkDefinition;
use sbor::ValueKind;
use sbor::ValueKind::Map;
use scrypto::prelude::*;
use crate::positions::{StepPosition};

fn main() {

    /*
    let args: Vec<String> = env::args().collect();


    match args.get(1).unwrap().parse::<u8>().unwrap()
    {
        1 => { //Number to decode loans
            let mutable_hex = args.get(2).unwrap();
            let immutable_hex = args.get(3).unwrap();

            decode_loan(immutable_hex, mutable_hex);
        }
        2 => { //Number to decode positions
            let mutable_hex = args.get(2).unwrap();
            let immutable_hex = args.get(3).unwrap();
        }
        3 => { //Number to decode voter cards

            let mutable_hex = args.get(2).unwrap();

            decode_voter_card(mutable_hex);

        }

        _=> { panic!("No decode option for this number") }
    }*/

    let mutable_data_hex = String::from("5c210123082105000003b50000c84e676dc11b000000000000000000000000000000000000000000000000b50000000000000000000000000000000000000000000000000000000000000000b50000000000000000000000000000000000000000000000000000000000000000010003b50000c84e676dc11b000000000000000000000000000000000000000000000000b50000000000000000000000000000000000000000000000000000000000000000b50000000000000000000000000000000000000000000000000000000000000000020003b50000c84e676dc11b000000000000000000000000000000000000000000000000b50000000000000000000000000000000000000000000000000000000000000000b50000000000000000000000000000000000000000000000000000000000000000030003b50000c84e676dc11b000000000000000000000000000000000000000000000000b50000000000000000000000000000000000000000000000000000000000000000b50000000000000000000000000000000000000000000000000000000000000000a4bb03b5fe8c99900fc9fbfded0f00000000000000000000000000000000000000000000b50000000000000000000000000000000000000000000000000000000000000000b500743ba40b000000000000000000000000000000000000000000000000000000");
    let immutable_data_hex = String::from("5c21018200f25830a78601c5d6c54465ab1b8c8630263fe2e6f29721d7e274");
    decode_position(&immutable_data_hex, &mutable_data_hex);
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn decode_loan(immutable_hex: &String, mutable_hex: &String) {

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

pub fn decode_position(immutable_hex: &String, mutable_hex: &String) {

    let to_decode = String::from(&mutable_hex[14..]);
    let step_positions_len = 204;
    let nb_positions = to_decode.len() / step_positions_len;

    let mut step_positions_string = String::new();

    for i in 0..nb_positions
    {
        let start = i*step_positions_len;
        let end = (i+1)*step_positions_len;
        let step_position = String::from(&to_decode[start..end]);
        let step = String::from(&step_position[..6]);
        let dec_1_to_decode = format!("5c{}",  &step_position[6..72]);
        let dec_2_to_decode = String::from(&step_position[72..138]);
        let dec_3_to_decode = String::from(&step_position[138..204]);

        let step = Decimal::ZERO;
        let liquidity: Decimal = ScryptoDecoder::new(dec_1_to_decode.as_bytes()).decode_payload(92).unwrap();
        let last_stable_fees_per_liq: Decimal = ScryptoDecoder::new(dec_2_to_decode.as_bytes()).decode_deeper_body_with_value_kind( Decimal::value_kind()).unwrap();
        let last_other_fees_per_liq: Decimal = ScryptoDecoder::new(dec_3_to_decode.as_bytes()).decode_deeper_body_with_value_kind( Decimal::value_kind()).unwrap();

        step_positions_string = format!("{}({}, ({}, {}, {}), ", step_positions_string, step, liquidity, last_stable_fees_per_liq, last_other_fees_per_liq);
    }

    step_positions_string.pop();
    step_positions_string.pop();

    step_positions_string = format!("({})", step_positions_string);



    let immutable_vec_bytes = decode_hex(immutable_hex).expect("The input string could not be parsed correctly");
    let immutable_bytes = immutable_vec_bytes.as_slice();

    let other_token_address: ResourceAddress = ScryptoDecoder::new(immutable_bytes).decode_payload(92).unwrap();

    let bech = Bech32Encoder::new(&NetworkDefinition::nebunet());
    let other_token = bech.encode_resource_address_to_string(&other_token_address);

    println!("{} {}", other_token, "ok");
}

pub fn decode_voter_card(mutable_hex: &String) {

    let mutable_vec_bytes = decode_hex(mutable_hex).expect("The input string could not be parsed correctly");
    let mutable_bytes = mutable_vec_bytes.as_slice();

    let (voting_power, stablecoins_locked, positions_locked_ids, last_proposal_voted_id, proposals_voted): (Decimal, Decimal, Vec<NonFungibleLocalId>, u64, HashSet<u64>) = ScryptoDecoder::new(mutable_bytes).decode_payload(92).unwrap();

    let mut positions_ids_string = String::new();
    for id in positions_locked_ids {
        positions_ids_string = format!("{}{}, ", positions_ids_string, id);
    }
    positions_ids_string.pop();
    positions_ids_string.pop();

    positions_ids_string= format!("({})", positions_ids_string);

    let mut proposals_voted_string = String::new();
    for id in proposals_voted {
        proposals_voted_string = format!("{}{}, ", proposals_voted_string, id);
    }
    proposals_voted_string.pop();
    proposals_voted_string.pop();

    proposals_voted_string = format!("({})", proposals_voted_string);


    println!("{} {} {} {} {}", voting_power, stablecoins_locked, positions_ids_string, last_proposal_voted_id, proposals_voted_string);
}