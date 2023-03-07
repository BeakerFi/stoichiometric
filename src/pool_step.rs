use scrypto::blueprint;

// p = x/y
// x + p*y = k;


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
        /// * `token_x` - Address of the X token that will be traded by the beaker
        /// * `token_y` - Address of the Y token that will be traded by the beaker
        /// * `rate` - Fixed rate for this PoolStep
        pub fn new(
            token_x: ResourceAddress,
            token_y: ResourceAddress,
            rate: Decimal,
        ) -> PoolStepComponent {
            // Create the component
            let component = Self {
                x_vault: Vault::new(token_x.clone()),
                y_vault: Vault::new(token_y.clone()),
                rate: rate,
                x_fees_per_liq: Decimal::ZERO,
                y_fees_per_liq: Decimal::ZERO,
                x_fees_vault: Vault::new(token_x.clone()),
                y_fees_vault: Vault::new(token_y.clone()),
            }
            .instantiate();

            component
        }

        /// Adds liquidity to the PoolStep given two buckets and returns the excess amount of tokens
        ///
        /// # Arguments
        /// * `bucket_x` - Bucket containing X tokens to be added as liquidity
        /// * `bucket_y` - Bucket containing Y tokens to be added as liquidity
        /// * `step_position` - StepPosition already held by the user
        pub fn add_liquidity(
            &mut self,
            mut bucket_x: Bucket,
            mut bucket_y: Bucket,
            step_position: StepPosition,
        ) -> (Bucket, Bucket, StepPosition) {
            // Start by claiming_fees and adding them as potential liquidity
            let (fees_x, fees_y, mut new_step_position) = self.claim_fees(step_position);
            bucket_x.put(fees_x);
            bucket_y.put(fees_y);

            let buckets_rate = bucket_x.amount() / bucket_y.amount();

            // Right amount of tokens to take from the given buckets
            let right_x;
            let right_y;

            if buckets_rate > self.rate {
                // In this case, there is an excess of x token input
                right_y = bucket_y.amount();
                right_x = bucket_y.amount() * self.rate;
            } else {
                // In this case, there is an excess of y token input
                right_x = bucket_x.amount();
                right_y = bucket_x.amount() / self.rate;
            }

            self.x_vault.put(bucket_x.take(right_x));
            self.y_vault.put(bucket_y.take(right_y));

            // Update the StepPosition
            new_step_position.liquidity += right_x + right_y*self.rate;

            (bucket_x, bucket_y, new_step_position)
        }

        /// Removes liquidity from a PoolStep.
        /// Returns buckets containing X and Y tokens (unclaimed fees and tokens associated to removed
        /// liquidity).
        ///
        /// # Arguments
        /// * `liquidity` - Amount of liquidity to remove from the beaker
        pub fn remove_liquidity(
            &mut self,
            step_position: StepPosition,
        ) -> (Bucket, Bucket) {

            let liquidity = step_position.liquidity;

            // Start by claiming fees
            let (mut bucket_x, mut bucket_y, _) = self.claim_fees(step_position);

            // Compute amount of tokens to return and put them in the buckets
            let x = self.x_vault.amount();
            let y = self.y_vault.amount();
            let l = x + y*self.rate;
            let x_fraction = x / l;
            bucket_x.put(self.x_vault.take(x_fraction * liquidity));
            let y_take = (Decimal::ONE - x_fraction) * l / self.rate;
            bucket_y.put(self.x_vault.take(y_take));


            (bucket_x, bucket_y)
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
            let bucket_x = self.x_fees_vault.take(x_fees);
            let bucket_y = self.y_fees_vault.take(y_fees);

            //
            let mut new_step_position = step_position.clone();
            new_step_position.last_x_fees_per_liq = self.x_fees_per_liq;
            new_step_position.last_y_fees_per_liq = self.y_fees_per_liq;

            (bucket_x, bucket_y, new_step_position)
        }

        /// Swaps X tokens for Y tokens
        ///
        /// # Arguments
        /// * `input_tokens` - bucket containing X tokens to be swapped for Y tokens.
        /// * `fees` - bucket containing fees in X tokens
        pub fn swap_for_y(&mut self, mut input: Bucket) -> (Bucket, Bucket) {

            // Compute the real amount of tokens to be traded
            let max_y = input.amount() * self.rate / (Decimal::ONE + LP_FEE);
            let real_y = max_y.min(self.y_vault.amount());
            let real_x = self.rate / real_y;

            // Take fees
            let fees = real_x * LP_FEE;
            let l = self.x_vault.amount() + self.rate * self.y_vault.amount();
            self.x_fees_per_liq += fees / l;
            self.x_fees_vault.put(input.take(fees));

            // Make the swap
            self.x_vault.put(input.take(real_x * TRADED));
            let output = self.y_vault.take(real_y * TRADED);

            (input, output)
        }

        /// Swaps Y tokens for X tokens
        ///
        /// # Arguments
        /// * `input_tokens` - bucket containing Y tokens to be swapped for X tokens.
        /// * `fees` - bucket containing fees in Y tokens
        pub fn swap_for_x(&mut self, mut input: Bucket) -> (Bucket, Bucket) {

            // Compute the real amount of tokens to be traded
            let max_x = input.amount() / self.rate / (Decimal::ONE + LP_FEE);
            let real_x = max_x.min(self.x_vault.amount());
            let real_y = real_x * self.rate;

            // Take fees
            let fees = real_y * LP_FEE;
            let l = self.x_vault.amount() + self.rate * self.y_vault.amount();
            self.y_fees_per_liq += fees / l;
            self.y_fees_vault.put(input.take(fees));

            // Make the swap
            let output = self.x_vault.take(real_x * TRADED);
            self.y_vault.put(input.take(real_y * TRADED));

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
