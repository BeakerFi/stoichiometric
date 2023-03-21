use scrypto::{blueprint, external_component};

external_component! {
    ProposalLocalComponent {
        fn vote_for(&mut self, voter_card_proof: Proof);
        fn vote_against(&mut self, voter_card_proof: Proof);
        fn is_voting_stage(&self) -> bool;
        fn execute(&mut self) -> Option<ProposedChange>;
    }
}

external_component! {
    RouterLocalComponent {
        fn create_pool(&mut self, token: ResourceAddress, initial_rate: Decimal, min_rate: Decimal, max_rate: Decimal);
        fn claim_protocol_fees(&mut self) -> Vec<Bucket>;
    }
}

external_component! {
    IssuerLocalComponent {
        fn new_lender(&mut self, collateral_address: ResourceAddress, loan_to_value: Decimal, interest_rate: Decimal, liquidation_threshold: Decimal, liquidation_incentive: Decimal, oracle: ComponentAddress);
        fn change_lender_parameters(&mut self, lender_collateral: ResourceAddress, loan_to_value: Decimal, interest_rate: Decimal, liquidation_threshold: Decimal, liquidation_incentive: Decimal);
        fn change_lender_oracle(&mut self, lender_collateral: ResourceAddress, oracle: ComponentAddress);
        fn give_tokens(&mut self, tokens: Vec<Bucket>);
    }
}


#[blueprint]
mod dao {
    use stoichiometric_dex::router::RouterComponent;
    use stoichiometric_dex::position::Position;
    use stoichiometric_stablecoin::issuer::IssuerComponent;
    use crate::voter_card::VoterCard;
    use crate::proposed_change::ProposedChange;
    use crate::proposal::ProposalComponent;
    use crate::proposal_receipt::ProposalReceipt;
    use crate::utils::get_current_time;

    pub struct Dao {
        dex_router: ComponentAddress,
        stablecoin_issuer: ComponentAddress,
        stablecoin_address: ResourceAddress,
        stablecoin_minter: Vault,
        position_address: ResourceAddress,
        voter_card_address: ResourceAddress,
        voter_card_id: u64,
        proposal_receipt_address: ResourceAddress,
        resource_minter: Vault,
        protocol_admin_badge: Vault,
        proposals: HashMap<u64, ComponentAddress>,
        proposal_id: u64,
        locked_stablecoins: Vault,
        locked_positions: Vault,
        total_voting_power: Decimal,
        vote_period: i64,
        vote_validity_threshold: Decimal,
        reserves: HashMap<ResourceAddress, Vault>
    }

    impl Dao {

        pub fn new(vote_period: i64, vote_validity_threshold: Decimal) -> ComponentAddress {

            // Creates the protocol admin badge which will control everything
            let protocol_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Stoichiometric protocol admin badge")
                .burnable(rule!(allow_all), AccessRule::DenyAll)
                .mint_initial_supply(Decimal::ONE);

            // Creates the stablecoin minter
            let mut stablecoin_minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mintable(rule!(require(protocol_admin_badge.resource_address())), AccessRule::DenyAll)
                .burnable(rule!(require(protocol_admin_badge.resource_address())), AccessRule::DenyAll)
                .recallable(rule!(require(protocol_admin_badge.resource_address())), AccessRule::DenyAll)
                .mint_initial_supply(2);

            // Creates the stablecoin resource
            let stablecoin_address = ResourceBuilder::new_fungible()
                .divisibility(18)
                .mintable(rule!(require(stablecoin_minter.resource_address())), AccessRule::DenyAll)
                .burnable(rule!(require(stablecoin_minter.resource_address())), AccessRule::DenyAll)
                .updateable_metadata(rule!(require(stablecoin_minter.resource_address())), AccessRule::DenyAll)
                .metadata("name", "Stoichiometric USD")
                .metadata("symbol", "SUSD")
                .metadata("icon", "https://cdn-icons-png.flaticon.com/512/3215/3215346.png")
                .create_with_no_initial_supply();

            // Creates the VoterCard and ProposalReceipt minter
            let resource_minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mintable(rule!(require(protocol_admin_badge.resource_address())), AccessRule::DenyAll)
                .burnable(rule!(require(protocol_admin_badge.resource_address())), AccessRule::DenyAll)
                .recallable(rule!(require(protocol_admin_badge.resource_address())), AccessRule::DenyAll)
                .mint_initial_supply(Decimal::ONE);

            let voter_card_address = ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "Stoichiometric voter card")
                .mintable(
                    rule!(require(resource_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .burnable(
                    rule!(require(resource_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .updateable_non_fungible_data(
                    rule!(require(resource_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .create_with_no_initial_supply();

            let proposal_receipt_address = ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "Stoichiometric DAO proposal receipt")
                .mintable(rule!(require(protocol_admin_badge.resource_address())), AccessRule::DenyAll)
                .burnable(rule!(require(protocol_admin_badge.resource_address())), AccessRule::DenyAll)
                .create_with_no_initial_supply();

            let (dex_router, position_address) = RouterComponent::new(protocol_admin_badge.resource_address(), stablecoin_address);
            let stablecoin_issuer = IssuerComponent::new(protocol_admin_badge.resource_address(), stablecoin_minter.take(1),stablecoin_address);

            let dao_rules = AccessRules::new()
                .default(
                    rule!(allow_all),
                    AccessRule::DenyAll,
                );

            let mut component = Self{
                dex_router,
                stablecoin_issuer,
                stablecoin_address: stablecoin_address.clone(),
                stablecoin_minter : Vault::with_bucket(stablecoin_minter),
                position_address: position_address.clone(),
                voter_card_address,
                voter_card_id: 0,
                proposal_receipt_address,
                resource_minter : Vault::with_bucket(resource_minter),
                protocol_admin_badge: Vault::with_bucket(protocol_admin_badge),
                proposals: HashMap::new(),
                proposal_id: 0,
                locked_stablecoins: Vault::new(stablecoin_address),
                locked_positions: Vault::new(position_address),
                total_voting_power: Decimal::ZERO,
                vote_period,
                vote_validity_threshold,
                reserves: HashMap::new()
            }
                .instantiate();

            component.add_access_check(dao_rules);

            component.globalize()
        }

        pub fn lock(&mut self, stablecoins: Bucket, positions: Bucket, opt_voter_card_proof: Option<Proof>) -> Option<Bucket>{
            assert!(stablecoins.resource_address() == self.stablecoin_address, "You can only lock stablecoins as fungible resource");
            assert!(positions.resource_address() == self.position_address, "You can only lock positions as non fungible resource");

            let (mut voter_card, voter_card_id, opt_bucket) = match opt_voter_card_proof {
                Some(voter_card_proof) => {
                    let validated_proof = voter_card_proof.validate_proof(ProofValidationMode::ValidateContainsAmount(self.voter_card_address, Decimal::ONE)).expect("Please provide a valid voter card proof");
                    let voter_card_id = validated_proof.non_fungible::<VoterCard>().local_id().clone();
                    let voter_card_data = self.get_voter_card_data(&voter_card_id);

                    (voter_card_data, voter_card_id, None)
                }

                None => {
                    let voter_card_id = NonFungibleLocalId::Integer(self.voter_card_id.into());
                    let new_voter_card = self.resource_minter.authorize(|| {
                        borrow_resource_manager!(self.voter_card_address).mint_non_fungible(
                            &voter_card_id,
                            VoterCard::new()
                        )
                    });
                    self.voter_card_id+=1;

                    (VoterCard::new(), voter_card_id, Some(new_voter_card))
                }
            };

            self.total_voting_power += voter_card.add_stablecoins(stablecoins.amount());

            for position in positions.non_fungibles::<Position>() {
                let id = position.local_id().clone();
                let position_data = self.get_position_data(&id);

                self.total_voting_power += voter_card.add_position(&position_data, id);
            }

            self.update_voter_card_data(&voter_card_id, voter_card);

            opt_bucket
        }

        pub fn unlock(&mut self, voter_card: Bucket) -> (Bucket, Bucket) {
            assert!(voter_card.resource_address() == self.voter_card_address, "Please provide bucket with voter cards inside");
            assert!(voter_card.amount() == Decimal::ONE, "Please provide only one voter_card");

            let voter_card_nfr = voter_card.non_fungible::<VoterCard>();
            let voter_card_data = self.get_voter_card_data(voter_card_nfr.local_id());
            let last_proposal_voted = self.get_proposal(voter_card_data.last_proposal_voted_id);

            assert!(!last_proposal_voted.is_voting_stage(), "Cannot unlock tokens and positions from a VoterCard that is actively particpating in a vote!");

            self.total_voting_power -= voter_card_data.voting_power;
            let stablecoin_bucket = self.locked_stablecoins.take(voter_card_data.stablecoins_locked);
            let mut positions_bucket = Bucket::new(self.position_address);
            for position_id in voter_card_data.positions_locked_ids {
                positions_bucket.put(
                    self.locked_positions.take_non_fungible(&position_id)
                );
            }

            self.resource_minter.authorize(|| {
                borrow_resource_manager!(self.position_address).burn(voter_card);
            });

            (stablecoin_bucket, positions_bucket)
        }

        pub fn make_proposal(&mut self, proposed_change: ProposedChange) -> Bucket {
            let current_time = get_current_time();
            let vote_end = current_time + self.vote_period;
            let vote_threshold = self.total_voting_power * self.vote_validity_threshold;

            let voter_card_updater = self.protocol_admin_badge.authorize(|| {
                borrow_resource_manager!(self.resource_minter.resource_address()).mint(Decimal::ONE)
            });

            let proposal_comp = ProposalComponent::new(self.proposal_id, vote_end, vote_threshold, proposed_change, self.voter_card_address, voter_card_updater, self.protocol_admin_badge.resource_address());
            self.proposals.insert(self.proposal_id, proposal_comp);

            let receipt_data = ProposalReceipt{ proposal_id: self.proposal_id };
            let receipt = self.resource_minter.authorize(|| {
                borrow_resource_manager!(self.proposal_receipt_address).mint_non_fungible(&NonFungibleLocalId::Integer(self.proposal_id.into()), receipt_data)
            });

            self.proposal_id += 1;

            receipt
        }

        pub fn execute_proposal(&mut self, proposal_receipt: Bucket) -> Option<Vec<Bucket>> {
            assert!(proposal_receipt.resource_address() == self.proposal_receipt_address, "Please provide a proposal receipt");
            assert!(proposal_receipt.amount() == Decimal::ONE, "Can only execute one proposal at a time");

            let proposal_data: ProposalReceipt = borrow_resource_manager!(self.proposal_receipt_address).get_non_fungible_data(proposal_receipt.non_fungible::<ProposalReceipt>().local_id());
            let mut proposal = self.get_proposal(proposal_data.proposal_id);

            let changes_to_execute = self.protocol_admin_badge.authorize(|| {
                proposal.execute()
            });

            self.resource_minter.authorize(|| {
                borrow_resource_manager!(self.proposal_receipt_address).burn(proposal_receipt)
            });

            match changes_to_execute {
                None => None,
                Some(changes) => self.execute_proposed_change(changes)
            }


        }

        fn execute_proposed_change(&mut self, proposed_change: ProposedChange) -> Option<Vec<Bucket>> {

            match proposed_change
            {
                ProposedChange::ChangeVotePeriod(new_period) =>
                    {
                        self.vote_period = new_period;
                        None
                    }

                ProposedChange::ChangeMinimumVoteThreshold(new_threshold) =>
                    {
                        self.vote_validity_threshold = new_threshold;
                        None
                    }

                ProposedChange::GrantIssuingRight =>
                    {
                        let new_stablecoin_minter = self.protocol_admin_badge.authorize(||
                            {
                                borrow_resource_manager!(self.stablecoin_minter.resource_address()).mint(Decimal::ONE)
                            });
                        Some(vec![new_stablecoin_minter])
                    }

                ProposedChange::RemoveIssuingRight(_vault_bytes) =>
                    {
                        /* Not doable yet, but the recalling and burning the minter should be done like that:
                            self.protocol_admin_badge.authorize(|| {
                                let resource_manager = borrow_resource_manager!(self.stablecoin_minter.resource_address());
                                let minter = resource_manager.recall(vault_bytes);
                                resource_manager.burn(minter);

                            });
                            self.protocol_admin_badge.authorize(|| {
                                borrow_resource_manager!(self.stablecoin_minter.resource_address(
                            });
                         */
                        None
                    }

                ProposedChange::AllowClaim(claimed_resources) => {

                    let mut vec_bucket = vec![];
                    for (token, amount) in &claimed_resources {
                        let bucket = self.reserves.get_mut(token).expect("There are no reserves for some of the tokens").take(*amount);
                        vec_bucket.push(bucket)
                    }
                    Some(vec_bucket)
                }

                ProposedChange::AddNewCollateralToken(new_token, loan_to_value, interest_rate, liquidation_threshold, liquidation_penalty, initial_rate, minimum_rate, maximum_rate, oracle) =>
                    {
                        let mut router = RouterLocalComponent::at(self.dex_router);
                        let mut issuer = IssuerLocalComponent::at(self.stablecoin_issuer);

                        self.protocol_admin_badge.authorize(|| {
                            router.create_pool(new_token.clone(), initial_rate, minimum_rate, maximum_rate);
                            issuer.new_lender(new_token, loan_to_value, interest_rate, liquidation_threshold, liquidation_penalty, oracle);
                        });

                        None
                    }

                ProposedChange::ChangeLenderParameters(lender, loan_to_value, interest_rate, liquidation_threshold, liquidation_penalty) =>
                    {
                        let mut issuer = IssuerLocalComponent::at(self.stablecoin_issuer);

                        self.protocol_admin_badge.authorize(|| {
                            issuer.change_lender_parameters(lender, loan_to_value, interest_rate, liquidation_threshold, liquidation_penalty);
                        });

                        None
                    }

                ProposedChange::ChangeLenderOracle(lender, oracle_address) =>
                    {
                        let mut issuer = IssuerLocalComponent::at(self.stablecoin_issuer);

                        self.protocol_admin_badge.authorize(|| {
                            issuer.change_lender_oracle(lender, oracle_address);
                        });

                        None
                    }

                ProposedChange::AddTokensToIssuerReserves(tokens_to_transfer) =>
                    {
                        let mut issuer = IssuerLocalComponent::at(self.stablecoin_issuer);
                        let mut buckets_to_give = vec![];
                        for (token, amount) in &tokens_to_transfer {
                            let bucket = self.reserves.get_mut(token).expect("There are no reserves for some of the tokens").take(*amount);
                            buckets_to_give.push(bucket);
                        }
                        issuer.give_tokens(buckets_to_give);

                        None
                    }
            }
        }

        fn get_proposal(&self, proposal_id: u64) -> ProposalLocalComponent {
            match self.proposals.get(&proposal_id) {
                None => { panic!("Proposal {} does not exist", proposal_id) }
                Some(proposal_address) => { ProposalLocalComponent::at(*proposal_address) }
            }
        }

        #[inline]
        fn get_voter_card_data(&self, id: &NonFungibleLocalId) -> VoterCard {
            borrow_resource_manager!(self.voter_card_address).get_non_fungible_data(id)
        }

        #[inline]
        fn get_position_data(&self, id: &NonFungibleLocalId) -> Position {
            borrow_resource_manager!(self.position_address).get_non_fungible_data(id)
        }

        #[inline]
        fn update_voter_card_data(&self, id: &NonFungibleLocalId, data: VoterCard) {
            self.resource_minter.authorize(|| {
                borrow_resource_manager!(self.voter_card_address).update_non_fungible_data(id, data);
            });
        }
    }

}