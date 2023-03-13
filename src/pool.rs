use scrypto::blueprint;

#[blueprint]
mod pool {
    use crate::constants::{NB_STEP, PROTOCOL_FEE};
    use crate::decimal_maths::{ln, pow};
    use crate::pool_step::PoolStepComponent;
    use crate::position::Position;

    pub struct Pool {
        /// Percentage rate increase between each PoolStep
        rate_step: Decimal,

        /// Current step
        current_step: u16,

        ///
        min_rate: Decimal,

        steps: HashMap<u16, PoolStepComponent>,

        stable_protocol_fees: Vault,

        other_protocol_fees: Vault,
    }

    impl Pool {
        pub fn new(
            bucket_stable: Bucket,
            bucket_other: Bucket,
            initial_rate: Decimal,
            min_rate: Decimal,
            max_rate: Decimal,
        ) -> (PoolComponent, Bucket, Bucket, Position) {
            assert!(min_rate > Decimal::ZERO, "The minimum rate should be positive");
            assert!(max_rate > min_rate, "The maximum rate should be greater than the minimum rate");
            assert!(initial_rate >= min_rate && initial_rate <= max_rate, "The initial rate should be included in the given rate range");

            // Computes the rate % change between each steps
            let exponent = Decimal::ONE / NB_STEP;
            let rate_step = pow::<Decimal, Decimal>(max_rate / min_rate, exponent) - Decimal::ONE;

            // Computes the current pool step from input tokens
            let dec_step = ln(initial_rate / min_rate) / ln(Decimal::ONE + rate_step);
            assert!(dec_step >= Decimal::zero() && dec_step <= Decimal::from(NB_STEP));
            let current_step: u16 = ((dec_step.floor().0) / Decimal::ONE.0).try_into().unwrap();

            let component = Self {
                rate_step,
                current_step,
                min_rate,
                steps: HashMap::new(),
                stable_protocol_fees: Vault::new(bucket_stable.resource_address()),
                other_protocol_fees: Vault::new(bucket_other.resource_address()),
            }
            .instantiate();

            let position = Position::from(bucket_other.resource_address());
            let (stable_ret, other_ret, pos_ret) =
                component.add_liquidity_at_step(bucket_stable, bucket_other, position, current_step);

            (component, stable_ret, other_ret, pos_ret)
        }

        pub fn add_liquidity(
            &mut self,
            bucket_stable: Bucket,
            bucket_other: Bucket,
            rate: Decimal,
            position: Position,
        ) -> (Bucket, Bucket, Position) {
            let step_id = self.step_at_rate(rate);
            self.add_liquidity_at_step(bucket_stable, bucket_other, position, step_id)
        }

        pub fn add_liquidity_between_rates(&mut self, bucket_stable: Bucket, bucket_other: Bucket, position: Position, min_rate: Decimal, max_rate: Decimal) -> (Bucket, Bucket, Position) {
            let min_step = self.step_at_rate(min_rate);
            let max_step = self.step_at_rate(max_rate);
            info!("Adding liquidity between steps: {} and {} ({}, {})", min_step, max_step, min_rate, max_rate);
            self.add_liquidity_at_steps(bucket_stable, bucket_other, position, min_step, max_step)
        }

        pub fn add_liquidity_at_step(
            &mut self,
            bucket_stable: Bucket,
            bucket_other: Bucket,
            mut position: Position,
            step_id: u16,
        ) -> (Bucket, Bucket, Position) {
            let step_position = position.get_step(step_id);

            // Get or create the given step
            let pool_step = match self.steps.get_mut(&step_id) {
                Some(ps) => ps,
                None => {
                    let rate = self.rate_at_step(step_id);
                    let new_step = PoolStepComponent::new(
                        self.stable_protocol_fees.resource_address(),
                        self.other_protocol_fees.resource_address(),
                        rate,
                    );
                    self.steps.insert(step_id, new_step);
                    self.steps.get(&step_id).unwrap()
                }
            };

            // Add liquidity to step and return
            let (stable_return, other_return, new_step) =
                pool_step.add_liquidity(bucket_stable, bucket_other, self.current_step <= step_id, step_position);
            position.insert_step(step_id, new_step);

            (stable_return, other_return, position)
        }

        pub fn add_liquidity_at_steps(&mut self, mut bucket_stable: Bucket, mut bucket_other: Bucket, position: Position, start_step: u16, stop_step: u16) -> (Bucket, Bucket, Position) {
            let nb_steps = stop_step - start_step + 1;
            let stable_per_step = bucket_stable.amount()/nb_steps;
            let other_per_step = bucket_other.amount()/nb_steps;
            let mut position = position;
            let mut ret_stable = Bucket::new(bucket_stable.resource_address());
            let mut ret_other = Bucket::new(bucket_other.resource_address());
            for i in start_step..stop_step+1 {
                let (tmp_stable, tmp_other, tmp_pos) = self.add_liquidity_at_step(bucket_stable.take(stable_per_step), bucket_other.take(other_per_step), position, i);
                ret_stable.put(tmp_stable);
                ret_other.put(tmp_other);
                position = tmp_pos;
            }
            ret_stable.put(bucket_stable);
            ret_other.put(bucket_other);
            (ret_stable, ret_other, position)
        }

        pub fn remove_liquidity_at_step(
            &mut self,
            mut position: Position,
            step_id: u16,
        ) -> (Bucket, Bucket, Position) {
            let step_position = position.remove_step(step_id);
            let mut bucket_stable = Bucket::new(self.stable_protocol_fees.resource_address());
            let mut bucket_other = Bucket::new(position.token);

            if step_position.liquidity > Decimal::ZERO {
                let pool_step = self.steps.get(&step_id).unwrap();
                (bucket_stable, bucket_other) = pool_step.remove_liquidity(step_position);
            }
            (bucket_stable, bucket_other, position)
        }

        pub fn remove_liquidity_at_steps(&mut self, position: Position, start_step: u16, stop_step: u16) -> (Bucket, Bucket, Position) {
            let mut ret_stable = Bucket::new(self.stable_protocol_fees.resource_address());
            let mut ret_other = Bucket::new(position.token);
            let mut ret_pos = position;

            for i in start_step..stop_step+1 {
                let (tmp_stable, tmp_other, tmp_pos) = self.remove_liquidity_at_step(ret_pos, i);
                ret_stable.put(tmp_stable);
                ret_other.put(tmp_other);
                ret_pos = tmp_pos;
            }
            (ret_stable, ret_other, ret_pos)
        }

        pub fn remove_liquidity_at_rate(
            &mut self,
            position: Position,
            rate: Decimal,
        ) -> (Bucket, Bucket, Position) {
            let step_id = self.step_at_rate(rate);
            self.remove_liquidity_at_step(position, step_id)
        }

        pub fn remove_all_liquidity(&mut self, position: Position) -> (Bucket, Bucket) {
            let step_positions = position.step_positions;
            let mut bucket_stable = Bucket::new(self.stable_protocol_fees.resource_address());
            let mut bucket_other = Bucket::new(position.token);

            for (step, step_position) in step_positions {
                let pool_step = self.steps.get(&step).unwrap();
                let (tmp_stable, tmp_other) =
                    pool_step.remove_liquidity(step_position);
                bucket_stable.put(tmp_stable);
                bucket_other.put(tmp_other);
            }
            (bucket_stable, bucket_other)
        }

        pub fn claim_fees(&mut self, position: Position) -> (Bucket, Bucket, Position) {
            let step_positions = position.step_positions;
            let mut bucket_stable = Bucket::new(self.stable_protocol_fees.resource_address());
            let mut bucket_other = Bucket::new(position.token);

            let mut new_position = Position::from(position.token);
            for (step, step_position) in step_positions {
                let pool_step = self.steps.get(&step).unwrap();
                let (tmp_stable, tmp_other, step_position) = pool_step.claim_fees(step_position);
                bucket_stable.put(tmp_stable);
                bucket_other.put(tmp_other);
                new_position.insert_step(step, step_position);
            }

            (bucket_stable, bucket_other, new_position)
        }

        pub fn swap(&mut self, input_bucket: Bucket) -> (Bucket, Bucket) {
            if input_bucket.resource_address() == self.stable_protocol_fees.resource_address() {
                self.swap_for_other(input_bucket)
            } else {
                self.swap_for_stable(input_bucket)
            }
        }

        fn swap_for_other(&mut self, mut input_bucket: Bucket) -> (Bucket, Bucket) {
            // Input bucket has stable tokens

            // Take protocol fees
            self.stable_protocol_fees
                .put(input_bucket.take(input_bucket.amount() * PROTOCOL_FEE));

            let mut other_ret = Bucket::new(self.other_protocol_fees.resource_address());
            let mut stable_ret = Bucket::from(input_bucket);

            loop {
                match self.steps.get_mut(&self.current_step) {
                    Some(pool_step) => {
                        let (other_tmp, stable_tmp) = pool_step.swap_for_other(stable_ret);
                        other_ret.put(other_tmp);
                        stable_ret = stable_tmp;

                        if stable_ret.amount() == Decimal::ZERO {
                            break;
                        }
                    }
                    None => {}
                };

                if self.current_step == 0
                {
                    break;
                }
                self.current_step -= 1;
            }

            (other_ret, stable_ret)
        }

        fn swap_for_stable(&mut self, mut input_bucket: Bucket) -> (Bucket, Bucket) {
            // Input bucket has other tokens

            // Take protocol fees
            self.other_protocol_fees
                .put(input_bucket.take(input_bucket.amount() * PROTOCOL_FEE));

            let mut other_ret = Bucket::from(input_bucket);
            let mut stable_ret = Bucket::new(self.stable_protocol_fees.resource_address());

            loop {
                match self.steps.get_mut(&self.current_step) {
                    Some(pool_step) => {
                        let (stable_tmp, other_tmp) = pool_step.swap_for_stable(other_ret);
                        other_ret = other_tmp;
                        stable_ret.put(stable_tmp);

                        if other_ret.amount() == Decimal::ZERO {
                            break;
                        }
                    }
                    None => {}
                };
                if self.current_step == 0 {
                    break;
                } else {
                    self.current_step += 1;
                }
            }

            (other_ret, stable_ret)
        }

        pub fn claim_protocol_fees(&mut self) -> (Bucket, Bucket)
        {
            (self.stable_protocol_fees.take_all(), self.other_protocol_fees.take_all())
        }

        pub fn get_state(
            &self,
        ) -> (
            Decimal,
            u16,
            Decimal,
            (Decimal, Decimal),
            Vec<(u16, Vec<Decimal>)>,
        ) {
            let mut pool_steps_state = vec![];

            for (step_id, pool_step) in &self.steps {
                let state = pool_step.pool_step_state();

                pool_steps_state.push((*step_id, state));
            }


            (self.rate_step,
             self.current_step,
             self.min_rate,
             (self.stable_protocol_fees.amount(), self.other_protocol_fees.amount()),
             pool_steps_state)
        }

        #[inline]
        pub fn rate_at_step(&self, step_id: u16) -> Decimal {
            self.min_rate * (Decimal::ONE + self.rate_step).powi(step_id.into())
        }

        pub fn step_at_rate(&self, rate: Decimal) -> u16 {
            // rate = min_rate*(1 + rate_step)**step => ln(rate/min_rate) = step*ln(1 + rate_step)
            let dec_step = ln(rate / self.min_rate) / ln(Decimal::ONE + self.rate_step);
            assert!(dec_step >= Decimal::zero() && dec_step <= Decimal::from(NB_STEP));
            let step_id: u16 = ((dec_step.floor().0) / Decimal::ONE.0).try_into().unwrap();
            step_id
        }
    }
}
