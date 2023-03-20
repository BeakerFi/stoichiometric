
use scrypto::blueprint;

#[blueprint]
mod issuer {
    use crate::constants::FLASH_LOAN_FEE;
    use crate::flash_mint::FlashMint;
    use crate::lender::LenderComponent;
    use crate::loan::Loan;

    pub struct Issuer {
        reserves: HashMap<ResourceAddress, Vault>,
        lenders: HashMap<ResourceAddress, LenderComponent>,
        stablecoin_minter: Vault,
        resource_minter: Vault,
        stablecoin_address: ResourceAddress,
        loan_address: ResourceAddress,
        loan_id: u64,
        flash_mint_address: ResourceAddress,
        flash_mint_id: u64,
        admin_badge: ResourceAddress
    }

    impl Issuer {

        pub fn new(admin_badge: ResourceAddress, stablecoin_minter: Bucket, stablecoin_address: ResourceAddress) -> ComponentAddress {

            // Creates the resource minter
            let resource_minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(Decimal::ONE);

            // Creates the NFR Position address
            let loan_address = ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "Stoichiometric Loan")
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

            // Creates the NFR FlashMint address
            let flash_mint_address = ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "Stoichiometric FlashMint")
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
                .restrict_deposit(rule!(deny_all), AccessRule::DenyAll)
                .create_with_no_initial_supply();

            let issuer_rules = AccessRules::new()
                .method("take_loan", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("repay_loans", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("add_collateral", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("remove_collateral", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("liquidate", AccessRule::AllowAll, AccessRule::DenyAll)
                //.method("liquidate_list", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("flash_mint", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("repay_flash_mint", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("get_lender_state", AccessRule::AllowAll, AccessRule::DenyAll)
                .default(
                    rule!(require(admin_badge)),
                    AccessRule::DenyAll
                );


            let mut component = Self{
                reserves: HashMap::new(),
                lenders: HashMap::new(),
                stablecoin_minter: Vault::with_bucket(stablecoin_minter),
                resource_minter: Vault::with_bucket(resource_minter),
                stablecoin_address,
                loan_address,
                loan_id: 0,
                flash_mint_address,
                flash_mint_id: 0,
                admin_badge
            }
                .instantiate();

            component.add_access_check(issuer_rules);

            component.globalize()
        }

        pub fn new_lender(&mut self, collateral_address: ResourceAddress, loan_to_value: Decimal, interest_rate: Decimal, liquidation_threshold: Decimal, liquidation_incentive: Decimal, oracle: ComponentAddress) {
            assert!(self.lenders.get(&collateral_address).is_none(), "There is already a lender for the given token");
            assert!(loan_to_value.is_positive() && loan_to_value<Decimal::ONE, "LTV should be such that 0<LTV<1");
            assert!(interest_rate.is_positive() && interest_rate<Decimal::ONE, "The daily interest rate should be such that 0<DIR<1");
            assert!(liquidation_threshold > Decimal::ONE, "The liquidation threshold should be greater than one");
            assert!(liquidation_incentive.is_positive(), "The liquidation incentive should be positive");

            let new_lender = LenderComponent::new(collateral_address.clone(), loan_to_value, interest_rate, liquidation_threshold, liquidation_incentive, oracle);

            self.lenders.insert(collateral_address.clone(), new_lender);
        }

        pub fn take_loan(&mut self, collateral: Bucket, amount_to_loan: Decimal) ->(Bucket, Bucket) {

            let lender = self.get_lender(&collateral.resource_address());
            let loan = lender.take_loan(collateral, amount_to_loan);
            let loan_bucket = self.resource_minter.authorize(|| {
                    borrow_resource_manager!(self.loan_address).mint_non_fungible(
                        &NonFungibleLocalId::Integer(self.loan_id.into()),
                        loan
                    )
            });

            self.loan_id += 1;

            let stablecoin_bucket = self.mint(amount_to_loan);
            (stablecoin_bucket, loan_bucket)
        }

        pub fn repay_loans(&mut self, mut repayment: Bucket, loans: Bucket) -> (Bucket, Vec<Bucket>) {
            assert!(loans.resource_address() == self.loan_address, "Please provide loans to repay");
            assert!(repayment.resource_address() == self.stablecoin_address, "Repayment should be provided in stablecoins tokens");

            let mut buckets: Vec<Bucket> = Vec::new();
            let mut stablecoins_to_burn = Bucket::new(self.stablecoin_address);
            for loan_nfr in loans.non_fungibles::<Loan>() {

                let loan = self.get_loan_data(&loan_nfr);
                let lender = self.lenders.get(&loan.collateral_token).unwrap();
                let amount_lent = loan.amount_lent;

                let (interests, collateral) = lender.repay_loan(repayment.amount(), loan);

                self.put_in_reserves(repayment.take(interests));
                stablecoins_to_burn.put(repayment.take(amount_lent));
                buckets.push(collateral);
            }

            self.burn_bucket(stablecoins_to_burn);
            self.resource_minter.authorize(|| borrow_resource_manager!(self.loan_address).burn(loans));

            (repayment, buckets)
        }

        pub fn add_collateral(&mut self, collateral: Bucket, loan_proof: Proof) {

            let valid_proof = self.check_single_loan_proof(loan_proof);

            let loan_nfr = valid_proof.non_fungible::<Loan>();
            let loan = self.get_loan_data(&loan_nfr);

            assert!(loan.collateral_token == collateral.resource_address(), "Please provide the right tokens to add as collateral");
            let lender = self.get_lender(&loan.collateral_token);

            let new_loan_data = lender.add_collateral(collateral, loan);
            self.update_loan_data(loan_nfr, new_loan_data);
        }

        pub fn remove_collateral(&mut self, amount: Decimal,  loan_proof: Proof) -> Bucket {

            let valid_proof = self.check_single_loan_proof(loan_proof);

            let loan_nfr = valid_proof.non_fungible::<Loan>();
            let loan = self.get_loan_data(&loan_nfr);

            let lender = self.get_lender(&loan.collateral_token);

            let (new_loan_data, collateral) = lender.remove_collateral(amount, loan);
            self.update_loan_data(loan_nfr, new_loan_data);

            collateral
        }

        pub fn liquidate(&mut self, mut repayment: Bucket, non_fungible_id: NonFungibleLocalId) -> (Bucket, Bucket) {

            let loan: Loan = borrow_resource_manager!(self.loan_address).get_non_fungible_data(&non_fungible_id);

            let lender = self.get_lender(&loan.collateral_token);

            let (amount_to_burn, liquidator_bucket, reserve_bucket, new_loan_data) = lender.liquidate(repayment.amount(), loan);

            let bucket_to_burn = repayment.take(amount_to_burn);
            self.burn_bucket(bucket_to_burn);

            self.put_in_reserves(reserve_bucket);

            self.resource_minter.authorize(|| {
                borrow_resource_manager!(self.loan_address).update_non_fungible_data(&non_fungible_id, new_loan_data);
            });

            (repayment, liquidator_bucket)
        }
        /*
        pub fn liquidate_list(&mut self, mut repayment: Bucket, non_fungible_ids: Vec<NonFungibleLocalId>) -> (Bucket, Vec<Bucket>) {

            let mut buckets: Vec<Bucket> = Vec::new();
            let mut bucket_to_burn = Bucket::new(self.stablecoin_address);

            for non_fungible_id in non_fungible_ids {
                let loan: Loan = borrow_resource_manager!(self.loan_address).get_non_fungible_data(&non_fungible_id);
                let lender = self.get_lender(&loan.collateral_token);

                let (interests, amount_lent, liquidator_bucket, reserve_bucket, new_loan_data) = lender.liquidate(repayment.amount(), loan);

                bucket_to_burn.put(repayment.take(amount_lent));
                self.put_in_reserves(repayment.take(interests));
                self.put_in_reserves(reserve_bucket);
                buckets.push(liquidator_bucket);

                borrow_resource_manager!(self.loan_address).update_non_fungible_data(&non_fungible_id, new_loan_data);
            }

            self.burn_bucket(bucket_to_burn);

            (repayment, buckets)
        }
        */

        pub fn flash_mint(&mut self, amount_to_mint: Decimal) -> (Bucket, Bucket) {
            let stablecoin_amount = self.mint(amount_to_mint);

            let flash_loan = self.resource_minter.authorize(|| {
                borrow_resource_manager!(self.flash_mint_address).mint_non_fungible(
                    &NonFungibleLocalId::Integer(self.flash_mint_id.into()),
                    FlashMint::new(amount_to_mint)
                )
            });

            self.flash_mint_id += 1;

            (stablecoin_amount, flash_loan)
        }

        pub fn repay_flash_mint(&mut self, mut repayment: Bucket, flash_mint_bucket: Bucket) -> Bucket {
            assert!(flash_mint_bucket.resource_address() == self.flash_mint_address, "Please provide a flash mint to repay");
            assert!(repayment.resource_address() == self.stablecoin_address, "Please provide repayment in stablecoins");

            let flash_mint_nfr = flash_mint_bucket.non_fungible::<FlashMint>();
            let flash_mint_data: FlashMint = borrow_resource_manager!(self.flash_mint_address).get_non_fungible_data(flash_mint_nfr.local_id());

            let amount_due = flash_mint_data.amount_minted*FLASH_LOAN_FEE;
            assert!(repayment.amount() >= amount_due, "You did not provide enough stablecoins to repay the flash loan");

            self.resource_minter.authorize(|| {
                borrow_resource_manager!(self.flash_mint_address).burn(flash_mint_bucket);
            });

            self.burn_bucket(repayment.take(amount_due));

            repayment
        }

        pub fn change_lender_parameters(&mut self, lender_collateral: ResourceAddress, loan_to_value: Decimal, interest_rate: Decimal, liquidation_threshold: Decimal, liquidation_incentive: Decimal) {
            let lender = self.get_lender(&lender_collateral);
            lender.change_parameters(loan_to_value, interest_rate, liquidation_threshold, liquidation_incentive);
        }

        pub fn change_oracle(&mut self, lender_collateral: ResourceAddress, oracle: ComponentAddress)
        {
            let lender = self.get_lender(&lender_collateral);
            lender.change_oracle(oracle);
        }

        pub fn get_lender_state(&self, collateral_token: ResourceAddress) -> Vec<Decimal> {
            let lender = self.get_lender(&collateral_token);
            lender.get_state()
        }

        #[inline]
        fn mint(&mut self, amount: Decimal) -> Bucket {
            self.stablecoin_minter.authorize(|| {
                borrow_resource_manager!(self.stablecoin_address).mint(amount)
            })
        }

        #[inline]
        fn put_in_reserves(&mut self, bucket: Bucket) {
            match self.reserves.get_mut(&bucket.resource_address())
            {
                Some(vault) => { vault.put(bucket) }
                None => {
                    let new_vault = Vault::with_bucket(bucket);
                    self.reserves.insert(new_vault.resource_address(), new_vault);
                }
            };
        }

        #[inline]
        fn burn_bucket(&self, bucket: Bucket) {
            self.stablecoin_minter.authorize(|| {
                borrow_resource_manager!(self.stablecoin_address).burn(bucket);
            });
        }

        #[inline]
        fn check_single_loan_proof(&self, loan_proof: Proof) -> ValidatedProof {
            loan_proof.validate_proof(ProofValidationMode::ValidateContainsAmount(
                self.loan_address,
                Decimal::ONE
            )).expect("Please provide a valid proof of a single loan")
        }


        #[inline]
        fn get_loan_data(&self, loan_nfr: &NonFungible<Loan>) -> Loan {
            borrow_resource_manager!(self.loan_address)
                .get_non_fungible_data(loan_nfr.local_id())
        }

        #[inline]
        fn update_loan_data(&self, loan_nfr: NonFungible<Loan>, new_data: Loan) {
            self.resource_minter.authorize(|| loan_nfr.update_data(new_data));
        }

        #[inline]
        fn get_lender(&self, resource_address: &ResourceAddress) -> &LenderComponent
        {
            match self.lenders.get(resource_address){
                None => { panic!("There is no lenders for this token") },
                Some(lender) => { lender }
            }
        }
    }
}