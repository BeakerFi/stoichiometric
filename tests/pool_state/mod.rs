use std::cmp::min;
use std::collections::HashMap;
use std::process::Command;
use lazy_static::lazy_static;
use regex::Regex;
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

impl StepState {

    pub fn from(x: Decimal, y: Decimal, rate: Decimal, x_fees_per_liq: Decimal, y_fees_per_liq: Decimal, x_fees: Decimal, y_fees: Decimal) -> Self {
        Self {
            x,
            y,
            rate,
            x_fees_per_liq,
            y_fees_per_liq,
            x_fees,
            y_fees
        }
    }

    pub fn from_output(str_output: &str) -> HashMap<u16, Self>
    {
        let mut steps = HashMap::new();

        lazy_static! {
            static ref STEP_STATE_RE: Regex = Regex::new(r#"Tuple\((\w*)u16, Array<Decimal>\(Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)\)\)"#).unwrap();
        }

        for step_cap in STEP_STATE_RE.captures_iter(str_output)
        {
            let step = String::from(&step_cap[1]).parse::<u16>().unwrap();
            let step_state = StepState {
                x: Decimal::from(&step_cap[2]),
                y: Decimal::from(&step_cap[3]),
                rate: Decimal::from(&step_cap[4]),
                x_fees_per_liq: Decimal::from(&step_cap[5]),
                y_fees_per_liq: Decimal::from(&step_cap[6]),
                x_fees: Decimal::from(&step_cap[7]),
                y_fees: Decimal::from(&step_cap[8]),
            };

            steps.insert(step, step_state);
        }

        steps
    }

    fn assert_step_state(&self, other_step: &StepState, inverted: bool) {
        assert_eq!(self.rate, other_step.rate);

        if !inverted {
            assert_eq!(self.x, other_step.x);
            assert_eq!(self.y, other_step.y);
            assert_eq!(self.x_fees_per_liq, other_step.x_fees_per_liq);
            assert_eq!(self.y_fees_per_liq, other_step.y_fees_per_liq);
            assert_eq!(self.x_fees, other_step.x_fees);
            assert_eq!(self.y_fees, other_step.y_fees);
        }
        else {
            assert_eq!(self.x, other_step.y);
            assert_eq!(self.y, other_step.x);
            assert_eq!(self.x_fees_per_liq, other_step.y_fees_per_liq);
            assert_eq!(self.y_fees_per_liq, other_step.x_fees_per_liq);
            assert_eq!(self.x_fees, other_step.y_fees);
            assert_eq!(self.y_fees, other_step.x_fees);
        }
    }

    pub fn assert_step_states(steps_states_1: &HashMap<u16, StepState>, step_states_2: &HashMap<u16, StepState>, inverted: bool) {
        // Checks that both maps have the same amount of keys and that the keys match
        assert!(steps_states_1.len() == step_states_2.len() && steps_states_1.keys().all(|k| step_states_2.contains_key(k)));

        for (key,value) in step_states_2 {

            let state_2 = step_states_2.get(key).unwrap();
            value.assert_step_state(state_2, inverted);
        }
    }
}

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
    inverted: bool
}

impl PoolState {

    pub fn from(router_address: String, token_x: String, token_y: String, inverted: bool) -> Self {
        PoolState {
            router_address,
            token_x,
            token_y,
            rate_step: Decimal::ZERO,
            current_step: 0,
            min_rate: Decimal::ZERO,
            steps: HashMap::new(),
            x_protocol: Decimal::ZERO,
            y_protocol: Decimal::ZERO,
            inverted
        }
    }

    pub fn update(&mut self) {
        let output = run_command(Command::new("resim").arg("call-method").arg(&self.router_address).arg("get_pool_state").arg(&self.token_x).arg(&self.token_y));

        lazy_static! {
            static ref STATE_MATCH_RE: Regex = Regex::new(r#"├─ Tuple\(Decimal\("([\d.]*)"\), (\w*)u16, Decimal\("([\d.]*)"\), Tuple\(Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)\), Array<Tuple>\((.*)\)"#).unwrap();
        }
        let capture = &STATE_MATCH_RE.captures(&output).unwrap();
        self.rate_step = Decimal::from(&capture[1]);
        self.current_step = String::from(&capture[2]).parse::<u16>().unwrap();
        self.min_rate = Decimal::from(&capture[3]);
        self.x_protocol = Decimal::from(&capture[4]);
        self.y_protocol = Decimal::from(&capture[5]);
        self.steps = StepState::from_output(&capture[6]);
    }

    pub fn assert_state_is(&self, rate_step: Decimal, current_step : u16, min_rate: Decimal, steps: HashMap<u16, StepState>, a_protocol: Decimal, b_protocol: Decimal) {
        assert_eq!(self.rate_step, rate_step);
        assert_eq!(self.min_rate, min_rate);
        StepState::assert_step_states(&self.steps, &steps, self.inverted);

        if !self.inverted {
            assert_eq!(self.current_step, current_step);
            assert_eq!(self.x_protocol, a_protocol);
            assert_eq!(self.y_protocol, b_protocol);
        }
        else {
            assert_eq!(self.x_protocol, b_protocol);
            assert_eq!(self.y_protocol, a_protocol);
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