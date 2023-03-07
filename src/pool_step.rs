use scrypto::blueprint;

#[blueprint]
mod pool_step {
    use crate::constants::{LP_FEE, TRADED};
    use crate::position::StepPosition;

    pub struct PoolStep {
        /// Vault containing X tokens as liquidity
        x_vault: Vault,

        /// Vault containing Y tokens as liquidity
        y_vault: Vault,

        /// Price of the pool step
        rate: Decimal,

        /// Accrued fees in token X per liquidity unit
        x_fees_per_liq: Decimal,

        /// Accrued fees in token Y per liquidity unit
        y_fees_per_liq: Decimal,

        /// Vault containing fees in token X
        x_fees_vault: Vault,

        /// Vault containing fees in token Y
        y_fees_vault: Vault,
    }

    impl PoolStep {
        /// Instantiates a new PoolStep Component and returns it
        ///
        /// # Arguments
        /// * `x_token` - Address of the X token that will be traded by the beaker
        /// * `y_token` - Address of the Y token that will be traded by the beaker
        /// * `rate` - Fixed rate for this PoolStep
        pub fn new(
            x_token: ResourceAddress,
            y_token: ResourceAddress,
            rate: Decimal,
        ) -> PoolStepComponent {
            // Create the component
            let component = Self {
                x_vault: Vault::new(x_token.clone()),
                y_vault: Vault::new(y_token.clone()),
                rate: rate,
                x_fees_per_liq: Decimal::ZERO,
                y_fees_per_liq: Decimal::ZERO,
                x_fees_vault: Vault::new(x_token.clone()),
                y_fees_vault: Vault::new(y_token.clone()),
            }
            .instantiate();

            component
        }

        /// Adds liquidity to the PoolStep given two buckets and returns the excess amount of tokens
        ///
        /// # Arguments
        /// * `x_bucket` - Bucket containing X tokens to be added as liquidity
        /// * `y_bucket` - Bucket containing Y tokens to be added as liquidity
        /// * `step_position` - StepPosition already held by the user
        pub fn add_liquidity(
            &mut self,
            mut x_bucket: Bucket,
            mut y_bucket: Bucket,
            step_position: StepPosition,
        ) -> (Bucket, Bucket, StepPosition) {
            // Start by claiming_fees and adding them as potential liquidity
            let (fees_x, fees_y, mut new_step_position) = self.claim_fees(step_position);
            x_bucket.put(fees_x);
            y_bucket.put(fees_y);

            let buckets_rate = x_bucket.amount() / y_bucket.amount();

            // Right amount of tokens to take from the given buckets
            let right_x;
            let right_y;

            if buckets_rate > self.rate {
                // In this case, there is an excess of x token input
                right_y = y_bucket.amount();
                right_x = y_bucket.amount() * self.rate;
            } else {
                // In this case, there is an excess of y token input
                right_x = x_bucket.amount();
                right_y = x_bucket.amount() / self.rate;
            }

            self.x_vault.put(x_bucket.take(right_x));
            self.y_vault.put(y_bucket.take(right_y));

            // Update the StepPosition
            new_step_position.liquidity += right_x * self.rate + right_y;

            (x_bucket, y_bucket, new_step_position)
        }

        /// Removes liquidity from a PoolStep.
        /// Returns buckets containing X and Y tokens (unclaimed fees and tokens associated to removed
        /// liquidity) and the new value of the Position.
        ///
        /// # Arguments
        /// * `liquidity` - Amount of liquidity to remove from the beaker
        /// * `step_position` - StepPosition already held by the user
        pub fn remove_liquidity(
            &mut self,
            liquidity: Decimal,
            step_position: StepPosition,
        ) -> (Bucket, Bucket, StepPosition) {
            // Start by claiming fees
            let real_liquidity = liquidity.min(step_position.liquidity);
            let (mut x_bucket, mut y_bucket, mut new_step_position) =
                self.claim_fees(step_position);

            // Compute amount of tokens to return and put them in the buckets
            let x = self.x_vault.amount();
            let y = self.y_vault.amount();
            let l = x * self.rate + y;
            let y_fraction = y / l;
            y_bucket.put(self.x_vault.take(y_fraction * real_liquidity));
            let x_take = (Decimal::ONE - y_fraction) * l / self.rate;
            x_bucket.put(self.x_vault.take(x_take));

            // Update the StepPosition
            new_step_position.liquidity -= real_liquidity;

            (x_bucket, y_bucket, new_step_position)
        }

        pub fn claim_fees(
            &mut self,
            step_position: StepPosition,
        ) -> (Bucket, Bucket, StepPosition) {
            // Compute the fees to give
            let x_fees =
                (self.x_fees_per_liq - step_position.last_x_fees_per_liq) * step_position.liquidity;
            let y_fees =
                (self.y_fees_per_liq - step_position.last_y_fees_per_liq) * step_position.liquidity;

            // Put the fees in buckets
            let x_bucket = self.x_fees_vault.take(x_fees);
            let y_bucket = self.y_fees_vault.take(y_fees);

            //
            let mut new_step_position = step_position.clone();
            new_step_position.last_x_fees_per_liq = self.x_fees_per_liq;
            new_step_position.last_y_fees_per_liq = self.y_fees_per_liq;

            (x_bucket, y_bucket, new_step_position)
        }

        /// Swaps X tokens for Y tokens
        ///
        /// # Arguments
        /// * `input_tokens` - bucket containing X tokens to be swapped for Y tokens.
        /// * `fees` - bucket containing fees in X tokens
        pub fn swap_for_y(&mut self, mut input: Bucket) -> (Bucket, Bucket) {
            let lp_fee = Decimal(BnumI256::from(LP_FEE));
            let traded = Decimal(BnumI256::from(TRADED));

            // Compute the real amount of tokens to be traded
            let max_y = input.amount() / self.rate / (Decimal::ONE + lp_fee);
            let real_y = max_y.min(self.y_vault.amount());
            let real_x = self.rate * real_y;

            // Take fees
            let fees = real_x * lp_fee;
            let l = self.rate * self.x_vault.amount() + self.y_vault.amount();
            self.x_fees_per_liq += fees / l;
            self.x_fees_vault.put(input.take(fees));

            // Make the swap
            self.x_vault.put(input.take(real_x * traded));
            let output = self.y_vault.take(real_y * traded);

            (input, output)
        }

        /// Swaps Y tokens for X tokens
        ///
        /// # Arguments
        /// * `input_tokens` - bucket containing Y tokens to be swapped for X tokens.
        /// * `fees` - bucket containing fees in Y tokens
        pub fn swap_for_x(&mut self, mut input: Bucket) -> (Bucket, Bucket) {
            let lp_fee = Decimal(BnumI256::from(LP_FEE));
            let traded = Decimal(BnumI256::from(TRADED));

            // Compute the real amount of tokens to be traded
            let max_x = input.amount() * self.rate / (Decimal::ONE + lp_fee);
            let real_x = max_x.min(self.x_vault.amount());
            let real_y = real_x / self.rate;

            // Take fees
            let fees = real_y * lp_fee;
            let l = self.rate * self.x_vault.amount() + self.y_vault.amount();
            self.y_fees_per_liq += fees / l;
            self.y_fees_vault.put(input.take(fees));

            // Make the swap
            let output = self.x_vault.take(real_x * traded);
            self.y_vault.put(input.take(real_y * traded));

            (input, output)
        }

        pub fn pool_step_state(&self) -> Vec<Decimal> {
            vec![
                self.x_vault.amount(),
                self.y_vault.amount(),
                self.rate,
                self.x_fees_per_liq,
                self.y_fees_per_liq,
                self.x_fees_vault.amount(),
                self.y_fees_vault.amount(),
            ]
        }
    }
}
