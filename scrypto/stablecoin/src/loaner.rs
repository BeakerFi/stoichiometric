use scrypto::blueprint;

#[blueprint]
mod loaner {

    pub struct Loaner {
        loan_to_value: Decimal,
        interest_rate: Decimal,
        

    }


}