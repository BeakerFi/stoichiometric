use std::process::Command;
use scrypto::math::Decimal;
use scrypto::prelude::dec;
use sqrt::package::Package;
use sqrt::test_environment::TestEnvironment;
use crate::dumb_oracle_sqrt::{DumbOracleBlueprint, DumbOracleMethods};
use crate::issuer_sqrt::{IssuerBlueprint, IssuerMethods};
use crate::issuer_state::{IssuerState, LenderState};

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

pub fn instantiate() -> (TestEnvironment, IssuerState) {
    let mut test_env = TestEnvironment::new();
    test_env.create_fixed_supply_token("btc", dec!(10000000));

    let issuer_blueprint = Box::new(IssuerBlueprint {});
    let mut issuer_package = Package::new(".");
    issuer_package.add_blueprint("issuer_bp", issuer_blueprint);
    test_env.publish_package("issuer", issuer_package);
    test_env.new_component(
        "issuer_comp",
        "issuer_bp",
        vec![]
    );

    let oracle_blueprint = Box::new(DumbOracleBlueprint {});
    let mut oracle_package = Package::new("tests/dumb_oracle/package/");
    oracle_package.add_blueprint("oracle_bp", oracle_blueprint);
    test_env.publish_package("oracle", oracle_package);

    let issuer_address = test_env.get_component("issuer_comp").unwrap();
    let mut issuer_state = IssuerState::from(issuer_address.to_string());
    issuer_state.update();

    (test_env, issuer_state)
}

pub fn new_default_lender(
    test_env: &mut TestEnvironment,
    token: &str,
)
{
    let component_name = new_oracle(test_env, token);

    test_env.call_method(IssuerMethods::NewLender(
        token.to_string(),
        dec!("0.7"),
        dec!("0.0001"),
        dec!("1.3"),
        dec!("0.1"),
        component_name
    )).run();

}

pub fn set_oracle_price(test_env: &mut TestEnvironment, token: &str, new_price: Decimal) {

    let current_component = test_env.get_current_component_name().to_string();
    let current_package = test_env.get_current_package_name().to_string();
    let oracle_component_name = format!("{}_component", token);

    test_env.set_current_package("oracle");
    test_env.set_current_component(&oracle_component_name);
    test_env.call_method(DumbOracleMethods::SetPrice(new_price)).run();

    test_env.set_current_package(&current_package);
    test_env.set_current_component(&current_component);
}

fn new_oracle(test_env: &mut TestEnvironment, token: &str) -> String {

    let current_component = test_env.get_current_component_name().to_string();
    let current_package = test_env.get_current_package_name().to_string();
    let oracle_component_name = format!("{}_component", token);

    test_env.set_current_package("oracle");
    test_env.new_component(&oracle_component_name, "oracle_bp", vec![]);

    test_env.set_current_package(&current_package);
    test_env.set_current_component(&current_component);

    oracle_component_name
}