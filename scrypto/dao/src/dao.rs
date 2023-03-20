use scrypto::{blueprint, external_component};

external_component! {
    ProposalLocalComponent {
        fn vote_for(&mut self, voter_card_proof: Proof);
        fn vote_against(&mut self, voter_card_proof: Proof);
        fn is_voting_stage(&self) -> bool;
        fn is_accepted(&self) -> bool;
    }
}


#[blueprint]
mod dao {
    use stoichiometric_dex::router::RouterComponent;
    use stoichiometric_dex::position::Position;
    use stoichiometric_stablecoin::issuer::IssuerComponent;
    use crate::voter_card::VoterCard;

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
        locked_positions: Vault
    }

    impl Dao {

        pub fn new() -> ComponentAddress {

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
                .metadata("name", "Stoichiometric proposal receipt")
                .updateable_metadata(rule!(require(resource_minter.resource_address())), AccessRule::DenyAll)
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
                locked_positions: Vault::new(position_address)

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

            voter_card.add_stablecoins(stablecoins.amount());

            for position in positions.non_fungibles::<Position>() {
                let id = position.local_id().clone();
                let position_data = self.get_position_data(&id);

                voter_card.add_position(&position_data, id);
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