use scrypto::blueprint;

#[blueprint]
mod proposal {

    use crate::proposed_change::ProposedChange;
    use crate::voter_card::VoterCard;
    use crate::proposal_status::ProposalStatus;

    pub struct Proposal{
        proposal_id: u64,
        proposal_status: ProposalStatus,
        vote_end: i64,
        votes_for: Decimal,
        votes_against: Decimal,
        votes_threshold: Decimal,
        proposed_change: ProposedChange,
        voter_card_address: ResourceAddress,
        voter_card_updater: Vault
    }

    impl Proposal{

        pub fn new(proposal_id: u64, vote_end: i64, votes_for: Decimal, votes_against: Decimal, votes_threshold: Decimal, proposed_change: ProposedChange, voter_card_address: ResourceAddress,voter_card_updater: Bucket) -> ComponentAddress
        {
            let proposal_rules = AccessRules::new()
                .default(
                    rule!(allow_all),
                    AccessRule::DenyAll,
                );

            let mut component = Self{
                proposal_id,
                proposal_status: ProposalStatus::VotingStage,
                vote_end,
                votes_for,
                votes_against,
                votes_threshold,
                proposed_change,
                voter_card_address,
                voter_card_updater: Vault::with_bucket(voter_card_updater)
            }.instantiate();

            component.add_access_check(proposal_rules);

            component.globalize()
        }

        pub fn vote_for(&mut self, voter_card_proof: Proof) {
            self.vote(voter_card_proof, true);
        }

        pub fn vote_against(&mut self, voter_card_proof: Proof) {
            self.vote(voter_card_proof, false);
        }

        pub fn is_voting_stage(&self) -> bool {
            self.proposal_status.is_voting_stage()
        }

        pub fn is_accepted(&self) -> bool {
            self.proposal_status.is_accepted()
        }

        fn vote(&mut self, voter_card_proof: Proof, vote_for: bool)  {
            let current_time = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            assert!(current_time <= self.vote_end, "Cannot vote for this proposal anymore!");

            let validated_proof = voter_card_proof.validate_proof(ProofValidationMode::ValidateResourceAddress(
                self.voter_card_address
            )).expect("Please provide a valid proof of your voter card(s)");

            let mut voting_power = Decimal::ZERO;
            for voter_card in validated_proof.non_fungibles::<VoterCard>() {
                let mut data: VoterCard = borrow_resource_manager!(self.voter_card_address).get_non_fungible_data(voter_card.local_id());
                let did_not_contained = data.add_proposals_to_voted(self.proposal_id);

                if did_not_contained {
                    voting_power += data.voting_power;
                    self.voter_card_updater.authorize(|| {
                        borrow_resource_manager!(self.voter_card_address).update_non_fungible_data(voter_card.local_id(), data)
                    });
                }
            }

            if vote_for { self.votes_for += voting_power } else { self.votes_against += voting_power };
        }
    }

}