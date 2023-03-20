use scrypto::prelude::*;
use stoichiometric_dex::position::Position;
use crate::utils::get_position_voting_power;

#[derive(NonFungibleData, ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub struct VoterCard {
    pub voting_power: Decimal,
    pub stablecoins_locked: Decimal,
    pub positions_locked_ids: Vec<NonFungibleLocalId>,
    pub last_proposal_voted_id: u64,
    pub proposals_voted: HashSet<u64>
}

impl VoterCard {

    pub fn new() -> Self {
        Self{
            voting_power: Decimal::ZERO,
            stablecoins_locked: Decimal::ZERO,
            positions_locked_ids: Vec::new(),
            last_proposal_voted_id: 0,
            proposals_voted: HashSet::new()
        }
    }

    pub fn add_stablecoins(&mut self, amount: Decimal) {
        self.voting_power += amount;
        self.stablecoins_locked += amount;
    }

    pub fn add_position(&mut self, position: &Position, id: NonFungibleLocalId) {
        self.voting_power += get_position_voting_power(position);
        self.positions_locked_ids.push(id);
    }

    pub fn add_proposals_to_voted(&mut self, proposal_id: u64) -> bool {
        let did_not_contained = self.proposals_voted.insert(proposal_id);

        if did_not_contained { self.last_proposal_voted_id = proposal_id };

        did_not_contained
    }
}