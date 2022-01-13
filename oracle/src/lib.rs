use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::json_types::{U128, WrappedTimestamp};
use near_sdk::{serde_json, Promise, env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Balance};
near_sdk::setup_alloc!();
use storage_manager::AccountStorageBalance;
use fungible_token::fungible_token_transfer;

// TODO replace all fungible token logic with built in standards implementation
// near_contract_standards::impl_fungible_token_core!(FirstPartyOracle, token, on_tokens_burned);

mod helpers;
mod storage_manager;
mod fungible_token;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Copy, Clone)]
pub struct PriceEntry {
    price: u128,      // Last reported price
    decimals: u32,    // Amount of decimals (e.g. if 2, 100 = 1.00)
    last_update: u64, // Time of report
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Outcome {
    entries: Option<Vec<PriceEntry>>,
    refund: Balance
}

// TODO is this stupid to have for just the aggregate_collect
// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
// pub struct Outcomes {
//     entries: Option<Vec<PriceEntry>>,
//     refund: Balance
// }

// #[derive(Serialize, Deserialize)]
// pub enum OutcomePayload {
//     Outcome(Outcome),
//     Outcomes(Outcomes),
//     None
// }

#[derive(Serialize, Deserialize)]
pub struct RequestPayload {
    method: String,
    pairs: Vec<String>,
    providers: Vec<AccountId>,
    min_last_update: WrappedTimestamp,
}

// TODO perhaps this is better?
// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
// pub struct Outcomes {
//     outcomes: Vec<Outcome>,
//     aggregated_refund: Balance
// }

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Provider {
    pub query_fee: u128,
    pub pairs: LookupMap<String, PriceEntry>, // Maps "{TICKER_1}/{TICKER_2}" => PriceEntry - e.g.: ETHUSD => PriceEntry
    pub earnings: Balance,
    pub balance: AccountStorageBalance
}

impl Provider {
    pub fn new() -> Self {
        Self {
            query_fee: 0,
            pairs: LookupMap::new(StorageKeys::Provider),
            earnings: 0,
            balance: AccountStorageBalance {
                total: 0,
                available: 0
            }
        }
    }
    // pub fn get_pairs(&self) -> LookupMap<String, PriceEntry> {
    //     self.pairs
    // }
    pub fn get_entry(&mut self, pair: &String) -> Option<PriceEntry> {
        self.pairs.get(pair)
    }
    pub fn get_earnings(&self) -> Balance {
        self.earnings
    }
    pub fn get_balance(&self) -> AccountStorageBalance {
        self.balance
    }
    pub fn set_balance(&mut self, balance: AccountStorageBalance) {
        self.balance = balance
    }
    pub fn set_fee(&mut self, fee: u128) {
        self.query_fee = fee
    }
    pub fn get_fee(&mut self) -> u128 {
        self.query_fee
    }
    pub fn set_price(&mut self, pair: String, price: u128, decimals: u32) {
        let entry = PriceEntry {
                price: price,      // Last reported price
                decimals: decimals,    // Amount of decimals (e.g. if 2, 100 = 1.00)
                last_update: env::block_timestamp().into(),
        };
        self.pairs.insert(&pair, &entry);
    }
    fn add_earnings(&mut self, amount: u128) {
        self.earnings += amount;
    }
    fn withdraw_earnings(&mut self, amount: Option<u128>) -> u128 {
        match amount {
            Some(i) => {
                assert!(self.earnings >= i,
                    "Not enough storage available");
                self.earnings -= i;
                i
            },
            None => {
                let withdrawal = self.earnings;
                self.earnings = 0;
                withdrawal
            }
        }
    }
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKeys {
    Providers,
    Provider,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct FirstPartyOracle {
    // pub oracle: AccountId,
    pub payment_token: AccountId,
    pub providers: LookupMap<AccountId, Provider>, // maps:  AccountId => Provider
}

// Private methods
impl FirstPartyOracle {
    // TODO change assertions to boolean conditionals to allow function to finish 
    fn provider_pair_exists(
            &self, 
            pair: &String, 
            provider: &AccountId) -> bool {
        self.providers.get(&provider)
            .unwrap()
            .pairs
            .get(&pair)
            .is_some()
    }
    fn pair_updated(
            &self, 
            pair: &String, 
            provider: &AccountId, 
            min_last_update: u64) -> bool {
        let pair_update: u64 = self.providers.get(provider)
            .unwrap()
            .pairs
            .get(pair)
            .unwrap()
            .last_update;
        min_last_update < pair_update
    }
    pub fn payment_sufficient(
            &self, 
            provider: &AccountId, 
            amount: u128) -> bool {
        let fee = self.providers
            .get(provider)
            .unwrap()
            .get_fee();
        amount >= fee
    }
    fn same_length(
            &self, 
            pairs: &Vec<String>, 
            providers: &Vec<AccountId>) -> bool {
        pairs.len() == providers.len() && pairs.len() > 0
    }
    fn add_earnings(&mut self, provider: &AccountId, amount: u128) {
        self.providers.get(provider).unwrap().add_earnings(amount);
    }
    fn withdraw_earnings(&mut self, provider: AccountId) -> u128 {
        self.providers.get(&provider).unwrap().withdraw_earnings(None)
    }
    fn run_method(&mut self, amount: u128, msg: String) -> Outcome {
        let payload: RequestPayload = serde_json::from_str(&msg).expect("Failed to parse the payload, invalid `msg` format");
        let min_last_update = u64::from(payload.min_last_update);
        match payload.method.as_ref() {
            "get_entry" => {
                self.get_entry(&payload.pairs[0], &payload.providers[0], min_last_update, amount)
            },
            "aggregate_avg" => {
                self.aggregate_avg(payload.pairs, payload.providers, min_last_update, amount)
            },
            "aggregate_collect" => {
                self.aggregate_collect(payload.pairs, payload.providers, min_last_update, amount)
            }
            &_ => {
                println!("how tf did you end up here");
                Outcome{
                    entries: None,
                    refund: 0
                }
            }
        }
    }
    fn run_checks(&self,
            pair: &String, 
            provider: &AccountId, 
            min_last_update: u64, 
            amount_left: u128
        ) -> bool {

        if !self.provider_pair_exists(pair, provider) {
            return false
        }
        if !self.payment_sufficient(provider, amount_left) {
            return false
        }
        if !self.pair_updated(pair, provider, min_last_update) {
            return false
        }
        true
    }
    fn get_entry(
            &mut self, 
            pair: &String, 
            provider: &AccountId, 
            min_last_update: u64, 
            amount_left: u128
        ) -> Outcome {
        if self.run_checks(pair, provider, min_last_update, amount_left) {
            let entry = self.providers.get(provider).unwrap().get_entry(pair).unwrap();
            let fee = self.providers.get(&provider).unwrap().get_fee();
            self.add_earnings(provider, fee);
            Outcome {
                entries: Some(vec![entry]),
                refund: amount_left - fee
            }
        } else {
            Outcome {
                entries: None,
                refund: amount_left
            }
        }
    }
    pub fn aggregate_avg(
            &mut self,
            pairs: Vec<String>,
            providers: Vec<AccountId>,
            min_last_update: u64,
            mut amount_left: u128
        ) -> Outcome {
        if !self.same_length(&pairs, &providers) {
            return Outcome {
                entries: None,
                refund: amount_left
            }
        }
        let min_last_update: u64 = min_last_update.into();
        let mut cum = 0.0_f64;
        for i in 0..pairs.len() {
            let outcome: Outcome = self.get_entry(&pairs[i], &providers[i], min_last_update, amount_left);
            if let Some(e) = outcome.entries {
                let price_decimals = u128::from(e[0].price) as f64 / 10_i32.pow(e[0].decimals) as f64;
                cum += price_decimals;
                amount_left = outcome.refund;
            }
        }
        let avg = cum / providers.len() as f64;
        // TODO uses number of decimals from aggregation for decimals in answer
        let decimals = helpers::precision(avg).unwrap_or(0);
        Outcome {
            entries: Some(vec![PriceEntry {
                price: (avg * 10_i32.pow(decimals) as f64) as u128,
                decimals: decimals,
                last_update: min_last_update
            }]),
            refund: amount_left
        }
    }

    pub fn aggregate_collect(
            &mut self,
            pairs: Vec<String>,
            providers: Vec<AccountId>,
            min_last_update: u64,
            mut amount_left: u128
        ) -> Outcome {
        if !self.same_length(&pairs, &providers) {
            return Outcome {
                entries: None,
                refund: amount_left
            }
        }
        let mut entries: Vec<PriceEntry> = vec![];
        for i in 0..pairs.len() {
            let outcome: Outcome = self.get_entry(&pairs[i], &providers[i], min_last_update, amount_left);
            if let Some(e) = outcome.entries {
                entries.push(e[0]);
                amount_left = outcome.refund
            }
        }
        Outcome {
            entries: Some(entries),
            refund: amount_left
        }
    }
}

#[near_bindgen]
impl FirstPartyOracle {
    #[init]
    pub fn new(
        payment_token: AccountId) -> Self {
        Self {
            payment_token,
            providers: LookupMap::new(StorageKeys::Providers),
        }
    }

    /********* PROVIDER METHODS *********/

    // #[payable]
    // pub fn create_pair(&mut self, pair: String, decimals: u32, initial_price: U128) {
    //     let initial_storage_usage = env::storage_usage();
    //     let mut provider = self
    //         .providers
    //         .get(&env::predecessor_account_id())
    //         .unwrap_or(Provider::new());
            
    //     // TODO test whether this actually creates the new provider 
    //     assert!(provider.pairs.get(&pair).is_none(), "pair already exists");

    //     provider.pairs.insert(
    //         &pair,
    //         &PriceEntry {
    //             price: u128::from(initial_price),
    //             decimals,
    //             last_update: env::block_timestamp().into(),
    //         },
    //     );

    //     self.providers
    //         .insert(&env::predecessor_account_id(), &provider);
    //     helpers::refund_storage(initial_storage_usage, env::predecessor_account_id());
    // }
    
    pub fn claim_earnings(&mut self) -> Promise {
        fungible_token_transfer(self.payment_token.clone(), 
            env::predecessor_account_id(), 
            self.withdraw_earnings(env::predecessor_account_id()))
    }

    #[payable]
    pub fn push_data(&mut self, pair: String, price: U128, decimals: u32) {
        let initial_storage_usage = env::storage_usage();
        let mut provider = self.providers.get(&env::predecessor_account_id()).unwrap_or(Provider::new());
        provider.set_price(pair, u128::from(price), decimals);
        self.providers.insert(&env::predecessor_account_id(), &provider);
        helpers::refund_storage(initial_storage_usage, env::predecessor_account_id());
    }

    pub fn set_fee(&mut self, fee: U128) {
        let mut provider = self.providers.get(&env::predecessor_account_id()).unwrap();
        provider.set_fee(u128::from(fee));
        self.providers.insert(&env::predecessor_account_id(), &provider);
    }

    /********* REQUESTER METHODS *********/

    pub fn get_fee_total(&self, pairs: &Vec<String>, providers: &Vec<AccountId>) -> u128 {
        assert!(self.same_length(pairs, providers), "must provide same number of pairs and providers to get fees for");
        let mut fee_total: u128 = 0;
        for i in 0..pairs.len() {
            if self.provider_pair_exists(&pairs[i], &providers[i]) {
                fee_total += self.providers.get(&providers[i]).unwrap().get_fee();
            }
        }
        fee_total
    }

    pub fn get_provider_pairs(&self, account_id: &AccountId) -> LookupMap<String, PriceEntry> {
        self.providers
            .get(account_id)
            .unwrap()
            .pairs
    }

    pub fn get_provider_exists(&self, account_id: &AccountId) -> bool {
        self.providers
            .get(account_id)
            .is_some()
    }

    pub fn get_pair_exists(&self, pair: String, provider: AccountId) -> bool {
        match self.get_provider_exists(&provider) {
            false => false,
            true => self.providers.get(&provider).unwrap().pairs.get(&pair).is_some()
        }
    }
}