use std::collections::HashMap;
use scrypto::prelude::{Decimal, dec};
use crate::issuer_sqrt::ADMIN_BADGE_NAME;
use crate::issuer_state::LenderState;
use crate::utils::{instantiate, new_default_lender};

mod issuer_sqrt;
mod issuer_state;
mod utils;
mod dumb_oracle;
mod dumb_oracle_sqrt;


#[test]
fn test_instantiate() {
    let (test_env, _) = instantiate();
    assert_eq!(test_env.amount_owned_by_current(ADMIN_BADGE_NAME), Decimal::ONE)
}

#[test]
fn test_new_lender() {
    let (mut test_env, mut issuer_state) = instantiate();
    new_default_lender(&mut test_env, "btc");

    issuer_state.update();

    let mut lenders = HashMap::new();
    let btc_lender = LenderState::from(Decimal::ZERO, dec!("0.7"), dec!("0.0001"), dec!("1.3"), dec!("0.1"));
    lenders.insert(test_env.get_resource("btc").clone(), btc_lender);

    issuer_state.assert_state_is(&HashMap::new(), &lenders, 0, 0);
}

