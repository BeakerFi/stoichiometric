use scrypto::prelude::*;

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub struct StepPosition {
    /// Step of the position
    pub step: u16,

    /// Liquidity of the position
    pub liquidity: Decimal,

    /// Value of the `y_fees_per_liq` variable of the PoolStep
    /// last time that the position collected fees
    pub last_x_fees_per_liq: Decimal,

    /// Value of the `y_fees_per_liq` variable of the PoolStep
    /// last time that the position collected fees
    pub last_y_fees_per_liq: Decimal,
}

impl StepPosition {
    pub fn new() -> Self {
        Self {
            step: 0,
            liquidity: Decimal::zero(),
            last_x_fees_per_liq: Decimal::zero(),
            last_y_fees_per_liq: Decimal::zero(),
        }
    }

    pub fn from(step: u16) -> Self {
        Self {
            step,
            liquidity: Decimal::zero(),
            last_x_fees_per_liq: Decimal::zero(),
            last_y_fees_per_liq: Decimal::zero(),
        }
    }
}

#[derive(NonFungibleData, ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode)]
pub struct Position {
    /// First token of the position
    pub token_x: ResourceAddress,

    /// Second token of the position
    pub token_y: ResourceAddress,

    /// Second token of the position
    #[mutable]
    pub step_positions: HashMap<u16, StepPosition>,
}

impl Position {
    pub fn new(token_x: ResourceAddress, token_y: ResourceAddress) -> Self {
        Self {
            token_x,
            token_y,
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
}
