use std::process::Command;
use scrypto::prelude::{dec, Decimal};
use sqrt::blueprint::Blueprint;
use sqrt::method::{Arg, Method};
use sqrt::method::Arg::{DecimalArg, FungibleBucketArg, NonFungibleProofArg, ResourceAddressArg, U16};
use sqrt::method_args;
use sqrt::package::Package;
use sqrt::test_environment::TestEnvironment;
use crate::pool_state::{PoolState, run_command};
use lazy_static::lazy_static;
use regex::Regex;

pub(crate) const POSITION_NAME: &str = "Stoichiometric Position";
pub(crate) const ADMIN_BADGE_NAME: &str = "Router admin badge";

pub struct RouterBlueprint {}

impl Blueprint for RouterBlueprint {
    fn instantiation_name(&self) -> &str {
        "new"
    }

    fn name(&self) -> &str {
        "Router"
    }

    fn has_admin_badge(&self) -> bool {
        true
    }
}

pub enum RouterMethods {
    CreatePool(String, Decimal, String, Decimal, Decimal, Decimal),
    RemoveLiquidityAtStep(String, String, u16),
    RemoveLiquidityAtSteps(String, String, u16, u16),
    RemoveLiquidityAtRate(String, String, Decimal),
    RemoveLiquidityAtRates(String, String, Decimal, Decimal),
    RemoveAllLiquidity(String, Vec<String>),
    ClaimFees(String, Vec<String>),
    Swap(String, Decimal, String)
}

impl Method for RouterMethods {

    fn name(&self) -> &str {
        match self {
            RouterMethods::CreatePool(_, _, _, _, _, _) => { "create_pool" }
            RouterMethods::RemoveLiquidityAtStep(_, _, _) => { "remove_liquidity_at_step" }
            RouterMethods::RemoveLiquidityAtSteps(_, _, _, _) => { "remove_liquidity_at_steps" }
            RouterMethods::RemoveLiquidityAtRate(_, _, _) => { "remove_liquidity_at_rate" }
            RouterMethods::RemoveLiquidityAtRates(_, _, _, _) => { "remove_liquidity_at_rates" }
            RouterMethods::RemoveAllLiquidity(_, _) => { "remove_all_liquidity" }
            RouterMethods::ClaimFees(_, _) => { "claim_fees" }
            RouterMethods::Swap(_, _, _) => { "swap" }
        }
    }

    fn args(&self) -> Option<Vec<Arg>> {
        match self {
            RouterMethods::CreatePool(token_a, token_a_amount, token_b, token_b_amount, min_rate, max_rate) =>
                {
                    method_args!(
                        FungibleBucketArg(token_a.clone(), token_a_amount.clone()),
                        FungibleBucketArg(token_b.clone(), token_b_amount.clone()),
                        DecimalArg(min_rate.clone()),
                        DecimalArg(max_rate.clone())
                    )
                }
            RouterMethods::RemoveLiquidityAtStep(position, position_id, step) =>
                {
                    method_args!(
                        NonFungibleProofArg(position.clone(), vec![position_id.clone()]),
                        U16(step.clone())
                    )
                }
            RouterMethods::RemoveLiquidityAtSteps(position, position_id, start_step, stop_step) =>
                {
                    method_args!(
                        NonFungibleProofArg(position.clone(), vec![position_id.clone()]),
                        U16(start_step.clone()),
                        U16(stop_step.clone())
                    )
                }
            RouterMethods::RemoveLiquidityAtRate(position,position_id, rate) =>
                {
                    method_args!(
                        NonFungibleProofArg(position.clone(), vec![position_id.clone()]),
                        DecimalArg(rate.clone())
                    )
                }
            RouterMethods::RemoveLiquidityAtRates(position, position_id, min_rate, max_rate) =>
                {
                    method_args!(
                        NonFungibleProofArg(position.clone(), vec![position_id.clone()]),
                        DecimalArg(min_rate.clone()),
                        DecimalArg(max_rate.clone())
                    )
                }
            RouterMethods::RemoveAllLiquidity(position, position_ids) =>
                {
                    method_args!(
                        NonFungibleProofArg(position.clone(), position_ids.clone())
                    )
                }
            RouterMethods::ClaimFees(position, position_ids) =>
                {
                    method_args!(
                        NonFungibleProofArg(position.clone(), position_ids.clone())
                    )
                }
            RouterMethods::Swap(token_input, amount_input, token_output) =>
                {
                    method_args!(
                        FungibleBucketArg(token_input.clone(), amount_input.clone()),
                        ResourceAddressArg(token_output.clone())
                    )
                }
        }
    }

    fn needs_admin_badge(&self) -> bool {
        match self {
            RouterMethods::CreatePool(_,_,_,_, _, _) => true,
            _ => false
        }
    }
}

pub fn instantiate() -> TestEnvironment {
    let mut test_env = TestEnvironment::new();
    let router_blueprint = Box::new(RouterBlueprint {});
    let mut router_package = Package::new(".");
    router_package.add_blueprint("router_bp", router_blueprint);
    test_env.publish_package("router", router_package);
    test_env.create_fixed_supply_token("usd", dec!(10000000));
    test_env.create_fixed_supply_token("btc", dec!(10000000));
    test_env.new_component("router_comp", "router_bp", vec![]);
    test_env
}

pub fn create_pool(test_env: &mut TestEnvironment, token_a: &str, token_a_amount: Decimal, token_b: &str, token_b_amount: Decimal, min_rate: Decimal, max_rate: Decimal) -> PoolState
{
    test_env.call_method( RouterMethods::CreatePool(
        token_a.to_string(),
        token_a_amount,
        token_b.to_string(),
        token_b_amount,
        min_rate,
        max_rate
    )).run();

    let mut pool_state: PoolState = PoolState::from(String::new(), String::new(), String::new());

    let router_address = test_env.get_component("router_comp").unwrap();
    let token_a_address = test_env.get_resource(token_a).clone();
    let token_b_address = test_env.get_resource(token_b).clone();

    let output = run_command(Command::new("resim").arg("show").arg(router_address));

    lazy_static! {
        static ref POOLS_LIST_RE: Regex = Regex::new(r#"Map<Tuple, Tuple>\((.*), Tuple\(Own"#).unwrap();
    }

    let pools_list_cap = &POOLS_LIST_RE.captures(&output).expect("Could not find pools list");
    let pools_list = &pools_list_cap[1];

    lazy_static! {
        static ref POOLS_RE: Regex = Regex::new(r#"Tuple\(ResourceAddress\("(\w*)"\), ResourceAddress\("(\w*)"\)\)"#).unwrap();
    }

    for cap in POOLS_RE.captures_iter(pools_list) {

        let first_res = String::from(&cap[1]);
        let second_res = String::from(&cap[2]);

        if first_res == token_a_address && second_res == token_b_address
        {
            pool_state = PoolState::from(router_address.to_string(), token_a_address, token_b_address);
            break;
        }
        else if first_res == token_b_address && second_res == token_a_address
        {
            pool_state = PoolState::from(router_address.to_string(), token_b_address, token_a_address);
            break;
        }
    }

    //pool_state.update();
    pool_state
}

