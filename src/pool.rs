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

        x_protocol_fees: Vault,

        y_protocol_fees: Vault,
    }

    impl Pool {
        pub fn new(
            bucket_x: Bucket,
            bucket_y: Bucket,
            min_rate: Decimal,
            max_rate: Decimal,
        ) -> (PoolComponent, Bucket, Bucket, Position) {
            // Computes the rate % change between each steps
            let exponent = Decimal::ONE / NB_STEP;
            let rate_step = pow::<Decimal, Decimal>(max_rate / min_rate, exponent) - Decimal::ONE;

            // Computes the current pool step from input tokens
            let rate = bucket_x.amount() / bucket_y.amount();
            let dec_step = ln(rate / min_rate) / ln(Decimal::ONE + rate_step);
            assert!(dec_step >= Decimal::zero() && dec_step <= Decimal::from(NB_STEP));
            let current_step: u16 = ((dec_step.floor().0) / Decimal::ONE.0).try_into().unwrap();

            let component = Self {
                rate_step,
                current_step,
                min_rate,
                steps: HashMap::new(),
                x_protocol_fees: Vault::new(bucket_x.resource_address()),
                y_protocol_fees: Vault::new(bucket_y.resource_address()),
            }
            .instantiate();

            let position = Position::from(bucket_x.resource_address(), bucket_y.resource_address());
            let (x_ret, y_ret, pos_ret) =
                component.add_liquidity_at_step(bucket_x, bucket_y, position, current_step);

            (component, x_ret, y_ret, pos_ret)
        }

        pub fn add_liquidity(
            &mut self,
            bucket_x: Bucket,
            bucket_y: Bucket,
            position: Position,
        ) -> (Bucket, Bucket, Position) {
            let rate = bucket_x.amount() / bucket_y.amount();
            let step_id = self.step_at_rate(rate);
            self.add_liquidity_at_step(bucket_x, bucket_y, position, step_id)
        }

        pub fn add_liquidity_between_rates(&mut self, bucket_x: Bucket, bucket_y: Bucket, position: Position, min_rate: Decimal, max_rate: Decimal) -> (Bucket, Bucket, Position) {
            let min_step = self.step_at_rate(min_rate);
            let max_step = self.step_at_rate(max_rate);
            self.add_liquidity_at_steps(bucket_x, bucket_y, position, min_step, max_step)
        }

        pub fn add_liquidity_at_step(
            &mut self,
            bucket_x: Bucket,
            bucket_y: Bucket,
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
                        self.x_protocol_fees.resource_address(),
                        self.y_protocol_fees.resource_address(),
                        rate,
                    );
                    self.steps.insert(step_id, new_step);
                    self.steps.get(&step_id).unwrap()
                }
            };

            // Add liquidity to step and return
            let (x_return, y_return, new_step) =
                pool_step.add_liquidity(bucket_x, bucket_y, step_position);
            position.insert_step(new_step);

            (x_return, y_return, position)
        }

        pub fn add_liquidity_at_steps(&mut self, mut bucket_x: Bucket, mut bucket_y: Bucket, position: Position, start_step: u16, stop_step: u16) -> (Bucket, Bucket, Position) {
            let nb_steps = stop_step - start_step;
            let x_per_step = bucket_x.amount()/nb_steps;
            let y_per_step = bucket_y.amount()/nb_steps;
            let mut position = position;
            let mut ret_x = Bucket::new(bucket_x.resource_address());
            let mut ret_y = Bucket::new(bucket_y.resource_address());

            for i in start_step..stop_step+1 {
                let (tmp_x, tmp_y, tmp_pos) = self.add_liquidity_at_step(bucket_x.take(x_per_step), bucket_y.take(y_per_step), position, i);
                ret_x.put(tmp_x);
                ret_y.put(tmp_y);
                position = tmp_pos;
            }
            (ret_x, ret_y, position)
        }

        pub fn remove_liquidity_at_step(
            &mut self,
            mut position: Position,
            step_id: u16,
        ) -> (Bucket, Bucket, Position) {
            let step_position = position.remove_step(step_id);
            let bucket_x = Bucket::new(position.token_x);
            let bucket_y = Bucket::new(position.token_y);

            if step_position.liquidity > Decimal::ZERO {
                let pool_step = self.steps.get(&step_id).unwrap();
                let (bucket_x, bucket_y) = pool_step.remove_liquidity(step_position);
                (bucket_x, bucket_y, position)
            } else {
                (bucket_x, bucket_y, position)
            }
        }

        pub fn remove_liquidity_at_steps(&mut self, position: Position, start_step: u16, stop_step: u16) -> (Bucket, Bucket, Position) {
            let mut ret_x = Bucket::new(position.token_x);
            let mut ret_y = Bucket::new(position.token_y);
            let mut ret_pos = position;

            for i in start_step..stop_step+1 {
                let (tmp_x, tmp_y, tmp_pos) = self.remove_liquidity_at_step(ret_pos, i);
                ret_x.put(tmp_x);
                ret_y.put(tmp_y);
                ret_pos = tmp_pos;
            }
            (ret_x, ret_y, ret_pos)
        }

        pub fn remove_liquidity_at_rate(
            &mut self,
            position: Position,
            rate: Decimal,
        ) -> (Bucket, Bucket, Position) {
            let step_id = self.step_at_rate(rate);
            self.remove_liquidity_at_step(position, step_id)
        }

        pub fn remove_liquidity_between_rates(&mut self, position: Position, min_rate: Decimal, max_rate: Decimal) -> (Bucket, Bucket, Position) {
            let min_step = self.step_at_rate(min_rate);
            let max_step = self.step_at_rate(max_rate);
            self.remove_liquidity_at_steps(position, min_step, max_step)
        }

        pub fn remove_all_liquidity(&mut self, position: Position) -> (Bucket, Bucket) {
            let step_positions = position.step_positions;
            let mut bucket_x = Bucket::new(position.token_x);
            let mut bucket_y = Bucket::new(position.token_y);

            for (_, step_position) in step_positions {
                let pool_step = self.steps.get(&step_position.step).unwrap();
                let (tmp_x, tmp_y) =
                    pool_step.remove_liquidity(step_position);
                bucket_x.put(tmp_x);
                bucket_y.put(tmp_y);
            }
            (bucket_x, bucket_y)
        }

        pub fn claim_fees(&mut self, position: Position) -> (Bucket, Bucket, Position) {
            let step_positions = position.step_positions;
            let mut bucket_x = Bucket::new(position.token_x);
            let mut bucket_y = Bucket::new(position.token_y);

            let mut new_position = Position::from(position.token_x, position.token_y);
            for (_, step_position) in step_positions {
                let pool_step = self.steps.get(&step_position.step).unwrap();
                let (tmp_x, tmp_y, step_position) = pool_step.claim_fees(step_position);
                bucket_x.put(tmp_x);
                bucket_y.put(tmp_y);
                new_position.insert_step(step_position);
            }

            (bucket_x, bucket_y, new_position)
        }

        pub fn swap(&mut self, input_bucket: Bucket) -> (Bucket, Bucket) {
            if input_bucket.resource_address() == self.x_protocol_fees.resource_address() {
                self.swap_for_y(input_bucket)
            } else {
                self.swap_for_x(input_bucket)
            }
        }

        fn swap_for_x(&mut self, mut input_bucket: Bucket) -> (Bucket, Bucket) {
            // Input bucket has tokens Y

            // Take protocol fees
            self.x_protocol_fees
                .put(input_bucket.take(input_bucket.amount() * PROTOCOL_FEE));

            let mut x_ret = Bucket::new(self.x_protocol_fees.resource_address());
            let mut y_ret = Bucket::from(input_bucket);

            loop {
                match self.steps.get_mut(&self.current_step) {
                    Some(pool_step) => {
                        let (x_tmp, y_tmp) = pool_step.swap_for_x(y_ret);
                        x_ret.put(x_tmp);
                        y_ret = y_tmp;

                        if y_ret.amount() == Decimal::ZERO {
                            break;
                        } else {
                            self.current_step -= 1;
                        }
                    }
                    None => {
                        if self.current_step == 0 {
                            break;
                        } else {
                            self.current_step -= 1;
                        }
                    }
                };
            }

            (x_ret, y_ret)
        }

        fn swap_for_y(&mut self, mut input_bucket: Bucket) -> (Bucket, Bucket) {
            // Input bucket has tokens X

            // Take protocol fees
            self.y_protocol_fees
                .put(input_bucket.take(input_bucket.amount() * PROTOCOL_FEE));

            let mut x_ret = Bucket::from(input_bucket);
            let mut y_ret = Bucket::new(self.y_protocol_fees.resource_address());

            loop {
                match self.steps.get_mut(&self.current_step) {
                    Some(pool_step) => {
                        let (x_tmp, y_tmp) = pool_step.swap_for_y(x_ret);
                        x_ret = x_tmp;
                        y_ret.put(y_tmp);

                        if x_ret.amount() == Decimal::ZERO {
                            break;
                        } else {
                            self.current_step += 1;
                        }
                    }
                    None => {
                        if self.current_step == 0 {
                            break;
                        } else {
                            self.current_step += 1;
                        }
                    }
                };
            }

            (x_ret, y_ret)
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
             (self.x_protocol_fees.amount(), self.y_protocol_fees.amount()),
             pool_steps_state)
        }

        #[inline]
        pub fn rate_at_step(&self, step_id: u16) -> Decimal {
            self.min_rate * (Decimal::ONE + self.rate_step).powi(step_id.into())
        }

        pub fn step_at_rate(&self, rate: Decimal) -> u16 {
            // rate = min_rate*(1 + rate_step)**step => ln(rate) = step*ln(1 + rate_step)
            let dec_step = ln(rate / self.min_rate) / ln(Decimal::ONE + self.rate_step);
            assert!(dec_step >= Decimal::zero() && dec_step <= Decimal::from(NB_STEP));
            let step_id: u16 = ((dec_step.floor().0) / Decimal::ONE.0).try_into().unwrap();
            step_id
        }
    }
}
