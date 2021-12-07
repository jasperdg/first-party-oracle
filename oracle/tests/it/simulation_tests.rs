use crate::utils::*;

// Scenario: Bob creates pair, and Alice requests it
#[test]
fn dr_scenario_1() {
    let init_res = TestUtils::init();
    // let init_balance_alice = init_res.alice.get_token_balance(None);
    // let init_balance_bob = init_res.bob.get_token_balance(None);
    // println!(
    //     "Request interface before data request creation: {}",
    //     init_res
    //         .alice
    //         .get_token_balance(Some(ORACLE_CONTRACT_ID.to_string()))
    // );
    
    // let _new_dr_res = init_res.alice.dr_new(0, Some(validity_bond));
    // let dr_exist = init_res.alice.dr_exists(0);
    // assert!(dr_exist, "something went wrong during dr creation");

    // println!(
    //     "Request interface before staking: {}",
    //     init_res
    //         .alice
    //         .get_token_balance(Some(REQUESTER_CONTRACT_ID.to_string()))
    // );
    // // println!("Bob balance before staking:   {}", init_balance_bob); // same for carol

    // for i in 0..12 {
    //     let bond_size = calc_bond_size(validity_bond, i, None); // stake 2, 4, 16, 32, ...
    //                                                             // even numbers => Bob stakes on correct outcome
    //                                                             // odd numbers => Carol stakes on incorrect outcome
    //     match i % 2 == 0 {
    //         true => {
    //             println!(
    //                 "Round {}, bond size: {}, staking correctly with Bob",
    //                 i, bond_size
    //             );
    //             let pre_stake_balance_bob = init_res.bob.get_token_balance(None);
    //             let outcome_to_stake = Outcome::Answer(AnswerType::String("test".to_string()));
    //             let _res = init_res.bob.stake(0, outcome_to_stake, bond_size);
    //             let post_stake_balance_bob = init_res.bob.get_token_balance(None);
    //             // make sure no refund (bond size is exactly met)
    //             assert_eq!(post_stake_balance_bob, pre_stake_balance_bob - bond_size);
    //         }
    //         false => {
    //             println!(
    //                 "Round {}, bond size: {}, staking incorrectly with Carol",
    //                 i, bond_size
    //             );
    //             let pre_stake_balance_carol = init_res.carol.get_token_balance(None);
    //             let outcome_to_stake =
    //                 Outcome::Answer(AnswerType::String("test_wrong".to_string()));
    //             let _res = init_res.carol.stake(0, outcome_to_stake, bond_size);
    //             let post_stake_balance_carol = init_res.carol.get_token_balance(None);
    //             // make sure no refund (bond size is exactly met)
    //             assert_eq!(
    //                 post_stake_balance_carol,
    //                 pre_stake_balance_carol - bond_size
    //             );
    //         }
    //     };
    //     // println!("Request interface balance after stake: {}", init_res.alice.get_token_balance(Some(REQUESTER_CONTRACT_ID.to_string())));
    // }

    // // since final arbitrator is invoked, any stakes after this point will be fully refunded

    // // get balances before finalization and claim and amount spent on staking
    // let pre_claim_balance_bob = init_res.bob.get_token_balance(None);
    // let pre_claim_balance_carol = init_res.carol.get_token_balance(None);
    // let pre_claim_difference_bob = init_balance_bob - pre_claim_balance_bob;
    // let pre_claim_difference_carol = init_balance_carol - pre_claim_balance_carol;
    // println!("Bob pre-claim balance:    {}", pre_claim_balance_bob);
    // println!("Carol pre-claim balance:  {}", pre_claim_balance_carol);
    // println!(
    //     "Bob has spent {} altogether on staking",
    //     pre_claim_difference_bob
    // );
    // println!(
    //     "Carol has spent {} altogether on staking",
    //     pre_claim_difference_carol
    // );

    // // // finalize
    // println!(
    //     "Request interface balance before claim: {}",
    //     init_res
    //         .alice
    //         .get_token_balance(Some(REQUESTER_CONTRACT_ID.to_string()))
    // );
    // let correct_outcome = Outcome::Answer(AnswerType::String("test".to_string()));
    // init_res
    //     .alice
    //     .dr_final_arbitrator_finalize(0, correct_outcome);
    // let post_outcome = init_res.alice.get_outcome(0);
    // println!("Outcome after finalize: {:?}", post_outcome);

    // // claim
    // let _claim_res = init_res.bob.claim(0);

    // // get balances and differences from after staking/before claiming and before staking
    // let post_balance_alice = init_res.alice.get_token_balance(None);
    // let post_balance_bob = init_res.bob.get_token_balance(None);
    // let post_balance_carol = init_res.carol.get_token_balance(None);
    // let post_stake_difference_bob = post_balance_bob - pre_claim_balance_bob;
    // let post_stake_difference_carol = post_balance_carol - pre_claim_balance_carol;
    // let post_total_difference_bob = post_balance_bob - init_balance_bob;
    // let post_total_difference_carol = init_balance_carol - post_balance_carol;
    // let post_total_difference_alice = init_balance_alice - post_balance_alice;

    // println!("Alice final balance:             {}", post_balance_alice);
    // println!("Bob final balance:               {}", post_balance_bob);
    // println!("Carol final balance:             {}", post_balance_carol);
    // println!(
    //     "Request interface final balance: {}",
    //     init_res
    //         .alice
    //         .get_token_balance(Some(REQUESTER_CONTRACT_ID.to_string()))
    // );

    // println!(
    //     "Bob gained {} from claim for a total profit of {}",
    //     post_stake_difference_bob, post_total_difference_bob
    // );
    // println!(
    //     "Carol gained {} from claim for a total loss of {}",
    //     post_stake_difference_carol, post_total_difference_carol
    // );
    // println!(
    //     "Alice lost {} altogether from validity bond",
    //     post_total_difference_alice
    // );
}

// TODO turn into sim tests

    // #[test]
    // fn user_requests_pair_properly() {
    //     let context = get_context(vec![], false, alice(), 9000000000000000000000);
    //     testing_env!(context);
    //     let mut contract = RequesterContract::new(alice(), token());
    //     contract.create_pair("ETHUSD".to_owned(), 2, U128(400000));
    //     // contract.set_fee(U128(1));
    //     let context = get_context(vec![], false, gustavo(), 0);
    //     testing_env!(context);
    //     let entry = contract.get_entry("ETHUSD".to_owned(), alice());
    //     assert_eq!(entry.price, U128(400000));
    // }

    // sim test, requires different setup
    // #[test]
    // fn provider_sets_fee() {
    //     let context = get_context(vec![], false, alice(), 1690000000000000000000);
    //     testing_env!(context);
    //     let mut contract = RequesterContract::new(alice(), token());
    //     contract.create_pair("ETHUSD".to_owned(), 2, U128(400000));
    //     contract.set_fee(U128(1));
    //     assert_eq!(contract.providers.get(&alice()).unwrap().query_fee, 1);
    //     assert_eq!(contract.providers.get(&alice()).unwrap().balance, 0);
    //     let context = get_context(vec![], false, gustavo(), 1);
    //     testing_env!(context);
    //     let entry = contract.get_entry("ETHUSD".to_owned(), alice());
    //     assert_eq!(entry.price, U128(400000));
    //     assert_eq!(contract.providers.get(&alice()).unwrap().balance, 1);
    // }

        // #[test]
    // fn user_requests_existing_aggregate_avg() {
    //     let context = get_context(vec![], false, alice(), 1690000000000000000000);
    //     testing_env!(context);
    //     let mut contract = RequesterContract::new(alice(), token());
    //     contract.create_pair("ETHUSD".to_owned(), 2, U128(400000));
    //     contract.set_fee(U128(1));

    //     let context = get_context(vec![], false, gustavo(), 1690000000000000000000);
    //     testing_env!(context);
    //     contract.create_pair("ETHUSD".to_owned(), 2, U128(400100));
    //     contract.set_fee(U128(1));
    //     // assert!(contract.get_provider(&gustavo()).pairs.get(&"ETHUSD".to_string()).is_some());

    //     // let entry = contract.aggregate_avg(vec!["ETHUSD".to_owned(),"ETHUSD".to_owned()], vec![alice(), gustavo()], U64(env::block_timestamp()));
    //     // assert_eq!(entry.price, U128(400050))
    // }
// }