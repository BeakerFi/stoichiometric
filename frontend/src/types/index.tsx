type token = {
    address: string,
    name: string, 
    symb: string, 
    icon_url: string
}

type step = number[]

type pool = {
    token: token,
    id: string
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

type loan = {
    token: token,
    collateral_amount: number,
    amount_lent: number,
    loan_time: number,
    loan_to_value: number,
    interest_rate: number,
    id: string
}

type lender = any

export type {token, pool, step, position, account, tokenOwned, lender, loan};