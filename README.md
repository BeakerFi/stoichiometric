# SUSD: Stoichiometric USD

Stoichiometric is a Collateralized Debt Position (CDP) Stablecoin ecosystem which integrates 3 main features: a DAO, a Stablecoin, an AMM
# DAO
The DAO controls the whole stablecoin ecosystem. It has admin badge access to the DEX and the Stabelcoin lender. Users can make the following decisions:
- Change the voting period for the DAO
- Change the minimum amount of votes to make a proposal valid
- Change the parameters of a Lender
- Allow a new token to be used as collateral
- Give a non-fungible resource that allows the minting and burning of the stablecoin. This enables new protocols to enter the stablecoin ecosystem
- Recall a stablecoin minter. This enables the users to remove a protocol from the ecosystem.

# DEX
The DEX is vastly inspired by TraderJoe, which is basically Uniswap v3 but with a constant-sum AMM between each tick. The DEX is built so that every pair has to include SUSD. The goal of the DEX is to concentrate as much liquidity around SUSD as possible. Therefore, the DEX pairs graph is a star with SUS at the center. This enables in theory to make SUSD more stable.

# Issuer
The stablecoin lender enables users to lock allowed collateral against SUSD. The liquidation process differs from MakerDAO, DAI and AAVE in the sense that it uses a constant-product AMM curve to decide the amount of collateral to be liquidated when the liquidation threshold is exceeded.

## References:
- Uniswap v3 whitepaper: https://uniswap.org/whitepaper-v3.pdf
- TraderJoe docs: https://docs.traderjoexyz.com/
- Curve's stablecoin withepaper: https://github.com/curvefi/curve-stablecoin/blob/master/doc/curve-stablecoin.pdf
- MakerDao docs: https://makerdao.com/en/whitepaper/

### Natural Logarithm function is not fully optimized
Due to a lack of time to make a implement the natural logarithm, the current cost for a `ln` call can be up to 5 XRD. 
Therefore, the current fee limit (10 XRD) does not allow to work with rates. 