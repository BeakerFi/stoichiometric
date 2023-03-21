use crate::pool_state::{StepState};
use crate::router_sqrt::{RouterMethods, ADMIN_BADGE_NAME, POSITION_NAME};
use crate::utils::{
    add_liquidity, add_liquidity_at_step, add_liquidity_at_steps, assert_current_position,
    assert_no_positions, create_pool, instantiate,
};
use scrypto::prelude::{dec, Decimal};
use sqrt::error::Error;
use std::collections::HashMap;

mod pool_state;
mod router_sqrt;
mod utils;

#[test]
fn test_instantiate() {
    let test_env = instantiate();
    assert_eq!(
        test_env.amount_owned_by_current(ADMIN_BADGE_NAME),
        Decimal::ONE
    );
}

#[test]
fn test_create_pool() {
    let mut test_env = instantiate();

    let pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    let pool_states = HashMap::new();
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );
}

#[test]
fn test_create_multiple_pools() {
    let mut test_env = instantiate();

    let pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );
    let pool_states = HashMap::new();
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    test_env.create_fixed_supply_token("eth", dec!(1000000));
    let pool_usd_eth = create_pool(
        &mut test_env,
        "eth",
        dec!(1700),
        dec!(10),
        dec!(20000),
    );
    let pool_states = HashMap::new();
    pool_usd_eth.assert_state_is(
        dec!("1.000115989063276095"),
        44280,
        dec!(10),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );
}

#[test]
fn test_create_pool_with_stablecoin_fail() {
    let mut test_env = instantiate();

    test_env
        .call_method(RouterMethods::CreatePool(
            "usd".to_string(),
            Decimal::ONE,
            dec!("0.0001"),
                dec!(2)
        ))
        .should_panic(Error::AssertFailed(
            "Two pools cannot trade the same token".to_string(),
        ))
        .run();
}

#[test]
fn test_create_pool_already_exists_fail() {
    let mut test_env = instantiate();
    create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    test_env
        .call_method(RouterMethods::CreatePool(
            "btc".to_string(),
            dec!(20000),
            dec!(100),
            dec!(100000),
        ))
        .should_panic(Error::AssertFailed(
            "A pool trading these tokens already exists".to_string(),
        ))
        .run();
}

#[test]
fn test_create_pool_min_rate_zero_fail() {
    let mut test_env = instantiate();
    test_env
        .call_method(RouterMethods::CreatePool(
            "btc".to_string(),
            Decimal::ZERO,
            Decimal::ZERO,
            dec!(0),
        ))
        .should_panic(Error::AssertFailed(
            "The minimum rate should be positive".to_string(),
        ))
        .run();
}

#[test]
fn create_pool_max_rate_less_than_min_fail() {
    let mut test_env = instantiate();
    test_env
        .call_method(RouterMethods::CreatePool(
            "btc".to_string(),
            Decimal::ZERO,
            Decimal::ONE,
            dec!("0.5"),
        ))
        .should_panic(Error::AssertFailed(
            "The maximum rate should be greater than the minimum rate".to_string(),
        ))
        .run();
}

#[test]
fn create_pool_initial_rate_less_than_min_fail() {
    let mut test_env = instantiate();
    test_env
        .call_method(RouterMethods::CreatePool(
            "btc".to_string(),
            dec!("0.5"),
            Decimal::ONE,
            dec!(2),
        ))
        .should_panic(Error::AssertFailed(
            "The initial rate should be included in the given rate range".to_string(),
        ))
        .run();
}

#[test]
fn create_pool_initial_rate_greater_than_max_fail() {
    let mut test_env = instantiate();
    test_env
        .call_method(RouterMethods::CreatePool(
            "btc".to_string(),
            dec!(3),
            Decimal::ONE,
            dec!(2),
        ))
        .should_panic(Error::AssertFailed(
            "The initial rate should be included in the given rate range".to_string(),
        ))
        .run();
}

#[test]
fn add_liquidity_at_step_no_position() {
    let mut test_env = instantiate();
    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    add_liquidity_at_step(&mut test_env, dec!(100), "btc", dec!(1), 30000, None).run();
    pool_usd_btc.update();

    // Rates are not correct due to small computational errors
    let pool_state = StepState::from(
        dec!(100),
        Decimal::ZERO,
        dec!("2362.1744661270945891"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(30000, pool_state);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(9999900));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(10000000));

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(30000, (dec!(100), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn add_liquidity_at_step_with_position() {
    let mut test_env = instantiate();
    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    add_liquidity_at_step(&mut test_env, dec!(100), "btc", dec!(1), 30000, None).run();

    add_liquidity_at_step(
        &mut test_env,
        dec!(100),
        "btc",
        dec!(1),
        50266,
        Some("#0#".to_string()),
    )
    .run();

    pool_usd_btc.update();

    // Rates are not correct due to small computational errors
    let pool_state = StepState::from(
        dec!(100),
        Decimal::ZERO,
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let pool_state_2 = StepState::from(
        dec!(100),
        Decimal::ZERO,
        dec!("2362.1744661270945891"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_states.insert(30000, pool_state_2);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(9999800));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(10000000));

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(30000, (dec!(100), Decimal::ZERO, Decimal::ZERO));
    step_pos_map.insert(50266, (dec!(100), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn add_liquidity_same_step() {
    let mut test_env = instantiate();
    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    add_liquidity_at_step(
        &mut test_env,
        dec!(100),
        "btc",
        dec!(1),
        50266,
        None,
    )
    .run();

    add_liquidity_at_step(
        &mut test_env,
        dec!(100),
        "btc",
        dec!(1),
        50266,
        Some("#0#".to_string()),
    )
        .run();

    pool_usd_btc.update();

    // Rates are not correct due to small computational errors
    let pool_state = StepState::from(
        dec!(200),
        Decimal::ZERO,
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(9999800));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(10000000));

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(50266, (dec!(200), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn add_liquidity_greater_step() {
    let mut test_env = instantiate();
    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    add_liquidity_at_step(
        &mut test_env,
        dec!(100),
        "btc",
        dec!(1),
        50266,
        None,
    )
        .run();

    add_liquidity_at_step(
        &mut test_env,
        dec!(100),
        "btc",
        dec!(1),
        65000,
        Some("#0#".to_string()),
    )
    .run();

    pool_usd_btc.update();

    // Rates are not correct due to small computational errors
    let pool_state = StepState::from(
        dec!(100),
        Decimal::ZERO,
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let pool_state_2 = StepState::from(
        Decimal::ZERO,
        dec!(1),
        dec!("94516.8566500089249139"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_states.insert(65000, pool_state_2);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(9999900));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(9999999));

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(
        65000,
        (dec!("94516.8566500089249139"), Decimal::ZERO, Decimal::ZERO),
    );
    step_pos_map.insert(50266, (dec!(100), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn add_liquidity_at_steps_no_position() {
    let mut test_env = instantiate();
    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    let mut steps = vec![];
    let mut pool_states = HashMap::new();
    let mut step_pos_map = HashMap::new();
    let step_change = dec!("1.000105411144423293");

    for i in 51000_u16..51005 {
        steps.push((i, dec!(200), Decimal::ONE));

        let new_pool_state = StepState::from(
            Decimal::ZERO,
            Decimal::ONE,
            dec!(100) * step_change.powi(i.into()),
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
        );
        pool_states.insert(i, new_pool_state);

        step_pos_map.insert(
            i,
            (
                dec!(100) * step_change.powi(i.into()),
                Decimal::ZERO,
                Decimal::ZERO,
            ),
        );
    }

    add_liquidity_at_steps(
        &mut test_env,
        dec!(1000),
        "btc",
        dec!(5),
        steps,
        None
    )
    .run();
    pool_usd_btc.update();

    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(10000000));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(9999995));

    // Check the position
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn add_liquidity_at_steps_with_position() {
    let mut test_env = instantiate();
    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    add_liquidity_at_step(
        &mut test_env,
        dec!(100),
        "btc",
        dec!(1),
        50266,
        None,
    )
        .run();

    let mut steps = vec![];
    let mut pool_states = HashMap::new();
    let mut step_pos_map = HashMap::new();
    let step_change = dec!("1.000105411144423293");

    for i in 51000_u16..51005 {
        steps.push((i, dec!(200), Decimal::ONE));

        let new_pool_state = StepState::from(
            Decimal::ZERO,
            Decimal::ONE,
            dec!(100) * step_change.powi(i.into()),
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
        );
        pool_states.insert(i, new_pool_state);

        step_pos_map.insert(
            i,
            (
                dec!(100) * step_change.powi(i.into()),
                Decimal::ZERO,
                Decimal::ZERO,
            ),
        );
    }

    add_liquidity_at_steps(
        &mut test_env,
        dec!(1000),
        "btc",
        dec!(5),
        steps,
        Some("#0#".to_string())
    )
        .run();
    pool_usd_btc.update();

    let pool_state = StepState::from(
        dec!(100),
        Decimal::ZERO,
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    pool_states.insert(50266, pool_state);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(9999900));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(9999995));


    step_pos_map.insert(50266, (dec!(100), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn add_liquidity_at_rate_no_position() {
    let mut test_env = instantiate();
    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    add_liquidity(
        &mut test_env,
        dec!(100),
        "btc",
        dec!(1),
        dec!("2362.175"),
        None,
    )
    .run();
    pool_usd_btc.update();

    let pool_state = StepState::from(
        dec!(100),
        Decimal::ZERO,
        dec!("2362.1744661270945891"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(30000, pool_state);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(9999900));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(10000000));

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(30000, (dec!(100), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn add_liquidity_at_rate_with_position() {
    let mut test_env = instantiate();
    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    add_liquidity_at_step(&mut test_env, dec!(100), "btc", dec!(1), 30000, None).run();

    add_liquidity(
        &mut test_env,
        dec!(100),
        "btc",
        dec!(1),
        dec!("2362.175"),
        Some("#0#".to_string()),
    )
    .run();
    pool_usd_btc.update();

    // Rates are not correct due to small computational errors
    let pool_state = StepState::from(
        dec!(100),
        Decimal::ZERO,
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let pool_state_2 = StepState::from(
        dec!(100),
        Decimal::ZERO,
        dec!("2362.1744661270945891"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_states.insert(30000, pool_state_2);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(9999800));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(10000000));

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(30000, (dec!(100), Decimal::ZERO, Decimal::ZERO));
    step_pos_map.insert(50266, (dec!(100), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn remove_liquidity_at_step() {
    let mut test_env = instantiate();
    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    test_env.create_account("other");
    test_env.transfer_to("other", "usd", dec!(100));
    test_env.transfer_to("other", "btc", dec!(1));
    test_env.set_current_account("other");

    add_liquidity_at_step(&mut test_env, dec!(100), "btc", dec!(1), 30000, None).run();
    pool_usd_btc.update();

    test_env
        .call_method(RouterMethods::RemoveLiquidityAtStep(
            POSITION_NAME.to_string(),
            "#1#".to_string(),
            30000,
        ))
        .run();
    pool_usd_btc.update();

    // Rates are not correct due to small computational errors
    let pool_state = StepState::from(
        dec!(20000),
        Decimal::ZERO,
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let pool_state_2 = StepState::from(
        Decimal::ZERO,
        Decimal::ZERO,
        dec!("2362.1744661270945891"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_states.insert(30000, pool_state_2);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(100));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(1));

    // Check that the user got the right position
    let step_pos_map = HashMap::new();
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]/*
fn remove_liquidity_at_steps() {
    let mut test_env = instantiate();
    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    add_liquidity_at_steps(
        &mut test_env,
        dec!(1000),
        "btc",
        dec!(5),
        51000,
        51004,
        Some("#0#".to_string()),
    )
    .run();
    test_env
        .call_method(RouterMethods::RemoveLiquidityAtSteps(
            POSITION_NAME.to_string(),
            "#0#".to_string(),
            51000,
            51002,
        ))
        .run();
    pool_usd_btc.update();

    let mut pool_states = HashMap::new();
    let pool_state = StepState::from(
        dec!(20000),
        Decimal::ZERO,
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    pool_states.insert(50266, pool_state);
    let step_change = dec!("1.000105411144423293");
    for i in 51000_u16..51005 {
        let new_pool_state = StepState::from(
            Decimal::ZERO,
            if i < 51003 {
                Decimal::ZERO
            } else {
                Decimal::ONE
            },
            dec!(100) * step_change.powi(i.into()),
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
        );
        pool_states.insert(i, new_pool_state);
    }
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(9980000));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(9999998));

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();

    for i in 51003_u16..51005 {
        step_pos_map.insert(
            i,
            (
                dec!(100) * step_change.powi(i.into()),
                Decimal::ZERO,
                Decimal::ZERO,
            ),
        );
    }
    step_pos_map.insert(50266, (dec!(20000), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env, "btc", &step_pos_map);
}*/

#[test]
fn remove_liquidity_at_rate() {
    let mut test_env = instantiate();
    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );
    add_liquidity(
        &mut test_env,
        dec!(100),
        "btc",
        dec!(1),
        dec!("2362.175"),
        Some("#0#".to_string()),
    )
    .run();
    test_env
        .call_method(RouterMethods::RemoveLiquidityAtRate(
            POSITION_NAME.to_string(),
            "#0#".to_string(),
            dec!("2362.2"),
        ))
        .run();
    pool_usd_btc.update();

    // Rates are not correct due to small computational errors
    let pool_state = StepState::from(
        dec!(20000),
        Decimal::ZERO,
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let pool_state_2 = StepState::from(
        Decimal::ZERO,
        Decimal::ZERO,
        dec!("2362.1744661270945891"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_states.insert(30000, pool_state_2);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(9980000));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(10000000));

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(50266, (dec!(20000), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]/*
fn remove_all_liquidity() {
    let mut test_env = instantiate();
    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    add_liquidity_at_steps(
        &mut test_env,
        dec!(1000),
        "btc",
        dec!(5),
        51000,
        51004,
        Some("#0#".to_string()),
    )
    .run();
    test_env
        .call_method(RouterMethods::RemoveAllLiquidity(
            POSITION_NAME.to_string(),
            test_env
                .get_non_fungible_ids_owned_by_current(POSITION_NAME)
                .unwrap()
                .clone(),
        ))
        .run();
    pool_usd_btc.update();

    let mut pool_states = HashMap::new();
    let pool_state = StepState::from(
        Decimal::ZERO,
        Decimal::ZERO,
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    pool_states.insert(50266, pool_state);
    let step_change = dec!("1.000105411144423293");
    for i in 51000_u16..51005 {
        let new_pool_state = StepState::from(
            Decimal::ZERO,
            Decimal::ZERO,
            dec!(100) * step_change.powi(i.into()),
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
        );
        pool_states.insert(i, new_pool_state);
    }
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(10000000));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(10000000));

    // Check that the user does not have any position
    assert_no_positions(&test_env, "btc");
}*/

#[test]
fn remove_all_liquidity_from_multiple_pools() {
    let mut test_env = instantiate();

    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );
    test_env.create_fixed_supply_token("eth", dec!(10000000));
    let mut pool_usd_eth = create_pool(
        &mut test_env,
        "eth",
        dec!(1700),
        dec!(10),
        dec!(20000),
    );

    test_env
        .call_method(RouterMethods::RemoveAllLiquidity(
            POSITION_NAME.to_string(),
            test_env
                .get_non_fungible_ids_owned_by_current(POSITION_NAME)
                .unwrap()
                .clone(),
        ))
        .run();

    pool_usd_btc.update();
    let pool_state = StepState::from(
        Decimal::ZERO,
        Decimal::ZERO,
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    pool_usd_eth.update();
    let pool_state = StepState::from(
        Decimal::ZERO,
        Decimal::ZERO,
        dec!("1699.82907049827534548"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(44280, pool_state);
    pool_usd_eth.assert_state_is(
        dec!("1.000115989063276095"),
        44280,
        dec!(10),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(10000000));
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!(10000000));
    assert_eq!(test_env.amount_owned_by_current("eth"), dec!(10000000));

    // Check that the user does not have any position
    assert_no_positions(&test_env, "btc");
}

#[test]
fn swap_for_stable() {
    let mut test_env = instantiate();

    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    test_env
        .call_method(RouterMethods::Swap(
            "btc".to_string(),
            dec!("0.5"),
            "usd".to_string(),
        ))
        .run();
    pool_usd_btc.update();

    // Rate is not exactly 20000:1 because of computational errors
    let pool_state = StepState::from(
        dec!("10030.00026827474923125"),
        dec!("0.4985"),
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        dec!("0.0000000625"),
        Decimal::ZERO,
        dec!("0.00125"),
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        dec!("0.00025"),
    );

    // Check tokens owned by current account
    assert_eq!(
        test_env.amount_owned_by_current("usd"),
        dec!("9989969.99973172525076875")
    );
    assert_eq!(test_env.amount_owned_by_current("btc"), dec!("9999999.5"));
}

#[test]
fn swap_for_other() {
    let mut test_env = instantiate();

    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );
    add_liquidity_at_step(&mut test_env, dec!(100), "btc", dec!(10), 50267, None).run();

    test_env
        .call_method(RouterMethods::Swap(
            "usd".to_string(),
            dec!("1000"),
            "btc".to_string(),
        ))
        .run();
    pool_usd_btc.update();

    let pool_state = StepState::from(
        dec!(20000),
        Decimal::ZERO,
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let pool_state_2 = StepState::from(
        dec!("996.999999999999997323"),
        dec!("9.95015525285046674"),
        dec!(100) * dec!("1.000105411144423293").powi(50267),
        dec!("0.00001249868283589"),
        Decimal::ZERO,
        dec!("2.499999999999999993"),
        Decimal::ZERO,
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_states.insert(50267, pool_state_2);

    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50267,
        dec!(100),
        pool_states,
        dec!("0.499999999999999998"),
        Decimal::ZERO,
    );

    assert_eq!(
        test_env.amount_owned_by_current("usd"),
        dec!("9979000.000000000000002686")
    );
    assert_eq!(
        test_env.amount_owned_by_current("btc"),
        dec!("9999990.04984474714953326")
    );
}

#[test]
fn claim_fees_stable() {
    let mut test_env = instantiate();

    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );
    add_liquidity_at_step(
        &mut test_env,
        dec!(100),
        "btc",
        dec!(10),
        50267,
        Some("#0#".to_string()),
    )
    .run();

    test_env
        .call_method(RouterMethods::Swap(
            "usd".to_string(),
            dec!("1000"),
            "btc".to_string(),
        ))
        .run();
    test_env
        .call_method(RouterMethods::ClaimFees(
            POSITION_NAME.to_string(),
            test_env
                .get_non_fungible_ids_owned_by_current(POSITION_NAME)
                .unwrap()
                .clone(),
        ))
        .run();
    pool_usd_btc.update();

    let pool_state = StepState::from(
        dec!(20000),
        Decimal::ZERO,
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let pool_state_2 = StepState::from(
        dec!("996.999999999999997323"),
        dec!("9.95015525285046674"),
        dec!(100) * dec!("1.000105411144423293").powi(50267),
        dec!("0.00001249868283589"),
        Decimal::ZERO,
        dec!("0.000000000000197621"),
        Decimal::ZERO,
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_states.insert(50267, pool_state_2);

    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50267,
        dec!(100),
        pool_states,
        dec!("0.499999999999999998"),
        Decimal::ZERO,
    );

    assert_eq!(
        test_env.amount_owned_by_current("usd"),
        dec!("9979002.499999999999805058")
    );
    assert_eq!(
        test_env.amount_owned_by_current("btc"),
        dec!("9999990.04984474714953326")
    );

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(50266, (dec!(20000), Decimal::ZERO, Decimal::ZERO));
    step_pos_map.insert(
        50267,
        (
            dec!(10) * dec!(100) * dec!("1.000105411144423293").powi(50267),
            dec!("0.00001249868283589"),
            Decimal::ZERO,
        ),
    );
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn claim_fees_other() {
    let mut test_env = instantiate();

    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    test_env
        .call_method(RouterMethods::Swap(
            "btc".to_string(),
            dec!("0.5"),
            "usd".to_string(),
        ))
        .run();
    test_env
        .call_method(RouterMethods::ClaimFees(
            POSITION_NAME.to_string(),
            test_env
                .get_non_fungible_ids_owned_by_current(POSITION_NAME)
                .unwrap()
                .clone(),
        ))
        .run();
    pool_usd_btc.update();

    // Rate is not exactly 20000:1 because of computational errors
    let pool_state = StepState::from(
        dec!("10030.00026827474923125"),
        dec!("0.4985"),
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        dec!("0.0000000625"),
        Decimal::ZERO,
        Decimal::ZERO,
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        dec!("0.00025"),
    );

    // Check tokens owned by current account
    assert_eq!(
        test_env.amount_owned_by_current("usd"),
        dec!("9989969.99973172525076875")
    );
    assert_eq!(
        test_env.amount_owned_by_current("btc"),
        dec!("9999999.50125")
    );

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(50266, (dec!(20000), Decimal::ZERO, dec!("0.0000000625")));
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn claim_protocol_fees() {
    let mut test_env = instantiate();

    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );

    test_env
        .call_method(RouterMethods::Swap(
            "btc".to_string(),
            dec!("0.5"),
            "usd".to_string(),
        ))
        .run();
    test_env
        .call_method(RouterMethods::Swap(
            "usd".to_string(),
            dec!(9900),
            "btc".to_string(),
        ))
        .run();
    test_env.call_method(RouterMethods::ClaimProtocolFees).run();
    pool_usd_btc.update();

    // Rate is not exactly 20000:1 because of computational errors
    let pool_state = StepState::from(
        dec!("19900.300268274749218449"),
        dec!("0.004984986720399557"),
        dec!("19999.9994618360095662"),
        dec!("0.001237499999999999"),
        dec!("0.0000000625"),
        dec!("24.749999999999999967"),
        dec!("0.00125"),
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    // Check tokens owned by current account
    assert_eq!(
        test_env.amount_owned_by_current("usd"),
        dec!("9980074.949731725250781584")
    );
    assert_eq!(
        test_env.amount_owned_by_current("btc"),
        dec!("9999999.993765013279600443")
    );

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(50266, (dec!(20000), Decimal::ZERO, Decimal::ZERO));
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn add_liquidity_to_mixed_step_too_much_other() {
    let mut test_env = instantiate();

    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );
    test_env
        .call_method(RouterMethods::Swap(
            "btc".to_string(),
            dec!("0.5"),
            "usd".to_string(),
        ))
        .run();

    test_env.create_account("other");
    test_env.transfer_to("other", "usd", dec!(1000));
    test_env.transfer_to("other", "btc", Decimal::ONE);
    test_env.set_current_account("other");

    add_liquidity_at_step(&mut test_env, dec!(100), "btc", Decimal::ONE, 50266, None).run();
    pool_usd_btc.update();

    // Rate is not exactly 20000:1 because of computational errors
    let pool_state = StepState::from(
        dec!("10130.00026827474923125"),
        dec!("0.503470089597871431"),
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        dec!("0.0000000625"),
        Decimal::ZERO,
        dec!("0.00125"),
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        dec!("0.00025"),
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!("900"));
    assert_eq!(
        test_env.amount_owned_by_current("btc"),
        dec!("0.995029910402128569")
    );

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(
        50266,
        (
            dec!("199.401789282705369195"),
            Decimal::ZERO,
            dec!("0.0000000625"),
        ),
    );
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn add_liquidity_to_mixed_step_too_much_stable() {
    let mut test_env = instantiate();

    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );
    test_env
        .call_method(RouterMethods::Swap(
            "btc".to_string(),
            dec!("0.5"),
            "usd".to_string(),
        ))
        .run();

    test_env.create_account("other");
    test_env.transfer_to("other", "usd", dec!(100000));
    test_env.transfer_to("other", "btc", Decimal::ONE);
    test_env.set_current_account("other");

    add_liquidity_at_step(
        &mut test_env,
        dec!(100000),
        "btc",
        Decimal::ONE,
        50266,
        None,
    )
    .run();
    pool_usd_btc.update();

    // Rate is not exactly 20000:1 because of computational errors
    let pool_state = StepState::from(
        dec!("30150.361889688488867523"),
        dec!("1.4985"),
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        dec!("0.0000000625"),
        Decimal::ZERO,
        dec!("0.00125"),
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        dec!("0.00025"),
    );

    // Check tokens owned by current account
    assert_eq!(
        test_env.amount_owned_by_current("usd"),
        dec!("79879.638378586260363727")
    );
    assert_eq!(test_env.amount_owned_by_current("btc"), Decimal::ZERO);

    // Check that the user got the right position
    let mut step_pos_map = HashMap::new();
    step_pos_map.insert(
        50266,
        (
            dec!("40120.361083249749202473"),
            Decimal::ZERO,
            dec!("0.0000000625"),
        ),
    );
    assert_current_position(&test_env, "btc", &step_pos_map);
}

#[test]
fn remove_liquidity_from_mixed_step() {
    let mut test_env = instantiate();

    let mut pool_usd_btc = create_pool(
        &mut test_env,
        "btc",
        dec!(20000),
        dec!(100),
        dec!(100000),
    );
    test_env
        .call_method(RouterMethods::Swap(
            "btc".to_string(),
            dec!("0.5"),
            "usd".to_string(),
        ))
        .run();

    test_env.create_account("other");
    test_env.transfer_to("other", "usd", dec!(100000));
    test_env.transfer_to("other", "btc", Decimal::ONE);
    test_env.set_current_account("other");

    add_liquidity_at_step(
        &mut test_env,
        dec!(100000),
        "btc",
        Decimal::ONE,
        50266,
        None,
    )
    .run();
    test_env
        .call_method(RouterMethods::RemoveLiquidityAtStep(
            POSITION_NAME.to_string(),
            "#1#".to_string(),
            50266,
        ))
        .run();
    pool_usd_btc.update();

    // Rate is not exactly 20000:1 because of computational errors
    let pool_state = StepState::from(
        dec!("10030.00026827474923125"),
        dec!("0.498500000000000001"),
        dec!("19999.9994618360095662"),
        Decimal::ZERO,
        dec!("0.0000000625"),
        Decimal::ZERO,
        dec!("0.00125"),
    );
    let mut pool_states = HashMap::new();
    pool_states.insert(50266, pool_state);
    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        dec!("0.00025"),
    );

    // Check tokens owned by current account
    assert_eq!(test_env.amount_owned_by_current("usd"), dec!(100000));
    assert_eq!(
        test_env.amount_owned_by_current("btc"),
        dec!("0.999999999999999999")
    );

    // Check that the user got the right position
    let step_pos_map = HashMap::new();
    assert_current_position(&test_env, "btc", &step_pos_map);
}
