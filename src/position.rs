use scrypto::prelude::*;

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub struct StepPosition {
    /// Liquidity of the position
    pub liquidity: Decimal,

    /// Value of the `y_fees_per_liq` variable of the PoolStep
    /// last time that the position collected fees
    pub last_stable_fees_per_liq: Decimal,

    /// Value of the `y_fees_per_liq` variable of the PoolStep
    /// last time that the position collected fees
    pub last_other_fees_per_liq: Decimal,
}

impl StepPosition {

    pub fn new() -> Self {
        Self {
            liquidity: Decimal::zero(),
            last_stable_fees_per_liq: Decimal::zero(),
            last_other_fees_per_liq: Decimal::zero(),
        }
    }
    pub fn update(&mut self, new_position: StepPosition)
    {
        self.liquidity = new_position.liquidity;
        self.last_stable_fees_per_liq = new_position.last_stable_fees_per_liq;
        self.last_other_fees_per_liq = new_position.last_other_fees_per_liq;
    }
}

#[derive(NonFungibleData, ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub struct Position {

    /// Token of the position
    pub token: ResourceAddress,

    /// Second token of the position
    #[mutable]
    pub step_positions: HashMap<u16, StepPosition>,
}

impl Position {
    pub fn from(token: ResourceAddress) -> Self {
        Self {
            token,
            step_positions: HashMap::new(),
        }
    }

    pub fn get_step(&self, step: u16) -> StepPosition {
        match self.step_positions.get(&step) {
            None => StepPosition::new(),
            Some(step_position) => step_position.clone(),
        }
    }

    pub fn insert_step(&mut self, step: u16, pool_step: StepPosition) {
        self.step_positions.insert(step, pool_step);
    }

    pub fn remove_step(&mut self, step: u16) -> StepPosition {
        match self.step_positions.remove(&step) {
            None => StepPosition::new(),
            Some(step_position) => step_position,
        }
    }
}
