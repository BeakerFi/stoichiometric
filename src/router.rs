use scrypto::blueprint;

#[blueprint]
mod router {

    use crate::pool::PoolComponent;
    use crate::position::Position;
    use crate::utils::{sort, sort_buckets};

    pub struct Router{
        pools: HashMap<(ResourceAddress, ResourceAddress), PoolComponent>,
        position_minter: Vault,
        position_address: ResourceAddress,
        position_id: u64,
        admin_badge: ResourceAddress
    }

    impl Router {

        pub fn new() -> (ComponentAddress, Bucket) {
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
                .method("add_liquidity_between_rates", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("add_liquidity_at_step", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("add_liquidity_at_steps", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("remove_liquidity_at_rate", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("remove_liquidity_between_rates", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("remove_liquidity_at_step", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("remove_liquidity_at_steps", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("remove_all_liquidity", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("claim_fees", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("swap", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("get_pool_state", AccessRule::AllowAll, AccessRule::DenyAll)
                .default(
                    rule!(require(admin_badge.resource_address())),
                    AccessRule::DenyAll
                );

            let mut component = Self {
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

        pub fn create_pool(&mut self, bucket_a: Bucket, bucket_b: Bucket, min_rate: Decimal, max_rate: Decimal) ->(Bucket, Bucket, Bucket)
        {
            assert!(bucket_a.resource_address() != bucket_b.resource_address(), "Two pools cannot trade the same token");

            let (bucket_x, bucket_y) = sort_buckets(bucket_a, bucket_b);
            assert!(self.pools.get(&(bucket_x.resource_address(), bucket_y.resource_address())).is_none(), "A pool trading these tokens already exists");

            let (pool, ret_x, ret_y, position) = PoolComponent::new(bucket_x, bucket_y, min_rate, max_rate);
            self.pools.insert((ret_x.resource_address(), ret_y.resource_address()), pool);
            let ret_pos = self.position_minter.authorize(|| {
                borrow_resource_manager!(self.position_address).mint_non_fungible(
                    &NonFungibleLocalId::Integer(self.position_id.into()),
                    position
                )
            });
            self.position_id+=1;

            (ret_x, ret_y, ret_pos)
        }

        pub fn add_liquidity(&mut self, bucket_a: Bucket, bucket_b: Bucket, opt_position_proof: Option<Proof>) -> (Bucket, Bucket, Option<Bucket>)
        {
            let (bucket_x, bucket_y) = sort_buckets(bucket_a, bucket_b);
            let pool = self.get_pool(bucket_x.resource_address(), bucket_y.resource_address());

            match opt_position_proof {
                Some(position_proof) => {
                    let valid_proof = self.check_single_position_proof(position_proof);
                    let position_nfr = valid_proof.non_fungible::<Position>();
                    let data = self.get_position_data(&position_nfr);

                    let (ret_x, ret_y, new_data) = pool.add_liquidity(bucket_x, bucket_y, data);
                    self.update_position(position_nfr, new_data);
                    (ret_x, ret_y, None)
                }
                None => {
                    let empty_pos = Position::from(bucket_x.resource_address(), bucket_y.resource_address());
                    let (ret_x, ret_y, new_data) = pool.add_liquidity(bucket_x, bucket_y, empty_pos);

                    let bucket_pos = self.position_minter.authorize(|| {
                        borrow_resource_manager!(self.position_address).mint_non_fungible(
                            &NonFungibleLocalId::Integer(self.position_id.into()),
                            new_data
                        )
                    });
                    self.position_id+=1;
                    (ret_x, ret_y, Some(bucket_pos))
                }
            }
        }

        pub fn add_liquidity_between_rates(&mut self, bucket_a: Bucket, bucket_b: Bucket, min_rate: Decimal, max_rate: Decimal, opt_position_proof: Option<Proof>) -> (Bucket, Bucket, Option<Bucket>) {
            let (bucket_x, bucket_y) = sort_buckets(bucket_a, bucket_b);
            let pool = self.get_pool(bucket_x.resource_address(), bucket_y.resource_address());

            match opt_position_proof {
                Some(position_proof) => {
                    let valid_proof = self.check_single_position_proof(position_proof);
                    let position_nfr = valid_proof.non_fungible::<Position>();
                    let data = self.get_position_data(&position_nfr);

                    let (ret_x, ret_y, new_data) = pool.add_liquidity_between_rates(bucket_x, bucket_y, data, min_rate, max_rate);
                    self.update_position(position_nfr, new_data);
                    (ret_x, ret_y, None)
                }
                None => {
                    let empty_pos = Position::from(bucket_x.resource_address(), bucket_y.resource_address());
                    let (ret_x, ret_y, new_data) = pool.add_liquidity_between_rates(bucket_x, bucket_y, empty_pos, min_rate, max_rate);

                    let bucket_pos = self.position_minter.authorize(|| {
                        borrow_resource_manager!(self.position_address).mint_non_fungible(
                            &NonFungibleLocalId::Integer(self.position_id.into()),
                            new_data
                        )
                    });
                    self.position_id+=1;
                    (ret_x, ret_y, Some(bucket_pos))
                }
            }
        }

        pub fn add_liquidity_at_step(&mut self, bucket_a: Bucket, bucket_b: Bucket, step_id: u16, opt_position_proof: Option<Proof>) -> (Bucket, Bucket, Option<Bucket>)
        {
            let (bucket_x, bucket_y) = sort_buckets(bucket_a, bucket_b);
            let pool = self.get_pool(bucket_x.resource_address(), bucket_y.resource_address());

            match opt_position_proof {
                Some(position_proof) => {
                    let valid_proof = self.check_single_position_proof(position_proof);
                    let position_nfr = valid_proof.non_fungible::<Position>();
                    let data = self.get_position_data(&position_nfr);

                    let (ret_x, ret_y, new_data) = pool.add_liquidity_at_step(bucket_x, bucket_y, data, step_id);
                    self.update_position(position_nfr, new_data);
                    (ret_x, ret_y, None)
                }
                None => {
                    let empty_pos = Position::from(bucket_x.resource_address(), bucket_y.resource_address());
                    let (ret_x, ret_y, new_data) = pool.add_liquidity_at_step(bucket_x, bucket_y, empty_pos, step_id);

                    let bucket_pos = self.position_minter.authorize(|| {
                        borrow_resource_manager!(self.position_address).mint_non_fungible(
                            &NonFungibleLocalId::Integer(self.position_id.into()),
                            new_data
                        )
                    });
                    self.position_id+=1;
                    (ret_x, ret_y, Some(bucket_pos))
                }
            }
        }

        pub fn add_liquidity_at_steps(&mut self, bucket_a: Bucket, bucket_b: Bucket, start_step: u16, stop_step: u16, opt_position_proof: Option<Proof>) -> (Bucket, Bucket, Option<Bucket>)
        {
            let (bucket_x, bucket_y) = sort_buckets(bucket_a, bucket_b);
            let pool = self.get_pool(bucket_x.resource_address(), bucket_y.resource_address());

            match opt_position_proof {
                Some(position_proof) => {
                    let valid_proof = self.check_single_position_proof(position_proof);
                    let position_nfr = valid_proof.non_fungible::<Position>();
                    let data = self.get_position_data(&position_nfr);

                    let (ret_x, ret_y, new_data) = pool.add_liquidity_at_steps(bucket_x, bucket_y, data, start_step, stop_step);
                    self.update_position(position_nfr, new_data);
                    (ret_x, ret_y, None)
                }
                None => {
                    let empty_pos = Position::from(bucket_x.resource_address(), bucket_y.resource_address());
                    let (ret_x, ret_y, new_data) = pool.add_liquidity_at_steps(bucket_x, bucket_y, empty_pos, start_step, stop_step);

                    let bucket_pos = self.position_minter.authorize(|| {
                        borrow_resource_manager!(self.position_address).mint_non_fungible(
                            &NonFungibleLocalId::Integer(self.position_id.into()),
                            new_data
                        )
                    });
                    self.position_id+=1;
                    (ret_x, ret_y, Some(bucket_pos))
                }
            }
        }

        pub fn remove_liquidity_at_step(&mut self, position_proof: Proof, step: u16) -> (Bucket, Bucket)
        {
            let valid_proof = self.check_single_position_proof(position_proof);
            let position_nfr = valid_proof.non_fungible::<Position>();
            let data = self.get_position_data(&position_nfr);

            let pool = self.get_pool(data.token_x, data.token_y);
            let (ret_x, ret_y, new_data) = pool.remove_liquidity_at_step(data, step);
            self.update_position(position_nfr, new_data);
            (ret_x, ret_y)
        }

        pub fn remove_liquidity_at_steps(&mut self, position_proof: Proof, start_step: u16, stop_step: u16) -> (Bucket, Bucket)
        {
            let valid_proof = self.check_single_position_proof(position_proof);
            let position_nfr = valid_proof.non_fungible::<Position>();
            let data = self.get_position_data(&position_nfr);

            let pool = self.get_pool(data.token_x, data.token_y);
            let (ret_x, ret_y, new_data) = pool.remove_liquidity_at_steps(data, start_step, stop_step);
            self.update_position(position_nfr, new_data);
            (ret_x, ret_y)
        }

        pub fn remove_liquidity_at_rate(&mut self, position_proof: Proof, rate: Decimal) -> (Bucket, Bucket)
        {
            let valid_proof = self.check_single_position_proof(position_proof);
            let position_nfr = valid_proof.non_fungible::<Position>();
            let data = self.get_position_data(&position_nfr);

            let pool = self.get_pool(data.token_x, data.token_y);
            let (ret_x, ret_y, new_data) = pool.remove_liquidity_at_rate(data, rate);
            self.update_position(position_nfr, new_data);
            (ret_x, ret_y)
        }

        pub fn remove_liquidity_between_rates(&mut self, position_proof: Proof, min_rate: Decimal, max_rate: Decimal) -> (Bucket, Bucket) {
            let valid_proof = self.check_single_position_proof(position_proof);
            let position_nfr = valid_proof.non_fungible::<Position>();
            let data = self.get_position_data(&position_nfr);

            let pool = self.get_pool(data.token_x, data.token_y);
            let (ret_x, ret_y, new_data) = pool.remove_liquidity_between_rates(data, min_rate, max_rate);
            self.update_position(position_nfr, new_data);
            (ret_x, ret_y)
        }

        pub fn remove_all_liquidity(&mut self, positions_bucket: Bucket) -> Vec<Bucket> {
            assert!(positions_bucket.resource_address() == self.position_address);

            let mut buckets: Vec<Bucket> = Vec::new();
            for position_nfr in positions_bucket.non_fungibles::<Position>() {

                let data = self.get_position_data(&position_nfr);
                let pool = self.get_pool(data.token_x, data.token_y);
                let (ret_x, ret_y) = pool.remove_all_liquidity(data);

                buckets.push(ret_x);
                buckets.push(ret_y);
            }
            self.position_minter.authorize(|| { positions_bucket });
            buckets
        }

        pub fn claim_fees(&mut self, positions_proof: Proof) -> Vec<Bucket> {
            let valid_proof = self.check_multiple_position_proof(positions_proof);

            let mut buckets: Vec<Bucket> = Vec::new();
            for position_nfr in valid_proof.non_fungibles::<Position>() {

                let data = self.get_position_data(&position_nfr);
                let pool = self.get_pool(data.token_x, data.token_y);
                let (ret_x, ret_y, new_data) = pool.claim_fees(data);
                self.update_position(position_nfr, new_data);

                buckets.push(ret_x);
                buckets.push(ret_y);
            }
            buckets
        }

        pub fn swap(&mut self, input: Bucket, output: ResourceAddress) -> (Bucket, Bucket) {
            let (token_x, token_y) = sort(input.resource_address(), output);
            let pool = self.get_pool(token_x, token_y);
            pool.swap(input)
        }

        pub fn get_pool_state(&mut self, token_a: ResourceAddress, token_b: ResourceAddress) -> (
        Decimal,
        u16,
        Decimal,
        (Decimal, Decimal),
        Vec<(u16, Vec<Decimal>)>,
        ) {
            let (token_x, token_y) = sort(token_a, token_b);
            let pool = self.get_pool(token_x, token_y);
            pool.get_state()
        }

        fn get_pool(&self, token_x: ResourceAddress, token_y: ResourceAddress) -> &PoolComponent {
            match self.pools.get(&(token_x, token_y)) {
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
    }
}