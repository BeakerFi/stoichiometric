use std::collections::HashMap;
use std::process::Command;
use lazy_static::lazy_static;
use regex::Regex;
use scrypto::prelude::{dec, Decimal};
use sqrt::method::Arg::ResourceAddressArg;
use sqrt::package::Package;
use sqrt::test_environment::TestEnvironment;
use crate::pool_state::{PoolState};
use crate::router_sqrt::{RouterBlueprint, RouterMethods};

pub fn run_command(command: &mut Command) -> String {
    let output = command.output().expect("Failed to run command line");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    if !output.status.success() {
        println!("stdout:\n{}", stdout);
        panic!("{}", stderr);
    }
    stdout
}

pub fn instantiate() -> TestEnvironment {
    let mut test_env = TestEnvironment::new();
    let router_blueprint = Box::new(RouterBlueprint {});
    let mut router_package = Package::new(".");
    router_package.add_blueprint("router_bp", router_blueprint);
    test_env.publish_package("router", router_package);
    test_env.create_fixed_supply_token("usd", dec!(10000000));
    test_env.create_fixed_supply_token("btc", dec!(10000000));
    test_env.new_component("router_comp", "router_bp", vec![ResourceAddressArg("usd".to_string())]);
    test_env
}

pub fn create_pool(test_env: &mut TestEnvironment, stable_amount: Decimal, other: &str, other_amount: Decimal, min_rate: Decimal, max_rate: Decimal) -> PoolState
{
    test_env.call_method( RouterMethods::CreatePool(
        "usd".to_string(),
        stable_amount,
        other.to_string(),
        other_amount,
        stable_amount / other_amount,
        min_rate,
        max_rate
    )).run();

    let mut pool_state: PoolState = PoolState::from(String::new(), String::new());

    let router_address = test_env.get_component("router_comp").unwrap();
    let other_address = test_env.get_resource(other).clone();

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
            pool_state = PoolState::from(router_address.to_string(),other_address);
            break;
        }
    }

    pool_state.update();
    pool_state
}

pub fn assert_current_position(test_env: &TestEnvironment, token: &str, step_positions: &HashMap<u16, (Decimal, Decimal, Decimal)>)
{
    let output = run_command(Command::new("resim").arg("show").arg(test_env.get_current_account_address()));

    lazy_static!{
        static ref POSITIONS_RE: Regex = Regex::new(r#"NonFungible \{ id: NonFungibleLocalId\("(.*)"\), immutable_data: Tuple\(ResourceAddress\("(\w*)"\)\), mutable_data: Tuple\(Map<U16, Tuple>\((.*)\) \}"#).unwrap();
    }

    let mut position_found = false;
    for position_cap in POSITIONS_RE.captures_iter(&output)
    {
        let token_address = String::from(&position_cap[2]);
        if &token_address == test_env.get_resource(token)
        {
            position_found = true;
            assert_step_positions(&position_cap[3], step_positions);
        }
    }

    assert!(position_found);
}

fn assert_step_positions(output_str: &str, step_positions: &HashMap<u16, (Decimal, Decimal, Decimal)>)
{
    lazy_static!{
        static ref STEP_POSITION_RE: Regex = Regex::new(r#"(\w*)u16, Tuple\(Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)"#).unwrap();
    }

    let mut new_hashmap = HashMap::new();

    for step_position_cap in STEP_POSITION_RE.captures_iter(output_str)
    {
        let step_id: u16 = String::from(&step_position_cap[1]).parse::<u16>().unwrap();
        let liquidity = Decimal::from(&step_position_cap[2]);
        let last_stable_fees_per_liq = Decimal::from(&step_position_cap[3]);
        let last_other_fees_per_liq = Decimal::from(&step_position_cap[4]);
        new_hashmap.insert(step_id, (liquidity, last_stable_fees_per_liq, last_other_fees_per_liq));
    }

    assert!(new_hashmap.len() == step_positions.len() && new_hashmap.keys().all(|k| step_positions.contains_key(k)));

    for (key, value) in new_hashmap
    {
        let value_2 = step_positions.get(&key).unwrap();
        assert_eq!(value, *value_2);
    }
}