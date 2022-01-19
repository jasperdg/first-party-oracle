use crate::utils::*;
use near_sdk::{json_types::U128};



#[test]
fn accounts_initialized_with_balance() {
    let init_res = TestUtils::init();
    assert_eq!(init_res.jack.get_token_balance(None),
        INIT_BALANCE,
        "token storage requirement not satisfied for account");
    assert_eq!(init_res.alice.get_token_balance(None),
        INIT_BALANCE,
        "token storage requirement not satisfied for account");
    assert_eq!(init_res.bob.get_token_balance(None),
        INIT_BALANCE,
        "token storage requirement not satisfied for account");
}

#[test]
fn provider_sends_data_and_requester_recieves_it() {
    let init_res = TestUtils::init();

    init_res.jack.push_data("ETHUSD".to_owned(), U128(4000));
    // init_res.alice.get_entry("ETHUSD".to_owned(), init_res.jack.account.account_id);
}