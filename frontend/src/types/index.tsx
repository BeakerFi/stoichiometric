type token = {
    address: string,
    name: string, 
    symb: string, 
    icon_url: string
}

type step = {
    step_id: number
    stablecoin_amount: number,

    other_token_amount: number,

    rate: number,

}

type pool = {
    token: token,
    rate_step: number,

    current_step: number,

    min_rate: number,

    max_rate: number,

    steps: step[]
}

type position = {
    token: token | null,
    liquidity: number,
    price_x: number,
    value_locked: number | '?',
    x_fees: number | '?',
    y_fees: number | '?',
    nfIdValue: any,
    id: string | null
}

type account = {
    address: string,
    name: string
}

type tokenOwned = number[];

type lender = {

    collateral_token: token,

    collateral_price: number,

    oracle: string,

    loan_to_value: number,

    interest_rate: number,

    liquidation_threshold: number,

    liquidation_penalty: number,

}

type loan = {
    collateral_token: token,

    collateral_amount: number,

    amount_lent: number,

    loan_date: number,

    loan_to_value: number,

    interest_rate: number
}

export type {token, pool, step, position, account, tokenOwned, lender, loan};