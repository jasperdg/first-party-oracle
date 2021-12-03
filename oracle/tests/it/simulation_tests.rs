use crate::utils::*;

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