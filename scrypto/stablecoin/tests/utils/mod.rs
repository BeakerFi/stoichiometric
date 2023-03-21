use std::process::Command;
use scrypto::math::Decimal;
use lazy_static::lazy_static;
use regex::Regex;
use scrypto::prelude::dec;
use sqrt::method::Arg::{FungibleBucketArg, ResourceAddressArg};
use sqrt::package::Package;
use sqrt::test_environment::TestEnvironment;
use crate::dumb_oracle_sqrt::{DumbOracleBlueprint, DumbOracleMethods};
use crate::issuer_sqrt::{ADMIN_BADGE_NAME, IssuerBlueprint, IssuerMethods, STABLECOIN_NAME};
use crate::issuer_state::{IssuerState};

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
    test_env.create_fixed_supply_token(ADMIN_BADGE_NAME, dec!(2));

    test_env.create_fixed_supply_token("btc", dec!(10000000));
    test_env.create_mintable_token(STABLECOIN_NAME, ADMIN_BADGE_NAME);

    let issuer_blueprint = Box::new(IssuerBlueprint {});
    let mut issuer_package = Package::new(".");
    issuer_package.add_blueprint("issuer_bp", issuer_blueprint);
    test_env.publish_package("issuer", issuer_package);
    test_env.new_component(
        "issuer_comp",
        "issuer_bp",
        vec![
            ResourceAddressArg(ADMIN_BADGE_NAME.to_string()),
            FungibleBucketArg(ADMIN_BADGE_NAME.to_string(), Decimal::ONE),
            ResourceAddressArg(STABLECOIN_NAME.to_string())
        ]
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

pub fn assert_current_has_loan(test_env: &TestEnvironment, loan_id: &str, collateral_token: &str, collateral_amount: Decimal, amount_lent: Decimal, loan_date: i64, loan_to_value: Decimal, interest_rate: Decimal) {

    let current_account = test_env.get_current_account_address();
    let output = run_command(Command::new("resim").arg("show").arg(current_account));

    lazy_static!{
        static ref LOAN_RE: Regex = Regex::new(r#"NonFungible \{ id: NonFungibleLocalId\("(.)*"\), immutable_data: Tuple\(ResourceAddress\("(\w*)"\), (\w*)i64, Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)\), mutable_data: Tuple\(Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)\) \}"#).unwrap();
    }

    for loan_capture in LOAN_RE.captures_iter(&output) {

        if loan_id.to_string() == String::from(&loan_capture[1]) {


            let collateral_token_found = String::from(&loan_capture[2]);
            let loan_date_found = String::from(&loan_capture[3]).parse::<i64>().unwrap();
            let loan_to_value_found = Decimal::from(&loan_capture[4]);
            let interest_rate_found = Decimal::from(&loan_capture[5]);
            let collateral_amount_found = Decimal::from(&loan_capture[6]);
            let amount_lent_found = Decimal::from(&loan_capture[7]);

            assert_eq!(test_env.get_resource(collateral_token).clone(), collateral_token_found);
            assert_eq!(collateral_amount, collateral_amount_found);
            assert_eq!(amount_lent, amount_lent_found);
            assert_eq!(loan_date, loan_date_found);
            assert_eq!(loan_to_value, loan_to_value_found);
            assert_eq!(interest_rate, interest_rate_found);

            return;
        }
    }

}

pub fn assert_current_has_no_loan_id(test_env: &TestEnvironment, loan_id: &str) {

    let current_account = test_env.get_current_account_address();
    let output = run_command(Command::new("resim").arg("show").arg(current_account));

    lazy_static!{
        static ref LOAN_RE: Regex = Regex::new(r#"NonFungible \{ id: NonFungibleLocalId\("(.)*"\), immutable_data: Tuple\(ResourceAddress\("(\w*)"\), (\w*)i64, Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)\), mutable_data: Tuple\(Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)\) \}"#).unwrap();
    }

    for loan_capture in LOAN_RE.captures_iter(&output) {
        let loan_id_found = &loan_capture[1];
        assert_ne!(loan_id_found.to_string(), loan_id.to_string());
    }
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