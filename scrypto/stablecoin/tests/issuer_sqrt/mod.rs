use scrypto::prelude::Decimal;
use sqrt::blueprint::Blueprint;
use sqrt::method::{Arg, Method};
use sqrt::method::Arg::{ComponentAddressArg, DecimalArg, FungibleBucketArg, NonFungibleBucketArg, NonFungibleLocalId, NonFungibleProofArg, ResourceAddressArg, StringArg, VecArg};
use sqrt::method_args;

pub(crate) const LOAN_NAME: &str = "Stoichiometric Loan";
pub(crate) const FLASH_MINT_NAME: &str = "Stoichiometric Flash Mint";
pub(crate) const STABLECOIN_NAME: &str = "Stoichiometric USD";
pub(crate) const ADMIN_BADGE_NAME: &str = "Issuer admin badge";

pub struct IssuerBlueprint {}

impl Blueprint for IssuerBlueprint {
    fn instantiation_name(&self) -> &str {
        "new"
    }

    fn name(&self) -> &str {
        "Issuer"
    }

    fn has_admin_badge(&self) -> bool {
        true
    }
}

pub enum IssuerMethods {
    NewLender(String, Decimal, Decimal, Decimal, Decimal, String),
    TakeLoan(String, Decimal, Decimal),
    RepayLoans(Decimal, Vec<String>),
    AddCollateral(String, Decimal, String),
    RemoveCollateral(Decimal, String),
    Liquidate(Decimal, String),
    LiquidateList(Decimal, Vec<String>),
    ChangeLenderParameters(String, Decimal, Decimal, Decimal, Decimal, String)
}

impl Method for IssuerMethods {
    fn name(&self) -> &str {
        match self {
            IssuerMethods::NewLender(_, _, _, _, _, _) => { "new_lender" }
            IssuerMethods::TakeLoan(_, _, _) => { "take_loan" }
            IssuerMethods::RepayLoans(_, _) => { "repay_loans" }
            IssuerMethods::AddCollateral(_, _, _) => { "add_collateral" }
            IssuerMethods::RemoveCollateral(_, _) => { "remove_collateral" }
            IssuerMethods::Liquidate(_, _) => { "liquidate" }
            IssuerMethods::LiquidateList(_, _) => { "liquidate_list" }
            IssuerMethods::ChangeLenderParameters(_, _, _, _, _, _) => { "change_lender_parameters" }
        }
    }

    fn args(&self) -> Option<Vec<Arg>> {
        match self {
            IssuerMethods::NewLender(collateral_token, loan_to_value, interest_rate, liquidation_threshold, liquidation_incentive, oracle) =>
                {
                    method_args!(
                        ResourceAddressArg(collateral_token.clone()),
                        DecimalArg(loan_to_value.clone()),
                        DecimalArg(interest_rate.clone()),
                        DecimalArg(liquidation_threshold.clone()),
                        DecimalArg(liquidation_incentive.clone()),
                        ComponentAddressArg(oracle.clone())
                    )
                }
            IssuerMethods::TakeLoan(collateral_token, collateral_amount, amount_to_loan) =>
                {
                    method_args!(
                        FungibleBucketArg(collateral_token.clone(), collateral_amount.clone()),
                        DecimalArg(amount_to_loan.clone())
                    )
                }
            IssuerMethods::RepayLoans(repayment_amount, loan_ids) =>
                {
                    method_args!(
                        FungibleBucketArg(STABLECOIN_NAME.to_string(), repayment_amount.clone()),
                        NonFungibleBucketArg(LOAN_NAME.to_string(), loan_ids.clone())
                    )
                }
            IssuerMethods::AddCollateral(collateral_token, collateral_amount, loan_id) =>
                {
                    method_args!(
                        FungibleBucketArg(collateral_token.clone(), collateral_amount.clone()),
                        NonFungibleProofArg(LOAN_NAME.to_string(), vec![loan_id.clone()])
                    )
                }
            IssuerMethods::RemoveCollateral(amount, loan_id) =>
                {
                    method_args!(
                        DecimalArg(amount.clone()),
                        NonFungibleProofArg(LOAN_NAME.to_string(), vec![loan_id.clone()])
                    )
                }
            IssuerMethods::Liquidate(repayment_amount, loan_id) =>
                {
                    let boxed_arg = Box::new(StringArg(loan_id.clone()));
                    method_args!(
                        FungibleBucketArg(STABLECOIN_NAME.to_string(), repayment_amount.clone()),
                        NonFungibleLocalId(boxed_arg)
                    )
                }
            IssuerMethods::LiquidateList(repayment_amount, loan_ids) =>
                {
                    let mut vec = vec![];
                    for loan_id in loan_ids {
                        let boxed_loan_id = Box::new(StringArg(loan_id.clone()));
                        let nfr_id = NonFungibleLocalId(boxed_loan_id);
                        vec.push(nfr_id);
                    }
                    method_args!(
                        FungibleBucketArg(STABLECOIN_NAME.to_string(), repayment_amount.clone()),
                        VecArg(vec)
                    )
                }
            IssuerMethods::ChangeLenderParameters(collateral_token, loan_to_value, interest_rate, liquidation_threshold, liquidation_incentive, oracle) =>
                {
                    method_args!(
                        ResourceAddressArg(collateral_token.clone()),
                        DecimalArg(loan_to_value.clone()),
                        DecimalArg(interest_rate.clone()),
                        DecimalArg(liquidation_threshold.clone()),
                        DecimalArg(liquidation_incentive.clone()),
                        ComponentAddressArg(oracle.clone())
                    )
                }
        }
    }

    fn needs_admin_badge(&self) -> bool {
        match self
        {
            IssuerMethods::NewLender(_,_,_,_,_,_) | IssuerMethods::ChangeLenderParameters(_,_,_,_,_,_) => true,
            _ => false
        }
    }
}

