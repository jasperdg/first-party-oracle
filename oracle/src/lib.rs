mod helpers;
use fungible_token_handler::{fungible_token_transfer};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::json_types::{WrappedTimestamp, U128, U64};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise};
near_sdk::setup_alloc!();

mod fungible_token_handler;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct PriceEntry {
    price: U128,                   // Last reported price
    decimals: u16,                 // Amount of decimals (e.g. if 2, 100 = 1.00)
    last_update: WrappedTimestamp, // Time or report
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Provider {
    pub query_fee: u128,
    pub pairs: LookupMap<String, PriceEntry>, // Maps "{TICKER_1}/{TICKER_2}" => PriceEntry - e.g.: ETHUSD => PriceEntry
    pub balance: u128
}

impl Provider {
    pub fn new() -> Self {
        Self {
            query_fee: 0,
            pairs: LookupMap::new(StorageKeys::Provider),
            balance: 0
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

    pub fn set_price(&mut self, pair: String, price: U128) {
        let mut entry = self.pairs.get(&pair).expect("pair does not exist yet");
        entry.last_update = env::block_timestamp().into();
        entry.price = price;

        self.pairs.insert(&pair, &entry);
    }
    fn add_balance(&mut self, amount: u128) {
        self.balance = self.balance + amount;
    }
    fn withdraw_balance(&mut self) -> u128 {
        let withdrawal = self.balance;
        self.balance = 0;
        withdrawal
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
    pub payment_token: AccountId,
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
    pub fn assert_provider_exists(&self, provider: AccountId) {
        assert!(self.providers.get(&provider).is_some());
    }
    fn get_provider(&self, account_id: &AccountId) -> Provider {
        self.providers
            .get(account_id)
            .expect("no provider with this account id")
    }
    pub fn assert_paying_price(&self, providers: Vec<AccountId>, amount: u128) {
        let mut balance_left = amount;
        for provider in providers.iter() {
            assert!(
                balance_left >= self.get_provider(&provider).query_fee,
                "Not enough deposit for this query, {} needs {} when {} left",
                provider,
                self.get_provider(&provider).query_fee,
                balance_left
            );
            balance_left -= amount;
        }
    }
    // TODO: make a query to take all providers and tell user how much deposit is required to make query
    pub fn assert_pair_exists(&self, provider: AccountId, pair: String) {
        assert!(
            self.get_provider(&provider)
            .pairs
            .get(&pair)
            .is_some(),
            "{} doesn't exist for {}",
            pair,
            provider
        );
    }
    // fn assert_has_balance(&self, provider: AccountId) {
    //     assert!(self.providers.get(&provider).unwrap().balance > 0, "you don't have any money");
    // }
    fn add_balance(&mut self, provider: &AccountId, amount: u128) -> u128 {
        let excess = amount - self.providers.get(provider).unwrap().query_fee;
        self.providers.get(provider).unwrap().add_balance(amount);
        excess
    }
    fn withdraw_balance(&mut self, provider: AccountId) -> u128 {
        self.providers.get(&provider).unwrap().withdraw_balance()
    }
}

#[near_bindgen]
impl RequesterContract {
    #[init]
    pub fn new(oracle: AccountId, payment_token: AccountId) -> Self {
        Self {
            oracle,
            payment_token,
            providers: LookupMap::new(StorageKeys::Providers),
        }
    }

    #[payable]
    pub fn create_pair(&mut self, pair: String, decimals: u16, initial_price: U128) {
        let initial_storage_usage = env::storage_usage();
        // TODO see if user and pair exists
        // create provider if doesn't exist
        // create pair if doesn't exist
        let mut provider = self
            .providers
            .get(&env::predecessor_account_id())
            .unwrap_or(Provider::new());
            
        // TODO test whether this actually creates the new provider 
        // assert!(provider.pairs.get(&pair).is_none(), "pair already exists");

        provider.pairs.insert(
            &pair,
            &PriceEntry {
                price: initial_price,
                decimals,
                last_update: env::block_timestamp().into(),
            },
        );

        self.providers
            .insert(&env::predecessor_account_id(), &provider);

        helpers::refund_storage(initial_storage_usage, env::predecessor_account_id());
    }

    fn get_provider_exists(&self, account_id: &AccountId) -> bool {
        self.providers.get(account_id).is_some()
    }

    pub fn get_pair_exists(&self, pair: String, provider: AccountId) -> bool {
        self.get_provider(&provider)
            .pairs
            .get(&pair)
            .is_some()
    }

    #[payable]
    pub fn push_data(&mut self, pair: String, price: U128) {
        let initial_storage_usage = env::storage_usage();
        assert!(self.get_provider_exists(&env::predecessor_account_id()));
        let mut provider = self.providers.get(&env::predecessor_account_id()).unwrap();
        provider.set_price(pair, price);
        self.providers
            .insert(&env::predecessor_account_id(), &provider);
        
        helpers::refund_storage(initial_storage_usage, env::predecessor_account_id());
    }

    // TODO may need to implement promiseorvalue chain: get entry -> add_balance -> return data
    #[payable]
    pub fn get_entry(&mut self, pair: String, provider: AccountId) -> PriceEntry {
        self.assert_pair_exists(provider.clone(), pair.clone());
        self.assert_paying_price(vec![provider.clone()], env::attached_deposit());
        self.add_balance(&provider, env::attached_deposit());
        // let excess = self.add_balance(&provider, env::attached_deposit());
        // if excess > 0 {
            
        // }
        self.providers.get(&provider).unwrap().get_entry_expect(&pair)
        // TODO make sure provider pair has RECENT data in it 
    }

    pub fn claim_earnings(&mut self) -> Promise {
        fungible_token_transfer(self.payment_token.clone(), env::predecessor_account_id(), self.withdraw_balance(env::predecessor_account_id()))
    }

    pub fn set_fee(&mut self, fee: U128) {
        let mut provider = self.providers.get(&env::predecessor_account_id()).unwrap();
        provider.set_fee(u128::from(fee));
        self.providers.insert(&env::predecessor_account_id(), &provider);
    }

    // pay for queries
    #[payable]
    pub fn aggregate_avg(
        &mut self,
        pairs: Vec<String>,
        providers: Vec<AccountId>,
        min_last_update: WrappedTimestamp,
    ) -> PriceEntry {
        
        // TODO: check if all pairs exist, and if paid amount enough to cover all providers,
        //          or tell user how much they need to send, and return money
        // add balance to each provider
        // perform aggregation and return value

        assert_eq!(
            pairs.len(),
            providers.len(),
            "pairs and provider should be of equal length"
        );
        self.assert_paying_price(providers.clone(), env::attached_deposit());
        let min_last_update: u64 = min_last_update.into();
        let mut amount_of_providers = providers.len();

        let cum = pairs.iter().enumerate().fold(0, |s, (i, account_id)| {
            let provider = self.get_provider(&account_id);
            let entry = provider.get_entry_expect(&pairs[i]);
            self.add_balance(account_id, self.get_provider(&account_id).query_fee);
            // TODO return fee if last_update not recent enough
            // If this entry was updated after the min_last_update take it out of the average
            if u64::from(entry.last_update) < min_last_update {
                amount_of_providers -= 1;
                return s;
            } else {
                return s + u128::from(entry.price) / (10 * u128::from(entry.decimals));
            }
        });
        PriceEntry {
            price: U128(cum / amount_of_providers as u128),
            decimals: 0,
            last_update: U64(min_last_update)
        }
    }

    // pay for queries
    #[payable]
    pub fn aggregate_collect(
        &mut self,
        pairs: Vec<String>,
        providers: Vec<AccountId>,
        min_last_update: WrappedTimestamp,
    ) -> Vec<Option<PriceEntry>> {

        // TODO: check if all pairs exist, and if paid amount enough to cover all providers,
        //          or tell user how much they need to send, and return money
        // add balance to each provider
        // perform aggregation and return value

        assert_eq!(
            pairs.len(),
            providers.len(),
            "pairs and provider should be of equal length"
        );
        self.assert_paying_price(providers.clone(), env::attached_deposit());
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
                self.add_balance(account_id, self.get_provider(&account_id).query_fee);
                // TODO allow user to insert more deposit and have them claim it later
                // If this entry was updated after the min_last_update take it out of the average
                if u64::from(entry.last_update) < min_last_update {
                    return None;
                } else {
                    return Some(entry);
                }
            })
            .collect()
    }
}