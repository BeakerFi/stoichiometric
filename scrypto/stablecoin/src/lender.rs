use scrypto::{blueprint, external_component};

external_component! {
    OracleComponent {
        fn get_twap_since(&self, token: ResourceAddress, timestamp: i64) -> Decimal;
        fn new_observation(&mut self, token: ResourceAddress);
    }
}

#[blueprint]
mod lender {
    use crate::constants::SECONDS_PER_DAY;
    use crate::loan::Loan;

    pub struct Lender {
        collateral: Vault,
        loan_to_value: Decimal,
        interest_rate: Decimal,
        liquidation_threshold: Decimal,
        protocol_liquidation_share: Decimal,
        oracle: ComponentAddress,
    }

    impl Lender {
        pub fn new(
            collateral_address: ResourceAddress,
            loan_to_value: Decimal,
            interest_rate: Decimal,
            liquidation_threshold: Decimal,
            liquidation_penalty: Decimal,
            oracle: ComponentAddress,
        ) -> LenderComponent {
            Self {
                collateral: Vault::new(collateral_address),
                loan_to_value,
                interest_rate,
                liquidation_threshold,
                liquidation_penalty,
                oracle,
            }
            .instantiate()
        }

        pub fn take_loan(&mut self, collateral: Bucket, amount_to_loan: Decimal) -> Loan {
            let price = self.get_oracle_price();

            let collateral_needed = amount_to_loan / (self.loan_to_value * price);

            assert!(
                collateral.amount() >= collateral_needed,
                "You need to provide at least {} tokens to loan {}",
                collateral_needed,
                amount_to_loan
            );

            let current_time = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            let loan = Loan::from(
                collateral.resource_address(),
                collateral.amount(),
                amount_to_loan,
                current_time,
                self.loan_to_value,
                self.interest_rate,
            );
            self.collateral.put(collateral);
            loan
        }

        pub fn repay_loan(&mut self, repayment: Decimal, loan: Loan) -> (Decimal, Bucket) {
            let interests = self.compute_loan_interests(&loan);
            assert!(
                repayment >= loan.amount_lent + interests,
                "You need to provide {} stablecoins to repay your loan",
                loan.amount_lent + interests
            );

            let collateral = self.collateral.take(loan.collateral_amount);

            (interests, collateral)
        }

        pub fn add_collateral(&mut self, collateral: Bucket, mut loan: Loan) -> Loan {
            loan.collateral_amount = loan.collateral_amount + collateral.amount();
            self.collateral.put(collateral);
            loan
        }

        pub fn remove_collateral(&mut self, collateral: Decimal, mut loan: Loan) -> (Loan, Bucket) {
            let new_collateral_amount = loan.collateral_amount - collateral;
            let price = self.get_oracle_price();
            let current_time = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            let loan_days_duration = Decimal::from(current_time - loan.loan_date) / SECONDS_PER_DAY;
            let minimum_collateral = loan.amount_lent
                * (Decimal::ONE + loan_days_duration * loan.interest_rate)
                / (self.loan_to_value * price);
            assert!(
                new_collateral_amount >= minimum_collateral,
                "The new collateral amount should be at least {}",
                minimum_collateral
            );

            loan.collateral_amount = new_collateral_amount;
            (loan, self.collateral.take(collateral))
        }

        pub fn liquidate(
            &mut self,
            stablecoins_amount: Decimal,
            mut loan: Loan,
        ) -> (Decimal, Bucket, Bucket, Loan) {

            // First check that the loan can indeed be liquidated
            let accrued_interests = self.compute_interests(&loan);
            let total_lent = loan.amount_lent + accrued_interests;
            let collateral_price = self.get_oracle_price();
            let current_value_ratio = loan.collateral_amount * collateral_price / total_lent;
            assert!(current_value_ratio <= self.liquidation_threshold,
                    "Cannot liquidate this loan: the value ration is {} >= {}",
                    current_value_ratio,
                    self.liquidation_threshold);

            // If the previous assert worked, then it means that the loan can be partially or fully liquidated
            // In the case where the total amount lent is more valuable than the collateral, we liquidate everything
            if total_lent >= collateral_price*loan.collateral_amount {

                // In this case, we fully liquidate the loan and only take the interests that can be
                // paid

                assert!(stablecoins_amount >= loan.amount_lent,
                        "Please provide at least {} SUSD to liquidate this loan",
                        loan.amount_lent);

                // We only claim the interests that can be claimed
                let real_interests = accrued_interests.min(collateral_price*loan.collateral_amount - loan.amount_lent);
                let stablecoin_interest = real_interests / collateral_price;
                let liquidator_amount = loan.collateral_amount - stablecoin_interest;

                let liquidator_share = self.collateral.take(liquidator_amount);
                let protocol_share = self.collateral.take(stablecoin_interest);
                let stablecoins_needed = loan.amount_lent;

                loan.amount_lent = Decimal::ZERO;
                lon.collateral_amount = Decimal::ZERO;

                (stablecoins_needed, liquidator_share, protocol_share, loan)

            }
            else {

                // We liquidate partially to reach the right loan to value and we don't harvest interests

                let new_total_lent = loan.collateral_amount * collateral_price / self.liquidation_threshold;
                let stablecoins_needed = total_lent - new_total_lent;

                // Check that the user provided enough to liquidate the loan
                assert!(stablecoins_amount >= stablecoins_needed,
                        "Please provide at least {} SUSD to liquidate this loan",
                        stablecoins_needed
                );

                let new_collateral_amount = loan.collateral_amount * loan.loan_to_value / self.liquidation_threshold;
                let collateral_out = loan.collateral_amount - new_collateral_amount;

                let mut liquidator_share = self.collateral.take(collateral_out);
                let protocol_share = liquidator_share.take(collateral_out * self.protocol_liquidation_share);

                loan.amount_lent = new_total_lent;
                loan.collateral_amount = new_collateral_amount;

                (stablecoins_needed, liquidator_share, protocol_share, loan)
            }
        }

        pub fn clear_bad_debt(&mut self, mut loan :Loan) -> (Decimal, Bucket, Loan){
            let collateral_price = self.get_oracle_price();

            // Check that there is indeed bad debt
            let collateral_price = self.get_oracle_price();
            let collateral_value = loan.collateral_amount * collateral_price;

            assert!(collateral_value < loan.amount_lent,
                    "There is no bad debt to clear!");

            // If there is bad debt, fully liquidate the loan
            let collateral = self.collateral(loan.collateral_amount);
            let amount_to_clear = loan.amount_lent;
            loan.collateral_amount = Decimal::ZERO;
            loan.amount_lent = Decimal::ZERO;

            (amount_to_clear, collateral, loan)
        }

        pub fn change_parameters(
            &mut self,
            loan_to_value: Decimal,
            interest_rate: Decimal,
            liquidation_threshold: Decimal,
            liquidation_penalty: Decimal,
        ) {
            self.loan_to_value = loan_to_value;
            self.interest_rate = interest_rate;
            self.liquidation_threshold = liquidation_threshold;
            self.liquidation_penalty = liquidation_penalty;
        }

        pub fn change_oracle(&mut self, oracle: ComponentAddress) {
            self.oracle = oracle;
        }

        pub fn get_state(&self) -> Vec<Decimal> {
            vec![
                self.collateral.amount(),
                self.loan_to_value,
                self.interest_rate,
                self.liquidation_threshold,
                self.liquidation_penalty,
            ]
        }

        fn get_oracle_price(&self) -> Decimal {
            let mut oracle = OracleComponent::at(self.oracle);

            // Look at the TWAP with as much data as possible
            let price = oracle.get_twap_since(self.collateral.resource_address(), 0);

            // Make a new observation to have more input to the oracle for future requests
            oracle.new_observation(self.collateral.resource_address());

            price
        }

        fn compute_interests(&self, loan: &Loan) -> Decimal {
            let current_time = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            let loan_days_duration = Decimal::from(current_time - loan.loan_date) / SECONDS_PER_DAY;

            loan.amount_lent * loan_days_duration * loan.interest_rate
        }
    }
}
