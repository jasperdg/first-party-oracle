pub use oracle::*;
use near_sdk::{AccountId};

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use flux_sdk::DataRequestDataType;
    use near_sdk::json_types::U128;
    use near_sdk::serde_json;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn alice() -> AccountId {
        "alice.near".to_string()
    }

    fn gustavo() -> AccountId {
        "gustavo.near".to_string()
    }

    fn oracle() -> AccountId {
        "oracle.near".to_string()
    }

    fn token() -> AccountId {
        "token.near".to_string()
    }

    fn get_context(input: Vec<u8>, is_view: bool, user: AccountId, deposit: u128) -> VMContext {
        VMContext {
            current_account_id: alice(),
            signer_account_id: alice(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: user,
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 10000 * 10u128.pow(24),
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: deposit,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    // #[test]
    // fn oracle_created() {
    //     let context = get_context(vec![], false, alice(), 1690000000000000000000);
    //     testing_env!(context);
    //     let contract = FirstPartyOracle::new(alice(), token());
    // }

    // #[test]
    // #[should_panic(expected = "no provider with this account id")]
    // fn provider_requested_before_creation() {
    //     let context = get_context(vec![], false, alice(), 1690000000000000000000);
    //     testing_env!(context);
    //     let mut contract = FirstPartyOracle::new(alice(), token());
    //     contract.get_entry("ETHUSD".to_owned(), alice());
    // }

    // #[test]
    // #[should_panic(expected = "BTCUSD doesn't exist for alice.near")]
    // fn pair_requested_before_creation() {
    //     let context = get_context(vec![], false, alice(), 1690000000000000000000);
    //     testing_env!(context);
    //     let mut contract = FirstPartyOracle::new(alice(), token());
    //     contract.create_pair("ETHUSD".to_owned(), 2, U128(400000));
    //     contract.get_entry("BTCUSD".to_owned(), alice());
    // }

    // #[test]
    // fn provider_pair_created() {
    //     let context = get_context(vec![], false, alice(), 1690000000000000000000);
    //     testing_env!(context);
    //     let mut contract = FirstPartyOracle::new(alice(), token());
    //     contract.create_pair("ETHUSD".to_owned(), 2, U128(400000));
    //     contract.assert_provider_exists(alice());
    //     contract.get_pair_exists("ETHUSD".to_owned(), alice());
    // }

    // #[test]
    // #[should_panic(expected = "no provider with this account id")]
    // fn user_requests_existing_pair_from_other_provider() {
    //     let context = get_context(vec![], false, alice(), 1690000000000000000000);
    //     testing_env!(context);
    //     let mut contract = FirstPartyOracle::new(alice(), token());
    //     contract.create_pair("ETHUSD".to_owned(), 2, U128(400000));
    //     contract.set_fee(U128(1));
    //     let context = get_context(vec![], false, gustavo(), 1);
    //     testing_env!(context);
    //     let entry = contract.get_entry("ETHUSD".to_owned(), gustavo());
    // }
}