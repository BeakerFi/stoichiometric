use std::collections::HashMap;
use std::process::Command;
use lazy_static::lazy_static;
use regex::Regex;
use scrypto::prelude::Decimal;
use crate::dex::pool_state::PoolState;
use crate::stablecoin::issuer_state::IssuerState;
use crate::utils::run_command;

pub struct ProposalState {}

pub struct DaoState {
    dao_address: String,
    pub router_address: String,
    pub issuer_address: String,
    pub issuer_state: IssuerState,
    pub pool_states: HashMap<String, PoolState>,
    pub voter_card_id: u64,
    pub proposals: HashMap<u64, ProposalState>,
    pub proposal_id: u64,
    pub total_voting_power: Decimal,
    pub vote_period: i64,
    pub vote_validity_threshold: Decimal,
    pub reserves: HashMap<String, Decimal>
}

impl DaoState {

    pub fn new(component_address: String, initial_token: String) -> Self {

        Self::from_prompt(component_address, initial_token)

    }

    pub fn update(&mut self) {

        let new_states = Self::from_prompt(self.dao_address.clone(), String::new());

        self.voter_card_id = new_states.voter_card_id;
        self.proposal_id = new_states.voter_card_id;
        self.total_voting_power = new_states.total_voting_power;
        self.vote_period = new_states.vote_period;
        self.vote_validity_threshold = new_states.vote_validity_threshold;

        self.update_issuer_state();
        self.update_pool_states();
        self.update_proposals();
        self.update_reserves();
    }

    fn from_prompt(component_address: String, initial_token: String) -> Self {

        let output = run_command(Command::new("resim").arg("show").arg(component_address.clone()));

        lazy_static!{
            static ref DAO_STATE_RE: Regex = Regex::new(r#"ComponentAddress\("(\w*)"\), ComponentAddress\("(\w*)"\), ResourceAddress\("(\w*)"\), Own\("(.*)"\), ResourceAddress\("(\w*)"\), ResourceAddress\("(\w*)"\), (\w*)u64, ResourceAddress\("(\w*)"\), Own\("(.*)"\), Map<U64, ComponentAddress>\((.*)\), (\w*)u64, Own\("(.*)"\), Decimal\("([\d.]*)"\), (\w*)i64, Decimal\("([\d.]*)"\)"#).unwrap();
        }

        let dao_cap = &DAO_STATE_RE.captures(&output).unwrap();

        let dex_comp = String::from(&dao_cap[1]);
        let issuer_comp = String::from(&dao_cap[2]);
        let voter_card_id = String::from(&dao_cap[7]).parse::<u64>().unwrap();
        let proposal_id = String::from(&dao_cap[11]).parse::<u64>().unwrap();
        let total_voting_power = Decimal::from(&dao_cap[13]);
        let vote_period = String::from(&dao_cap[14]).parse::<i64>().unwrap();
        let vote_validity_threshold =Decimal::from(&dao_cap[15]);

        let issuer_state = IssuerState::from(issuer_comp.clone());
        let pool_state = PoolState::from(dex_comp.clone(), initial_token.clone());
        let mut pool_states = HashMap::new();
        pool_states.insert(initial_token, pool_state);

        Self{
            dao_address: component_address,
            router_address: dex_comp,
            issuer_address: issuer_comp,
            issuer_state,
            pool_states,
            voter_card_id,
            proposals: HashMap::new(),
            proposal_id,
            total_voting_power,
            vote_period,
            vote_validity_threshold,
            reserves: HashMap::new()
        }

    }


    fn update_pool_states(&mut self) {

        for (_, pool_state) in &mut self.pool_states {
            pool_state.update();
        }

    }

    fn update_issuer_state(&mut self) {
        self.issuer_state.update();
    }

    fn update_proposals(&mut self) {

    }

    fn update_reserves(&mut self) {

        let output = run_command(Command::new("resim").arg("show").arg(self.dao_address.clone()));

        lazy_static! {
            static ref RESOURCE_RE: Regex =
                Regex::new(r#"\{ amount: ([\d.]*), resource address: (\w*)"#).unwrap();
        }

        let mut reserves = HashMap::new();

        for resource_capture in RESOURCE_RE.captures_iter(&output) {
            let amount = Decimal::from(&resource_capture[1]);
            let resource = String::from(&resource_capture[2]);

            if amount != Decimal::ONE && !amount.is_zero(){
                reserves.insert(resource, amount);
            }
        }

        self.reserves = reserves;
    }
}