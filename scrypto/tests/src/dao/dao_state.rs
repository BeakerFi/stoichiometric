use std::collections::HashMap;
use scrypto::prelude::Decimal;
use crate::dex::pool_state::PoolState;
use crate::stablecoin::issuer_state::IssuerState;

pub struct ProposalState {}

pub struct DaoState {
    component_address: String,
    pub issuer_state: IssuerState,
    pub pool_states: HashMap<String, PoolState>,
    pub proposals: HashMap<u64, ProposalState>,
    pub proposal_id: u64,
    pub total_voting_power: Decimal,
    pub vote_period: i64,
    pub vote_validity_threshold: Decimal,
    pub reserves: HashMap<String, Decimal>

}