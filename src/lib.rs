use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{WrappedTimestamp, U128};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize)]
pub struct PriceEntry {
    price: u128,      // Last reported price
    last_update: u64, // Time or report
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Provider {
    pub query_fee: u128,
    pub pairs: LookupMap<String, PriceEntry>, // Maps "{TICKER_1}/{TICKER_2}" => PriceEntry - e.g.: ETHUSD => PriceEntry
}

impl Provider {
    pub fn new() -> Self {
        Self {
            query_fee: 0,
            pairs: LookupMap::new(StorageKeys::Provider),
        }
    }

    pub fn get_entry_expect(&self, pair: &String) -> PriceEntry {
        self.pairs
            .get(pair)
            .expect("no price available for this pair")
    }

    pub fn set_fee(&mut self, fee: u128) {
        self.query_fee = fee
    }

    pub fn set_price(&mut self, pair: String, price: u128) {
        let new_entry = PriceEntry {
            price,
            last_update: env::block_timestamp(),
        };
        self.pairs.insert(&pair, &new_entry);
    }
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKeys {
    Providers,
    Provider,
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
    pub fn new(oracle: AccountId) -> Self {
        Self {
            oracle,
            providers: LookupMap::new(StorageKeys::Providers),
        }
    }

    pub fn push_data(&mut self, pair: String, price: U128) {
        self.providers
            .get(&env::predecessor_account_id())
            .unwrap_or(Provider::new())
            .set_price(pair, price.into());
    }

    pub fn get_entry(
        &self,
        pair: String,
        provider: AccountId,
        min_last_update: WrappedTimestamp,
    ) -> U128 {
        let min_last_update: u64 = min_last_update.into();
        let entry = self.get_provider_expect(&provider).get_entry_expect(&pair);
        assert_eq!(
            entry.last_update >= min_last_update,
            "entry not updated recently enough"
        );
        entry.price.into()
    }

    pub fn aggregate_avg(
        &self,
        pairs: Vec<String>,
        providers: Vec<AccountId>,
        min_last_update: WrappedTimestamp,
    ) -> U128 {
        assert_eq!(
            pairs.len(),
            providers.len(),
            "pairs and provider should be of equal length"
        );
        let min_last_update: u64 = min_last_update.into();
        let mut amount_of_providers = providers.len();

        let cum = pairs.iter().enumerate().fold(0, |s, (i, account_id)| {
            let provider = self.get_provider_expect(&account_id);
            let entry = provider.get_entry_expect(&pairs[i]);

            // If this entry was updated after the min_last_update take it out of the average
            if entry.last_update < min_last_update {
                amount_of_providers -= 1;
                return s;
            } else {
                return s + entry.price;
            }
        });

        U128(cum / amount_of_providers as u128)
    }

    pub fn aggregate_collect(
        &self,
        pairs: Vec<String>,
        providers: Vec<AccountId>,
        min_last_update: WrappedTimestamp,
    ) -> Vec<Option<U128>> {
        assert_eq!(
            pairs.len(),
            providers.len(),
            "pairs and provider should be of equal length"
        );
        let min_last_update: u64 = min_last_update.into();
        pairs
            .iter()
            .enumerate()
            .map(|(i, account_id)| {
                let provider = self
                    .providers
                    .get(&account_id)
                    .expect("no provider with account id");
                let entry = provider.get_entry_expect(&pairs[i]);

                // If this entry was updated after the min_last_update take it out of the average
                if entry.last_update < min_last_update {
                    return None;
                } else {
                    return Some(U128(entry.price));
                }
            })
            .collect()
    }

    fn get_provider_expect(&self, account_id: &AccountId) -> Provider {
        self.providers
            .get(account_id)
            .expect("no provider with this account id")
    }
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
//         // let context = get_context(vec![], false);
//         // testing_env!(context);
//         // let contract = RequesterContract::new(oracle(), token(), None);
//         // contract.request_ft_transfer(token(), 100, alice());
//     }

// }
