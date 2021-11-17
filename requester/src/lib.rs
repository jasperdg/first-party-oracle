use flux_sdk::{
    NewDataRequestArgs, Outcome, AnswerType, DataRequestDataType, 
    AnswerNumberType
};
use near_sdk::serde_json::json;
use near_sdk::json_types::{U64, U128, WrappedBalance};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, BorshStorageKey, Promise};

mod oracle_handler;

near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize)]
pub struct PriceEntry {
    price: u128, // Last reported price
    last_update: u64 // Time or report
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Provider {
    pub query_fee: u128,
    pub pairs: LookupMap<String, PriceEntry> // Maps {TICKER_1}+{TICKER_2} => PriceEntry - e.g.: ETHUSD => PriceEntry
}

impl Provider {
    pub fn new() -> Self {
        Self {
            query_fee: 0,
            pairs: LookupMap::new(format!("p:{:?}", env::predecessor_account_id()))
        }
    }

    pub fn set_fee(&mut self, fee: u128) {
        self.query_fee = fee
    }

    pub fn set_price(&mut self, pair: String, price: u128) {
        let new_entry = PriceEntry { price, last_update: env::block_timestamp() };
        self.pairs.insert(&pair, &new_entry);
    }
    
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKeys {
    Providers
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct RequesterContract {
    pub oracle: AccountId,
    pub providers: LookupMap<AccountId, Provider>, // maps:  AccountId => Provider
}

// Private methods
impl RequesterContract {
    pub fn assert_oracle(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.oracle,
            "ERR_INVALID_ORACLE_ADDRESS"
        );
    }
}

#[near_bindgen]
impl RequesterContract {
    #[init]
    pub fn new(
        oracle: AccountId,
    ) -> Self {
        Self {
            oracle,
            providers: LookupMap::new(StorageKeys::Providers)
        }
    }

    pub fn push_data(
        &mut self,
        pair: String,
        price: WrappedBalance,
    ) -> Promise {
        let mut tags = Vec::new();

        let provider = self.providers
            .get(&env::predecessor_account_id())
            .unwrap_or(Provider::new())
            .set_price(pair, price.into());

        let json_string = json!({
            "outcome": into_outcome(price)
        }).to_string();
        tags.push(json_string);

        self.dr_new(
            NewDataRequestArgs {
                sources: None,
                tags,
                description: None,
                outcomes: None,
                challenge_period: U64(0),
                data_type: DataRequestDataType::String,
                provider: Some(env::predecessor_account_id()),
            }
        )
    }
}

fn into_outcome(value: WrappedBalance) -> Outcome {
    Outcome::Answer(AnswerType::Number(AnswerNumberType {value, multiplier: U128(1), negative: false}))
}

// #[cfg(not(target_arch = "wasm32"))]
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use flux_sdk::DataRequestDataType;
//     use near_sdk::json_types::U128;
//     use near_sdk::serde_json;
//     use near_sdk::MockedBlockchain;
//     use near_sdk::{testing_env, VMContext};

//     fn alice() -> AccountId {
//         "alice.near".to_string()
//     }

//     fn oracle() -> AccountId {
//         "oracle.near".to_string()
//     }

//     fn token() -> AccountId {
//         "token.near".to_string()
//     }

//     fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
//         VMContext {
//             current_account_id: alice(),
//             signer_account_id: alice(),
//             signer_account_pk: vec![0, 1, 2],
//             predecessor_account_id: alice(),
//             input,
//             block_index: 0,
//             block_timestamp: 0,
//             account_balance: 10000 * 10u128.pow(24),
//             account_locked_balance: 0,
//             storage_usage: 0,
//             attached_deposit: 0,
//             prepaid_gas: 10u64.pow(18),
//             random_seed: vec![0, 1, 2],
//             is_view,
//             output_data_receivers: vec![],
//             epoch_height: 0,
//         }
//     }

//     #[test]
//     #[should_panic(expected = "ERR_INVALID_ORACLE_ADDRESS")]
//     fn ri_not_oracle() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let contract = RequesterContract::new(oracle(), token(), None);
//         contract.request_ft_transfer(token(), 100, alice());
//     }

//     #[test]
//     fn ri_create_dr_success() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = RequesterContract::new(oracle(), token(), None);

//         contract.create_data_request(
//             U128(100),
//             NewDataRequestArgs {
//                 sources: Some(Vec::new()),
//                 outcomes: Some(vec!["a".to_string()].to_vec()),
//                 challenge_period: U64(1500),
//                 description: Some("a".to_string()),
//                 tags: Vec::new(),
//                 data_type: DataRequestDataType::String,
//             },
//         );
//     }

//     #[test]
//     fn ri_whitelisted_success() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = RequesterContract::new(
//             oracle(),
//             token(),
//             Some(vec![serde_json::from_str("\"alice.near\"").unwrap()]),
//         );

//         contract.create_data_request(
//             U128(100),
//             NewDataRequestArgs {
//                 sources: Some(Vec::new()),
//                 outcomes: Some(vec!["a".to_string()].to_vec()),
//                 challenge_period: U64(1500),
//                 description: Some("a".to_string()),
//                 tags: Vec::new(),
//                 data_type: DataRequestDataType::String,
//             },
//         );
//     }

//     #[test]
//     #[should_panic(expected = "ERR_NOT_WHITELISTED")]
//     fn ri_unwhitelisted_fail() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = RequesterContract::new(
//             oracle(),
//             token(),
//             Some(vec![serde_json::from_str("\"bob.near\"").unwrap()]),
//         );

//         contract.create_data_request(
//             U128(100),
//             NewDataRequestArgs {
//                 sources: Some(Vec::new()),
//                 outcomes: Some(vec!["a".to_string()].to_vec()),
//                 challenge_period: U64(1500),
//                 description: Some("a".to_string()),
//                 tags: Vec::new(),
//                 data_type: DataRequestDataType::String,
//             },
//         );
//     }

//     #[test]
//     fn ri_empty_tags_nonce_works() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = RequesterContract::new(
//             oracle(),
//             token(),
//             Some(vec![serde_json::from_str("\"alice.near\"").unwrap()]),
//         );

//         contract.create_data_request(
//             U128(100),
//             NewDataRequestArgs {
//                 sources: Some(Vec::new()),
//                 outcomes: Some(vec!["a".to_string()].to_vec()),
//                 challenge_period: U64(1500),
//                 description: Some("a".to_string()),
//                 tags: Vec::new(),
//                 data_type: DataRequestDataType::String,
//             },
//         );

//         assert!(contract.data_requests.get(&0).is_some());
//     }

//     #[test]
//     fn ri_some_tags_nonce_works() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = RequesterContract::new(
//             oracle(),
//             token(),
//             Some(vec![serde_json::from_str("\"alice.near\"").unwrap()]),
//         );

//         contract.create_data_request(
//             U128(100),
//             NewDataRequestArgs {
//                 sources: Some(Vec::new()),
//                 outcomes: Some(vec!["a".to_string()].to_vec()),
//                 challenge_period: U64(1500),
//                 description: Some("a".to_string()),
//                 tags: vec!["butt".to_owned(), "on".to_owned()],
//                 data_type: DataRequestDataType::String,
//             },
//         );

//         assert!(contract.data_requests.get(&0).is_some());
//     }

//     #[test]
//     fn ri_nonce_iterates_properly() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = RequesterContract::new(
//             oracle(),
//             token(),
//             Some(vec![serde_json::from_str("\"alice.near\"").unwrap()]),
//         );

//         contract.create_data_request(
//             U128(100),
//             NewDataRequestArgs {
//                 sources: Some(Vec::new()),
//                 outcomes: Some(vec!["a".to_string()].to_vec()),
//                 challenge_period: U64(1500),
//                 description: Some("a".to_string()),
//                 tags: Vec::new(),
//                 data_type: DataRequestDataType::String,
//             },
//         );

//         contract.create_data_request(
//             U128(100),
//             NewDataRequestArgs {
//                 sources: Some(Vec::new()),
//                 outcomes: Some(vec!["a".to_string()].to_vec()),
//                 challenge_period: U64(1500),
//                 description: Some("a".to_string()),
//                 tags: Vec::new(),
//                 data_type: DataRequestDataType::String,
//             },
//         );

//         assert!(contract.data_requests.get(&1).is_some());
//     }
// }
