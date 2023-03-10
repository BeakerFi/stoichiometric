use scrypto::prelude::*;

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub struct StepPosition {
    /// Step of the position
    pub step: u16,

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

    pub fn from(step: u16) -> Self {
        Self {
            step,
            liquidity: Decimal::zero(),
            last_stable_fees_per_liq: Decimal::zero(),
            last_other_fees_per_liq: Decimal::zero(),
        }
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
            None => StepPosition::from(step),
            Some(step_position) => step_position.clone(),
        }
    }

    pub fn insert_step(&mut self, pool_step: StepPosition) {
        let id = pool_step.step;
        self.step_positions.insert(id, pool_step);
    }

    pub fn remove_step(&mut self, step: u16) -> StepPosition {
        match self.step_positions.remove(&step) {
            None => StepPosition::from(step),
            Some(step_position) => step_position,
        }
    }
}
