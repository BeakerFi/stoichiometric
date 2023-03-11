use std::collections::HashMap;
use std::process::Command;
use lazy_static::lazy_static;
use regex::Regex;
use scrypto::prelude::{dec, Decimal};
use sqrt::error::Error;
use crate::pool_state::{assert_current_position, PoolState, run_command, StepState};
use crate::router_sqrt::{ADMIN_BADGE_NAME, create_pool, instantiate, RouterMethods};

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

    // Rate is not exactly 20000:1 because of computational errors
    let pool_state = StepState::from(dec!(20000), Decimal::ZERO, dec!("19999.9994618360095662"), Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO);
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
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(9980000));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(10000000));

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(50266, (dec!(20000), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn test_inverted_pools() {

    let mut test_env = instantiate();

    test_env.call_method( RouterMethods::CreatePool(
        "btc".to_string(),
        Decimal::ONE,
        "usd".to_string(),
        dec!(20000),
        Decimal::ONE / dec!(20000),
        Decimal::ONE / dec!(100000),
        Decimal::ONE / dec!(100)
    )).run();

    let mut pool_btc_usd: PoolState = PoolState::from(String::new(), String::new());

    let router_address = test_env.get_component("router_comp").unwrap();
    let other_address = test_env.get_resource("btc").clone();

    let output = run_command(Command::new("resim").arg("show").arg(router_address));

    lazy_static! {
        static ref POOLS_LIST_RE: Regex = Regex::new(r#"Map<ResourceAddress, Tuple>\((.*), Tuple\(Own"#).unwrap();
    }

    let pools_list_cap = &POOLS_LIST_RE.captures(&output).expect("Could not find pools list");
    let pools_list = &pools_list_cap[1];

    lazy_static! {
        static ref POOLS_RE: Regex = Regex::new(r#"ResourceAddress\("(\w*)"\)"#).unwrap();
    }

    for cap in POOLS_RE.captures_iter(pools_list) {

        let resource = String::from(&cap[1]);
        if resource == other_address
        {
            pool_btc_usd = PoolState::from(router_address.to_string(),other_address);
            break;
        }
    }

    pool_btc_usd.update();


    // Rate is not exactly 20000:1 because of computational errors
    let pool_state = StepState::from(dec!(20000), Decimal::ZERO, dec!("19999.9994618360095662"), Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO);
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_btc_usd.assert_state_is(
        dec!("0.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO
    );
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(9980000));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(10000000));

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(50266, (dec!(20000), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn test_create_multiple_pools() {
    let mut test_env = instantiate();

    let pool_usd_btc = create_pool(&mut test_env, dec!(20000), "btc", dec!(1), dec!(100), dec!(100000));
    // Rate is not exactly 20000:1 because of computational errors
    let pool_state = StepState::from(dec!(20000), Decimal::ZERO, dec!("19999.9994618360095662"), Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO);
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
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(9980000));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(10000000));

    test_env.create_fixed_supply_token("eth", dec!(1000000));
    let pool_usd_eth = create_pool(&mut test_env, dec!(17000), "eth",dec!(10), dec!(10), dec!(20000));
    // Rate is not exactly 1700:1 because of computational errors
    let pool_state = StepState::from(dec!(17000), Decimal::ZERO, dec!("1699.82907049827534548"), Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO);
    let mut pool_states = HashMap::new();
    pool_states.insert(44280, pool_state);
    pool_usd_eth.assert_state_is(
        dec!("0.000115989063276095"),
        44280,
        dec!(10),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO
    );
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(9963000));
    assert_eq!(test_env.amount_owned_by_current("eth"), dec!(1000000));

    // Check that the user got the right positions
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(50266, (dec!(20000), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env, "btc", &step_pos_map);
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(44280, (dec!(17000), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env,"eth", &step_pos_map);
}

#[test]
fn test_create_pool_with_same_token_fail() {
    let mut test_env = instantiate();

    test_env.call_method( RouterMethods::CreatePool(
        "btc".to_string(),
        Decimal::ONE,
        "btc".to_string(),
        dec!(20000),
        Decimal::ONE / dec!(20000),
        Decimal::ONE / dec!(100000),
        Decimal::ONE / dec!(100)
    )).should_panic(Error::AssertFailed("Two pools cannot trade the same token".to_string()))
        .run();
}

#[test]
fn test_create_pool_with_no_stablecoin_fail() {
    let mut test_env = instantiate();
    test_env.create_fixed_supply_token("eth", dec!(1000));
    test_env.call_method( RouterMethods::CreatePool(
        "eth".to_string(),
        Decimal::ONE,
        "btc".to_string(),
        dec!(20000),
        Decimal::ONE / dec!(20000),
        Decimal::ONE / dec!(100000),
        Decimal::ONE / dec!(100)
    )).should_panic(Error::AssertFailed("Every pool should be Stablecoin/Other".to_string()))
        .run();
}

#[test]
fn test_create_pool_already_exists_fail() {
    let mut test_env = instantiate();
    create_pool(&mut test_env, dec!(20000), "btc", dec!(1), dec!(100), dec!(100000));

    test_env.call_method( RouterMethods::CreatePool(
        "usd".to_string(),
        Decimal::ZERO,
        "btc".to_string(),
        Decimal::ZERO,
        Decimal::ONE,
        dec!(0),
        dec!(1)
    )).should_panic(Error::AssertFailed("A pool trading these tokens already exists".to_string()))
        .run();
}

#[test]
fn test_create_pool_min_rate_zero_fail() {
    let mut test_env = instantiate();
    test_env.call_method( RouterMethods::CreatePool(
        "usd".to_string(),
        Decimal::ZERO,
        "btc".to_string(),
        Decimal::ZERO,
        Decimal::ZERO,
        dec!(0),
        dec!(1)
    )).should_panic(Error::AssertFailed("The minimum rate should be positive".to_string()))
        .run();
}

#[test]
fn create_pool_max_rate_less_than_min_fail() {
    let mut test_env = instantiate();
    test_env.call_method( RouterMethods::CreatePool(
        "usd".to_string(),
        Decimal::ZERO,
        "btc".to_string(),
        Decimal::ZERO,
        Decimal::ONE,
        Decimal::ONE,
        dec!("0.5")
    )).should_panic(Error::AssertFailed("The maximum rate should be greater than the minimum rate".to_string()))
        .run();
}

#[test]
fn create_pool_initial_rate_less_than_min_fail() {
    let mut test_env = instantiate();
    test_env.call_method( RouterMethods::CreatePool(
        "usd".to_string(),
        Decimal::ZERO,
        "btc".to_string(),
        Decimal::ZERO,
        dec!("0.5"),
        Decimal::ONE,
        dec!(2)
    )).should_panic(Error::AssertFailed("The initial rate should be included in the given rate range".to_string()))
        .run();
}

#[test]
fn create_pool_initial_rate_greater_than_max_fail() {
    let mut test_env = instantiate();
    test_env.call_method( RouterMethods::CreatePool(
        "usd".to_string(),
        Decimal::ZERO,
        "btc".to_string(),
        Decimal::ZERO,
        dec!(3),
        Decimal::ONE,
        dec!(2)
    )).should_panic(Error::AssertFailed("The initial rate should be included in the given rate range".to_string()))
        .run();
}