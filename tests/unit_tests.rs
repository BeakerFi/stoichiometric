use std::collections::HashMap;
use scrypto::prelude::{dec, Decimal};
use crate::pool_state::StepState;
use crate::router_sqrt::{ADMIN_BADGE_NAME, create_pool, instantiate};

mod pool_state;
mod router_sqrt;

#[test]
fn test_instantiate() {
    let test_env = instantiate();
    assert_eq!(test_env.amount_owned_by_current(ADMIN_BADGE_NAME), Decimal::ONE);
}

#[test]
fn test_create_pool() {
    let mut test_env = instantiate();

    let pool_usd_btc = create_pool(&mut test_env, dec!(20000), "btc", dec!(1), dec!(100), dec!(100000));
    let pool_state = StepState::from(dec!(20000), Decimal::ONE, dec!(20000), Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO);
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_usd_btc.assert_state_is(
        dec!("0.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO
    );
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(99980000));
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(99999999));
}

#[test]
fn test_create_multiple_pool() {
    let mut test_env = instantiate();

    let pool_usd_btc = create_pool(&mut test_env, dec!(20000), "btc", dec!(1), dec!(100), dec!(100000));
    let pool_state = StepState::from(dec!(20000), Decimal::ONE, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO);
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_usd_btc.assert_state_is(
        dec!("0.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO
    );
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(99980000));
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(99999999));

    test_env.create_fixed_supply_token("eth", dec!(1000000));
    let pool_usd_eth = create_pool(&mut test_env, dec!(17000), "eth",dec!(10), dec!(17), dec!(17000));
    let pool_state = StepState::from(dec!(17000), dec!(10), Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO);


}