use scrypto::prelude::{dec, Decimal};
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
    let pool_usd_btc = create_pool(&mut test_env, "usd", dec!(20000), "btc", dec!(1), dec!(100), dec!(100000));
    let ok = 5;
}