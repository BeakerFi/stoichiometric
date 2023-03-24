use std::collections::{HashMap, HashSet};
use scrypto::math::Decimal;
use scrypto::prelude::{dec, Instant};
use sqrt::error::Error;
use stoichiometric_tests::dao::sqrt_implem::DaoMethods;
use stoichiometric_tests::dao::utils::{assert_voter_card_is, call_issuer_method, call_router_method, instantiate, lock_positions, lock_stablecoins, vote};
use stoichiometric_tests::dex::pool_state::PoolState;
use stoichiometric_tests::dex::utils::add_liquidity_at_step;
use stoichiometric_tests::dumb_oracle::utils::set_oracle_price;
use stoichiometric_tests::stablecoin::issuer_state::LenderState;
use stoichiometric_tests::stablecoin::sqrt_implem::IssuerMethods;

#[test]
fn test_instantiate() {
    let (test_env, mut dao_state) = instantiate();

    dao_state.update();

    let mut pool_states = HashMap::new();
    let btc_address = test_env.get_resource("btc");
    let btc_pool_state = PoolState{
        router_address: "".to_string(),
        other: btc_address.clone(),
        rate_step: dec!("1.000105411144423293"),
        current_step: 50266,
        min_rate: dec!(100),
        steps: HashMap::new(),
        stable_protocol: Decimal::ZERO,
        other_protocol: Decimal::ZERO,
    };
    pool_states.insert(btc_address.clone(), btc_pool_state);

    let mut lenders = HashMap::new();
    let btc_lender = LenderState::from(Decimal::ZERO, dec!("0.7"), dec!("0.0001"), dec!("1.3"), dec!("0.1"));
    lenders.insert(btc_address.clone(), btc_lender);

    dao_state.assert_variables_are(0, 0, Decimal::ZERO, 86400, dec!("0.5"));
    dao_state.assert_pool_states(&pool_states);
    dao_state.assert_issuer_state(&HashMap::new(), &lenders, 0, 0);
    dao_state.assert_reserves_state(&HashMap::new());
    dao_state.assert_proposals_state(&HashMap::new());

}

#[test]
fn test_lock_stablecoins_no_voter_card() {
    let (mut test_env, mut dao_state) = instantiate();

    // We take a loan to get stablecoins and then we lock them
    set_oracle_price(&mut test_env, "btc", dec!(20000));
    call_issuer_method(&mut test_env, IssuerMethods::TakeLoan(
        "btc".to_string(),
        dec!(3),
        dec!(42000),
    )).run();

    // Set back current component as the DAO component to make the next call
    test_env.set_current_component("dao_component");

    lock_stablecoins(&mut test_env, dec!(30000), None)
        .run();

    dao_state.update();

    // We only check variables because from the other tests, we know that the issuer and the btc lender will act as expected
    dao_state.assert_variables_are(1, 0, dec!(30000), 86400, dec!("0.5"));
    assert_voter_card_is(&test_env, "#0#".to_string(), dec!(30000), dec!(30000), vec![], 0, HashSet::new());
}

#[test]
fn test_lock_stablecoins_with_voter_card() {
    let (mut test_env, mut dao_state) = instantiate();
    set_oracle_price(&mut test_env, "btc", dec!(20000));
    call_issuer_method(&mut test_env, IssuerMethods::TakeLoan(
        "btc".to_string(),
        dec!(3),
        dec!(42000),
    )).run();

    // Set back current component as the DAO component to make the next call
    test_env.set_current_component("dao_component");

    lock_stablecoins(&mut test_env, dec!(20000), None)
        .run();

    lock_stablecoins(&mut test_env, dec!(13000), Some("#0#".to_string()))
        .run();

    dao_state.update();

    // We only check variables because from the other tests, we know that the issuer and the btc lender will act as expected
    dao_state.assert_variables_are(1, 0, dec!(33000), 86400, dec!("0.5"));
    assert_voter_card_is(&test_env, "#0#".to_string(), dec!(33000), dec!(33000), vec![], 0, HashSet::new());
}

#[test]
fn test_lock_positions_no_voter_card() {
    let (mut test_env, mut dao_state) = instantiate();

    set_oracle_price(&mut test_env, "btc", dec!(20000));
    call_issuer_method(&mut test_env, IssuerMethods::TakeLoan(
        "btc".to_string(),
        dec!(3),
        dec!(42000),
    )).run();

    test_env.set_current_component("router_component");
    add_liquidity_at_step(&mut test_env, Decimal::ZERO, "btc", dec!(20), 65000, None).run();

    // Set back current component as the DAO component to make the next call
    test_env.set_current_component("dao_component");
    lock_positions(&mut test_env, vec!["#0#".to_string()], None).run();

    dao_state.update();

    // We only check variables because from the other tests, we know that the issuer and the btc lender will act as expected
    dao_state.assert_variables_are(1, 0, dec!("1890337.133000178498278"), 86400, dec!("0.5"));
    assert_voter_card_is(&test_env, "#0#".to_string(), dec!("1890337.133000178498278"), Decimal::ZERO, vec!["#0#".to_string()], 0, HashSet::new());
}

#[test]
fn test_lock_positions_with_voter_card() {
    let (mut test_env, mut dao_state) = instantiate();

    set_oracle_price(&mut test_env, "btc", dec!(20000));
    call_issuer_method(&mut test_env, IssuerMethods::TakeLoan(
        "btc".to_string(),
        dec!(3),
        dec!(42000),
    )).run();

    test_env.set_current_component("router_component");
    add_liquidity_at_step(&mut test_env, Decimal::ZERO, "btc", dec!(10), 65000, None).run();
    add_liquidity_at_step(&mut test_env, Decimal::ZERO, "btc", dec!(10), 65000, None).run();

    // Set back current component as the DAO component to make the next call
    test_env.set_current_component("dao_component");
    lock_positions(&mut test_env, vec!["#0#".to_string()], None).run();
    lock_positions(&mut test_env, vec!["#1#".to_string()], Some("#0#".to_string())).run();

    dao_state.update();

    // We only check variables because from the other tests, we know that the issuer and the btc lender will act as expected
    dao_state.assert_variables_are(1, 0, dec!("1890337.133000178498278"), 86400, dec!("0.5"));
    assert_voter_card_is(&test_env, "#0#".to_string(), dec!("1890337.133000178498278"), Decimal::ZERO, vec!["#0#".to_string(), "#1#".to_string()], 0, HashSet::new());
}

#[test]
fn test_lock_position_and_stablecoins() {
    let (mut test_env, mut dao_state) = instantiate();

    set_oracle_price(&mut test_env, "btc", dec!(20000));
    call_issuer_method(&mut test_env, IssuerMethods::TakeLoan(
        "btc".to_string(),
        dec!(3),
        dec!(42000),
    )).run();

    test_env.set_current_component("router_component");
    add_liquidity_at_step(&mut test_env, Decimal::ZERO, "btc", dec!(20), 65000, None).run();

    test_env.set_current_component("dao_component");
    lock_stablecoins(&mut test_env, dec!(20000), None).run();
    lock_positions(&mut test_env, vec!["#0#".to_string()], Some("#0#".to_string())).run();

    dao_state.update();

    // We only check variables because from the other tests, we know that the issuer and the btc lender will act as expected
    dao_state.assert_variables_are(1, 0, dec!("1910337.133000178498278"), 86400, dec!("0.5"));
    assert_voter_card_is(&test_env, "#0#".to_string(), dec!("1910337.133000178498278"), Decimal::ZERO, vec!["#0#".to_string()], 0, HashSet::new());
}

#[test]
fn test_unlock() {

    let (mut test_env, mut dao_state) = instantiate();

    set_oracle_price(&mut test_env, "btc", dec!(20000));
    call_issuer_method(&mut test_env, IssuerMethods::TakeLoan(
        "btc".to_string(),
        dec!(3),
        dec!(42000),
    )).run();

    test_env.set_current_component("router_component");
    add_liquidity_at_step(&mut test_env, Decimal::ZERO, "btc", dec!(20), 65000, None).run();

    test_env.set_current_component("dao_component");
    lock_stablecoins(&mut test_env, dec!(20000), None).run();
    lock_positions(&mut test_env, vec!["#0#".to_string()], Some("#0#".to_string())).run();

    test_env.call_method(DaoMethods::Unlock("#0#".to_string())).run();

    dao_state.update();

    // We only check variables because from the other tests, we know that the issuer and the btc lender will act as expected
    dao_state.assert_variables_are(1, 0, Decimal::ZERO, 86400, dec!("0.5"));
    assert_voter_card_is(&test_env, "#0#".to_string(), dec!("1890337.133000178498278"), Decimal::ZERO, vec![], 0, HashSet::new());
}

#[test]
fn test_vote_for() {
    let (mut test_env, mut dao_state) = instantiate();

    // We take a loan to get stablecoins and then we lock them
    set_oracle_price(&mut test_env, "btc", dec!(20000));
    call_issuer_method(&mut test_env, IssuerMethods::TakeLoan(
        "btc".to_string(),
        dec!(3),
        dec!(42000),
    )).run();

    // Set back current component as the DAO component to make the next call
    test_env.set_current_component("dao_component");


    lock_stablecoins(&mut test_env, dec!(20000), None).run();

    // Make a proposal
    test_env.call_method(DaoMethods::MakeChangeVotePeriodProposal(3)).run();

    dao_state.update();

    // Vote for the proposal
    vote(&mut test_env, dao_state.get_proposal(0), "#0#".to_string(), true).run();
    let mut proposals_voted = HashSet::new();
    proposals_voted.insert(0);
    assert_voter_card_is(&test_env, "#0#".to_string(), dec!(20000), dec!(20000), vec![], 0, HashSet::new());
}

#[test]
fn test_vote_against() {
    let (mut test_env, mut dao_state) = instantiate();

    // We take a loan to get stablecoins and then we lock them
    set_oracle_price(&mut test_env, "btc", dec!(20000));
    call_issuer_method(&mut test_env, IssuerMethods::TakeLoan(
        "btc".to_string(),
        dec!(3),
        dec!(42000),
    )).run();

    // Set back current component as the DAO component to make the next call
    test_env.set_current_component("dao_component");


    lock_stablecoins(&mut test_env, dec!(20000), None).run();

    // Make a proposal
    test_env.call_method(DaoMethods::MakeChangeVotePeriodProposal(3)).run();

    dao_state.update();

    // Vote against the proposal
    vote(&mut test_env, dao_state.get_proposal(0), "#0#".to_string(), false).run();
    let mut proposals_voted = HashSet::new();
    proposals_voted.insert(0);
    assert_voter_card_is(&test_env, "#0#".to_string(), dec!(20000), dec!(20000), vec![], 0, HashSet::new());
}

#[test]
fn test_change_vote_period_proposal() {
    let (mut test_env, mut dao_state) = instantiate();

    // We take a loan to get stablecoins and then we lock them
    set_oracle_price(&mut test_env, "btc", dec!(20000));
    call_issuer_method(&mut test_env, IssuerMethods::TakeLoan(
        "btc".to_string(),
        dec!(3),
        dec!(42000),
    )).run();

    // Set back current component as the DAO component to make the next call
    test_env.set_current_component("dao_component");


    lock_stablecoins(&mut test_env, dec!(20000), None).run();

    // Make a proposal
    test_env.call_method(DaoMethods::MakeChangeVotePeriodProposal(3)).run();

    dao_state.update();

    // Vote for the proposal
    vote(&mut test_env, dao_state.get_proposal(0), "#0#".to_string(), true).run();

    // Wait after 1 day
    test_env.set_current_time(Instant::new(90000));

    // Execute Proposal
    test_env.call_method(DaoMethods::ExecuteProposal("#0#".to_string())).run();

    dao_state.update();

    // Check proposal worked
    dao_state.assert_variables_are(1, 1, dec!(20000), 3, dec!("0.5"));
    let mut proposals_voted = HashSet::new();
    proposals_voted.insert(0);
    assert_voter_card_is(&test_env, "#0#".to_string(), dec!(20000), dec!(20000), vec![], 0, HashSet::new());
}

#[test]
fn test_proposals_lasts_right_amount_of_time() {
    let (mut test_env, mut dao_state) = instantiate();

    // We take a loan to get stablecoins and then we lock them
    set_oracle_price(&mut test_env, "btc", dec!(20000));
    call_issuer_method(&mut test_env, IssuerMethods::TakeLoan(
        "btc".to_string(),
        dec!(3),
        dec!(42000),
    )).run();

    // Set back current component as the DAO component to make the next call
    test_env.set_current_component("dao_component");

    lock_stablecoins(&mut test_env, dec!(20000), None).run();

    // Make a proposal
    test_env.call_method(DaoMethods::MakeChangeVotePeriodProposal(3)).run();



    // Try to execute before vote
    test_env.call_method(DaoMethods::ExecuteProposal("#0#".to_string()))
        .should_panic(Error::AssertFailed("Vote has not finished yet!".to_string()))
        .run();

    // Try again one minute before (precision is only in minutes on Radix ledger)
    test_env.set_current_time(Instant::new(86360));
    test_env.call_method(DaoMethods::ExecuteProposal("#0#".to_string()))
        .should_panic(Error::AssertFailed("Vote has not finished yet!".to_string()))
        .run();


    // Try after right amount of time
    test_env.set_current_time(Instant::new(90000));
    test_env.call_method(DaoMethods::ExecuteProposal("#0#".to_string())).run();

    dao_state.update();

    // The vote_period has not changed because no votes were casted
    dao_state.assert_variables_are(1, 1, dec!(20000), 86400, dec!("0.5"));
    assert_voter_card_is(&test_env, "#0#".to_string(), dec!(20000), dec!(20000), vec!["#0#".to_string()], 0, HashSet::new());
}
