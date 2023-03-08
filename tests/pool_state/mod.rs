use std::collections::HashMap;
use std::process::Command;
use scrypto::prelude::{Decimal};

pub struct StepState {
    x: Decimal,
    y: Decimal,
    rate: Decimal,
    x_fees_per_liq: Decimal,
    y_fees_per_liq: Decimal,
    x_fees: Decimal,
    y_fees: Decimal
}

impl StepState {}

pub struct PoolState {
    router_address: String,
    token_x: String,
    token_y: String,
    rate_step: Decimal,
    current_step: u16,
    min_rate: Decimal,
    steps: HashMap<u16, StepState>,
    x_protocol: Decimal,
    y_protocol: Decimal,
}

impl PoolState {

    pub fn from(router_address: String, token_x: String, token_y: String) -> Self {
        PoolState {
            router_address,
            token_x,
            token_y,
            rate_step: Decimal::ZERO,
            current_step: 0,
            min_rate: Decimal::ZERO,
            steps: HashMap::new(),
            x_protocol: Decimal::ZERO,
            y_protocol: Decimal::ZERO
        }
    }

}

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