use scrypto::{blueprint, external_component};

external_component! {
    OracleComponent {
        fn get_twap_since(&self, token: ResourceAddress, timestamp: i64) -> Decimal;
        fn new_observation(&mut self, token: ResourceAddress);
    }
}

#[blueprint]
mod lender {
    use crate::loan::Loan;
    use crate::constants::SECONDS_PER_DAY;

    pub struct Lender {
        collateral: Vault,
        loan_to_value: Decimal,
        interest_rate: Decimal,
        liquidation_threshold: Decimal,
        liquidation_penalty: Decimal,
        oracle: ComponentAddress
    }

    impl Lender {

        pub fn new(collateral_address: ResourceAddress, loan_to_value: Decimal, interest_rate: Decimal, liquidation_threshold: Decimal, liquidation_penalty: Decimal, oracle: ComponentAddress) -> LenderComponent {
            Self{
                collateral: Vault::new(collateral_address),
                loan_to_value,
                interest_rate,
                liquidation_threshold,
                liquidation_penalty,
                oracle
            }
                .instantiate()
        }

        pub fn take_loan(&mut self, collateral: Bucket, amount_to_loan: Decimal) -> Loan {

            let price = self.get_oracle_price();

            let collateral_needed = amount_to_loan / ( self.loan_to_value * price );

            assert!(collateral.amount() >= collateral_needed,
                    "You need to provide at least {} tokens to loan {}",
                    collateral_needed,
                    amount_to_loan);

            let current_time = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            let loan = Loan::from(collateral.resource_address(), collateral.amount(), amount_to_loan, current_time, self.loan_to_value, self.interest_rate);
            self.collateral.put(collateral);
            loan
        }

        pub fn repay_loan(&mut self, repayment: Decimal, loan: Loan) -> (Decimal, Bucket) {

            let interests = self.compute_loan_interests(&loan);
            assert!(repayment >= loan.amount_lent + interests, "You need to provide {} stablecoins to repay your loan", loan.amount_lent + interests);

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
            let minimum_collateral =  loan.amount_lent *(Decimal::ONE +  loan_days_duration * loan.interest_rate)/ (self.loan_to_value * price);
            assert!(new_collateral_amount >= minimum_collateral, "The new collateral amount should be at least {}", minimum_collateral);

            loan.collateral_amount = new_collateral_amount;
            (loan, self.collateral.take(collateral))
        }

        pub fn liquidate(&mut self, stablecoins_amount: Decimal, mut loan: Loan) -> (Decimal, Bucket, Bucket, Loan) {
            let interests = self.compute_loan_interests(&loan);
            let price = self.get_oracle_price();

            let collateral_value = loan.collateral_amount * price;
            let loan_value = loan.amount_lent + interests;

            assert!(collateral_value / loan_value <= self.liquidation_threshold,
                    "Cannot liquidate this loan because liquidation threshold was not hit");

            let minimum_stablecoins_input = ( loan_value * self.liquidation_threshold - loan.collateral_amount*(Decimal::ONE - self.liquidation_penalty)*price )/(self.liquidation_threshold - Decimal::ONE);

            assert!(stablecoins_amount >= minimum_stablecoins_input, "You need to provide {} stablecoins to liquidate this loan", minimum_stablecoins_input);

            let new_collateral_amount = loan.collateral_amount * (Decimal::ONE - self.liquidation_penalty) - minimum_stablecoins_input/price;

            let collateral_to_share = loan.collateral_amount - new_collateral_amount;
            let liquidator_bucket = self.collateral.take(collateral_to_share * dec!("0.9"));
            let reserve_bucket = self.collateral.take(collateral_to_share * dec!("0.1"));

            loan.collateral_amount = new_collateral_amount;
            loan.amount_lent = loan.amount_lent - minimum_stablecoins_input;

            (minimum_stablecoins_input, liquidator_bucket, reserve_bucket, loan)
        }

        pub fn change_parameters(&mut self, loan_to_value: Decimal, interest_rate: Decimal,  liquidation_threshold: Decimal, liquidation_penalty: Decimal)
        {
            self.loan_to_value = loan_to_value;
            self.interest_rate = interest_rate;
            self.liquidation_threshold = liquidation_threshold;
            self.liquidation_penalty = liquidation_penalty;
        }

        pub fn change_oracle(&mut self, oracle: ComponentAddress)
        {
            self.oracle = oracle;
        }

        pub fn get_state(&self) -> Vec<Decimal>{
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

        fn compute_loan_interests(&self, loan: &Loan) -> Decimal {
            let current_time = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            let loan_days_duration = Decimal::from(current_time - loan.loan_date) / SECONDS_PER_DAY;

            loan.amount_lent*loan_days_duration*loan.interest_rate
        }

    }


}