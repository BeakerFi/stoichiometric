use scrypto::blueprint;

#[blueprint]
mod router {

    use crate::pool::PoolComponent;
    use crate::position::Position;

    pub struct Router{
        stablecoin_address: ResourceAddress,
        pools: HashMap<ResourceAddress, PoolComponent>,
        position_minter: Vault,
        position_address: ResourceAddress,
        position_id: u64,
        admin_badge: ResourceAddress
    }

    impl Router {

        pub fn new(stablecoin_address: ResourceAddress) -> (ComponentAddress, Bucket) {
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Router admin badge")
                .burnable(rule!(allow_all), AccessRule::DenyAll)
                .mint_initial_supply(Decimal::ONE);

            // Creates the position minter
            let position_minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(Decimal::ONE);

            // Creates the NFR Position address
            let position_resource = ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "Stoichiometric Position")
                .mintable(
                    rule!(require(position_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .burnable(
                    rule!(require(position_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .updateable_non_fungible_data(
                    rule!(require(position_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .create_with_no_initial_supply();

            let router_rules = AccessRules::new()
                .method("add_liquidity", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("add_liquidity_at_step", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("add_liquidity_at_steps", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("remove_liquidity_at_rate", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("remove_liquidity_at_step", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("remove_liquidity_at_steps", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("remove_all_liquidity", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("claim_fees", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("swap", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("get_pool_state", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("step_at_rate", AccessRule::AllowAll, AccessRule::DenyAll)
                .default(
                    rule!(require(admin_badge.resource_address())),
                    AccessRule::DenyAll
                );

            let mut component = Self {
                stablecoin_address,
                pools: HashMap::new(),
                position_minter: Vault::with_bucket(position_minter),
                position_address: position_resource,
                position_id: 0,
                admin_badge: admin_badge.resource_address()
            }
                .instantiate();

            component.add_access_check(router_rules);
            let component = component.globalize();
            (component, admin_badge)
        }

        pub fn create_pool(&mut self, bucket_a: Bucket, bucket_b: Bucket, initial_rate: Decimal, min_rate: Decimal, max_rate: Decimal) ->(Bucket, Bucket, Bucket)
        {
            assert!(bucket_a.resource_address() != bucket_b.resource_address(), "Two pools cannot trade the same token");
            assert!(bucket_a.resource_address() == self.stablecoin_address || bucket_b.resource_address() == self.stablecoin_address, "Every pool should be Stablecoin/Other");

            let (bucket_stable, bucket_other, rate_init, rate_min, rate_max) = if bucket_a.resource_address() == self.stablecoin_address {
                (bucket_a, bucket_b, initial_rate, min_rate, max_rate)
            } else {
                (bucket_b, bucket_a, Decimal::ONE / initial_rate, Decimal::ONE/ max_rate, Decimal::ONE / min_rate)
            };

            assert!(self.pools.get(&bucket_other.resource_address()).is_none(), "A pool trading these tokens already exists");

            let (pool, ret_stable, ret_other, position) = PoolComponent::new(bucket_stable, bucket_other, rate_init, rate_min, rate_max);
            self.pools.insert(ret_other.resource_address(), pool);
            let ret_pos = self.position_minter.authorize(|| {
                borrow_resource_manager!(self.position_address).mint_non_fungible(
                    &NonFungibleLocalId::Integer(self.position_id.into()),
                    position
                )
            });
            self.position_id+=1;

            (ret_stable, ret_other, ret_pos)
        }

        pub fn add_liquidity(&mut self, bucket_a: Bucket, bucket_b: Bucket, rate: Decimal, opt_position_proof: Option<Proof>) -> (Bucket, Bucket, Option<Bucket>)
        {
            let (bucket_stable, bucket_other) = self.sort_buckets(bucket_a, bucket_b);
            let pool = self.get_pool(bucket_other.resource_address());

            match opt_position_proof {
                Some(position_proof) => {
                    let valid_proof = self.check_single_position_proof(position_proof);
                    let position_nfr = valid_proof.non_fungible::<Position>();
                    let data = self.get_position_data(&position_nfr);

                    let (ret_stable, ret_other, new_data) = pool.add_liquidity(bucket_stable, bucket_other, rate, data);
                    self.update_position(position_nfr, new_data);
                    (ret_stable, ret_other, None)
                }
                None => {
                    let empty_pos = Position::from(bucket_other.resource_address());
                    let (ret_stable, ret_other, new_data) = pool.add_liquidity(bucket_stable, bucket_other, rate, empty_pos);

                    let bucket_pos = self.position_minter.authorize(|| {
                        borrow_resource_manager!(self.position_address).mint_non_fungible(
                            &NonFungibleLocalId::Integer(self.position_id.into()),
                            new_data
                        )
                    });
                    self.position_id+=1;
                    (ret_stable, ret_other, Some(bucket_pos))
                }
            }
        }

        pub fn add_liquidity_between_rates(&mut self, bucket_a: Bucket, bucket_b: Bucket, min_rate: Decimal, max_rate: Decimal, opt_position_proof: Option<Proof>) -> (Bucket, Bucket, Option<Bucket>) {
            let (bucket_stable, bucket_other) = self.sort_buckets(bucket_a, bucket_b);
            let pool = self.get_pool(bucket_other.resource_address());

            match opt_position_proof {
                Some(position_proof) => {
                    let valid_proof = self.check_single_position_proof(position_proof);
                    let position_nfr = valid_proof.non_fungible::<Position>();
                    let data = self.get_position_data(&position_nfr);

                    let (ret_stable, ret_other, new_data) = pool.add_liquidity_between_rates(bucket_stable, bucket_other, data, min_rate, max_rate);
                    self.update_position(position_nfr, new_data);
                    (ret_stable, ret_other, None)
                }
                None => {
                    let empty_pos = Position::from(bucket_other.resource_address());
                    let (ret_stable, ret_other, new_data) = pool.add_liquidity_between_rates(bucket_stable, bucket_other, empty_pos, min_rate, max_rate);

                    let bucket_pos = self.position_minter.authorize(|| {
                        borrow_resource_manager!(self.position_address).mint_non_fungible(
                            &NonFungibleLocalId::Integer(self.position_id.into()),
                            new_data
                        )
                    });
                    self.position_id+=1;
                    (ret_stable, ret_other, Some(bucket_pos))
                }
            }
        }

        pub fn add_liquidity_at_step(&mut self, bucket_a: Bucket, bucket_b: Bucket, step_id: u16, opt_position_proof: Option<Proof>) -> (Bucket, Bucket, Option<Bucket>)
        {
            let (bucket_stable, bucket_other) = self.sort_buckets(bucket_a, bucket_b);
            let pool = self.get_pool(bucket_other.resource_address());

            match opt_position_proof {
                Some(position_proof) => {
                    let valid_proof = self.check_single_position_proof(position_proof);
                    let position_nfr = valid_proof.non_fungible::<Position>();
                    let data = self.get_position_data(&position_nfr);

                    let (ret_stable, ret_other, new_data) = pool.add_liquidity_at_step(bucket_stable, bucket_other, data, step_id);
                    self.update_position(position_nfr, new_data);
                    (ret_stable, ret_other, None)
                }
                None => {
                    let empty_pos = Position::from(bucket_other.resource_address());
                    let (ret_stable, ret_other, new_data) = pool.add_liquidity_at_step(bucket_stable, bucket_other, empty_pos, step_id);

                    let bucket_pos = self.position_minter.authorize(|| {
                        borrow_resource_manager!(self.position_address).mint_non_fungible(
                            &NonFungibleLocalId::Integer(self.position_id.into()),
                            new_data
                        )
                    });
                    self.position_id+=1;
                    (ret_stable, ret_other, Some(bucket_pos))
                }
            }
        }

        pub fn add_liquidity_at_steps(&mut self, bucket_a: Bucket, bucket_b: Bucket, start_step: u16, stop_step: u16, opt_position_proof: Option<Proof>) -> (Bucket, Bucket, Option<Bucket>)
        {
            let (bucket_stable, bucket_other) = self.sort_buckets(bucket_a, bucket_b);
            let pool = self.get_pool(bucket_other.resource_address());

            match opt_position_proof {
                Some(position_proof) => {
                    let valid_proof = self.check_single_position_proof(position_proof);
                    let position_nfr = valid_proof.non_fungible::<Position>();
                    let data = self.get_position_data(&position_nfr);

                    let (ret_stable, ret_other, new_data) = pool.add_liquidity_at_steps(bucket_stable, bucket_other, data, start_step, stop_step);
                    self.update_position(position_nfr, new_data);
                    (ret_stable, ret_other, None)
                }
                None => {
                    let empty_pos = Position::from(bucket_other.resource_address());
                    let (ret_stable, ret_other, new_data) = pool.add_liquidity_at_steps(bucket_stable, bucket_other, empty_pos, start_step, stop_step);

                    let bucket_pos = self.position_minter.authorize(|| {
                        borrow_resource_manager!(self.position_address).mint_non_fungible(
                            &NonFungibleLocalId::Integer(self.position_id.into()),
                            new_data
                        )
                    });
                    self.position_id+=1;
                    (ret_stable, ret_other, Some(bucket_pos))
                }
            }
        }

        pub fn remove_liquidity_at_step(&mut self, position_proof: Proof, step: u16) -> (Bucket, Bucket)
        {
            let valid_proof = self.check_single_position_proof(position_proof);
            let position_nfr = valid_proof.non_fungible::<Position>();
            let data = self.get_position_data(&position_nfr);

            let pool = self.get_pool(data.token);
            let (ret_stable, ret_other, new_data) = pool.remove_liquidity_at_step(data, step);
            self.update_position(position_nfr, new_data);
            (ret_stable, ret_other)
        }

        pub fn remove_liquidity_at_steps(&mut self, position_proof: Proof, start_step: u16, stop_step: u16) -> (Bucket, Bucket)
        {
            let valid_proof = self.check_single_position_proof(position_proof);
            let position_nfr = valid_proof.non_fungible::<Position>();
            let data = self.get_position_data(&position_nfr);

            let pool = self.get_pool(data.token);
            let (ret_stable, ret_other, new_data) = pool.remove_liquidity_at_steps(data, start_step, stop_step);
            self.update_position(position_nfr, new_data);
            (ret_stable, ret_other)
        }

        pub fn remove_liquidity_at_rate(&mut self, position_proof: Proof, rate: Decimal) -> (Bucket, Bucket)
        {
            let valid_proof = self.check_single_position_proof(position_proof);
            let position_nfr = valid_proof.non_fungible::<Position>();
            let data = self.get_position_data(&position_nfr);

            let pool = self.get_pool(data.token);
            let (ret_stable, ret_other, new_data) = pool.remove_liquidity_at_rate(data, rate);
            self.update_position(position_nfr, new_data);
            (ret_stable, ret_other)
        }

        pub fn remove_all_liquidity(&mut self, positions_bucket: Bucket) -> Vec<Bucket> {
            assert!(positions_bucket.resource_address() == self.position_address);

            let mut buckets: Vec<Bucket> = Vec::new();
            let mut stable_bucket = Bucket::new(self.stablecoin_address);
            for position_nfr in positions_bucket.non_fungibles::<Position>() {

                let data = self.get_position_data(&position_nfr);
                let pool = self.get_pool(data.token);
                let (ret_stable, ret_other) = pool.remove_all_liquidity(data);

                stable_bucket.put(ret_stable);
                buckets.push(ret_other);
            }
            buckets.push(stable_bucket);
            self.position_minter.authorize(|| { positions_bucket });
            buckets
        }

        pub fn claim_fees(&mut self, positions_proof: Proof) -> Vec<Bucket> {
            let valid_proof = self.check_multiple_position_proof(positions_proof);

            let mut buckets: Vec<Bucket> = Vec::new();
            let mut stable_bucket = Bucket::new(self.stablecoin_address);
            for position_nfr in valid_proof.non_fungibles::<Position>() {

                let data = self.get_position_data(&position_nfr);
                let pool = self.get_pool(data.token);
                let (ret_stable, ret_other, new_data) = pool.claim_fees(data);
                self.update_position(position_nfr, new_data);

                stable_bucket.put(ret_stable);
                buckets.push(ret_other);
            }
            buckets.push(stable_bucket);
            buckets
        }

        pub fn claim_protocol_fees(&mut self) -> Vec<Bucket> {
            let mut buckets: Vec<Bucket> = Vec::new();
            let mut stable_bucket = Bucket::new(self.stablecoin_address);

            for (_, pool) in &self.pools {
                let (stable_tmp, other_bucket) = pool.claim_protocol_fees();
                buckets.push(other_bucket);
                stable_bucket.put(stable_tmp);
            }
            buckets.push(stable_bucket);
            buckets
        }

        pub fn swap(&mut self, input: Bucket, output: ResourceAddress) -> (Bucket, Bucket) {
            let pool;
            if output == self.stablecoin_address
            {
                pool = self.get_pool(input.resource_address())
            }
            else
            {
                pool = self.get_pool(output)
            }
            pool.swap(input)
        }

        pub fn get_pool_state(&mut self, token: ResourceAddress) -> (
        Decimal,
        u16,
        Decimal,
        (Decimal, Decimal),
        Vec<(u16, Vec<Decimal>)>,
        ) {
            let pool = self.get_pool(token);
            pool.get_state()
        }

        fn get_pool(&self, token: ResourceAddress) -> &PoolComponent {
            match self.pools.get(&token) {
                None => { panic!("There is no pool trading this pair") }
                Some(pool) => pool,
            }
        }

        fn check_single_position_proof(&self, position_proof: Proof) -> ValidatedProof {
            position_proof
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.position_address,
                    dec!(1),
                ))
                .expect("The provided proof is invalid")
        }

        fn check_multiple_position_proof(&self, positions_proof: Proof) -> ValidatedProof {
            positions_proof.validate_proof(ProofValidationMode::ValidateResourceAddress(
                    self.position_address,
                ))
                .expect("The provided proof is invalid")
        }

        fn get_position_data(&self, position_nfr: &NonFungible<Position>) -> Position {
            borrow_resource_manager!(self.position_address)
                .get_non_fungible_data::<Position>(position_nfr.local_id())
        }

        fn update_position(&self, position_nfr: NonFungible<Position>, new_data: Position) {
            self.position_minter.authorize(|| position_nfr.update_data(new_data));
        }

        #[inline]
        fn sort_buckets(&self, bucket_a: Bucket, bucket_b: Bucket) -> (Bucket, Bucket) {
            if bucket_a.resource_address() == self.stablecoin_address {
                (bucket_a, bucket_b)
            }
            else
            {
                (bucket_b, bucket_a)
            }
        }

        pub fn step_at_rate(&self, token: ResourceAddress, rate: Decimal) -> u16 {
            let pool = self.get_pool(token);
            pool.step_at_rate(rate)
        }

    }
}