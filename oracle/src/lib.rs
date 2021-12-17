// use fungible_token_handler::{fungible_token_transfer};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::json_types::{U128};
use near_sdk::{Promise, env, near_bindgen, ext_contract, AccountId, BorshStorageKey, PanicOnDefault, Balance};
use flux_sdk::{ types::WrappedTimestamp };
near_sdk::setup_alloc!();

// TODO replace all fungible token logic with built in standards implementation
// near_contract_standards::impl_fungible_token_core!(FirstPartyOracle, token, on_tokens_burned);

mod helpers;
// mod fungible_token_handler;
mod storage_manager;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct PriceEntry {
    price: u128,                   // Last reported price
    decimals: u32,                 // Amount of decimals (e.g. if 2, 100 = 1.00)
    last_update: u64, // Time or report
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    pub query_fee: u128,
    pub pairs: LookupMap<String, PriceEntry>, // Maps "{TICKER_1}/{TICKER_2}" => PriceEntry - e.g.: ETHUSD => PriceEntry
    pub balance: Balance
}

impl User {
    pub fn new() -> Self {
        Self {
            query_fee: 0,
            pairs: LookupMap::new(StorageKeys::User),
            balance: 0
        }
    }

    pub fn get_entry(&self, pair: &String, amount: u128) -> Option<PriceEntry> {
        let entry = self.pairs
            .get(pair);
        self.add_balance(amount);
        entry
    }
    pub fn get_balance(&self) -> Balance {
        self.balance
    }
    pub fn set_fee(&mut self, fee: u128) {
        self.query_fee = fee
    }
    pub fn get_fee(&mut self) -> u128 {
        self.query_fee
    }
    pub fn set_price(&mut self, pair: String, price: u128) {
        let mut entry = self.pairs.get(&pair).expect("pair does not exist yet");
        entry.last_update = env::block_timestamp().into();
        entry.price = price;

        self.pairs.insert(&pair, &entry);
    }
    fn add_balance(&mut self, amount: u128) {
        self.balance += amount;
    }
    fn withdraw_balance(&mut self, amount: Option<u128>) -> u128 {
        match amount {
            Some(i) => {
                assert!(self.balance >= i,
                    "Not enough storage available");
                self.balance -= i;
                i
            },
            None => {
                let withdrawal = self.balance;
                self.balance = 0;
                withdrawal
            }
        }
    }
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKeys {
    Users,
    User,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct FirstPartyOracle {
    pub oracle: AccountId,
    pub payment_token: AccountId,
    pub users: LookupMap<AccountId, User>, // maps:  AccountId => User
}

// Private methods
impl FirstPartyOracle {
    // pub fn assert_oracle(&self) {
    //     assert_eq!(
    //         &env::predecessor_account_id(),
    //         &self.oracle,
    //         "ERR_INVALID_ORACLE_ADDRESS"
    //     );
    // }
    pub fn assert_user_exists(&self, user: AccountId) {
        assert!(self.users.get(&user).is_some());
    }
    fn get_user(&self, account_id: &AccountId) -> User {
        self.users
            .get(account_id)
            .expect("no user with this account id")
    }
    fn assert_same_length(&self, pairs: Vec<String>, users: Vec<AccountId>) {
        assert_eq!(
            pairs.len(),
            users.len(),
            "pairs and user should be of equal length"
        );
    }
    // pub fn assert_paying_price(&self, users: Vec<AccountId>, amount: u128) {
    //     let mut balance_left = amount;
    //     for user in users.iter() {
    //         assert!(
    //             balance_left >= self.get_user(&user).query_fee,
    //             "Not enough deposit for this query, {} needs {} when {} left",
    //             user,
    //             self.get_user(&user).query_fee,
    //             balance_left
    //         );
    //         balance_left -= amount;
    //     }
    // }
    fn assert_user_pair_exists(&self, pair: String, user: AccountId) {
        assert!(
            self.get_user(&user)
            .pairs
            .get(&pair)
            .is_some(),
            "{} doesn't exist for {}",
            pair,
            user
        );
    }
    // asserts that provider and pairs exist for users query,
    // and that user has provided enough tokens to pay for data
    pub fn assert_pairs_exist_and_payment_sufficient_and_recent_enough(
            &self, 
            pairs: Vec<String>, 
            users: Vec<AccountId>, 
            min_last_update: u64,
            amount: u128) {
        self.assert_same_length(pairs, users);
        let fees = self.get_fee_total(pairs, users);
        assert!(
            amount >= fees,
            "Not enough deposit for this query, {} required when {} provided",
            fees,
            amount
        );
        for i in 0..users.len() {
            assert!(min_last_update < self.get_user(&users[i]).pairs.get(&pairs[i]).unwrap().last_update,
                "")
        }
    }
    // fn assert_has_balance(&self, user: AccountId) {
    //     assert!(self.users.get(&user).unwrap().balance > 0, "you don't have any money");
    // }
    fn add_balance(&mut self, user: &AccountId, amount: u128) {
        self.users.get(user).unwrap().add_balance(amount);
    }
    fn withdraw_balance(&mut self, user: AccountId) -> u128 {
        self.users.get(&user).unwrap().withdraw_balance(None)
    }
}

#[near_bindgen]
impl FirstPartyOracle {
    #[init]
    pub fn new(oracle: AccountId, payment_token: AccountId) -> Self {
        Self {
            oracle,
            payment_token,
            users: LookupMap::new(StorageKeys::Users),
        }
    }
    #[payable]
    pub fn create_pair(&mut self, pair: String, decimals: u32, initial_price: U128) {
        let initial_storage_usage = env::storage_usage();
        let mut user = self
            .users
            .get(&env::predecessor_account_id())
            .unwrap_or(User::new());
            
        // TODO test whether this actually creates the new user 
        assert!(user.pairs.get(&pair).is_none(), "pair already exists");

        user.pairs.insert(
            &pair,
            &PriceEntry {
                price: u128::from(initial_price),
                decimals,
                last_update: env::block_timestamp().into(),
            },
        );

        self.users
            .insert(&env::predecessor_account_id(), &user);
        helpers::refund_storage(initial_storage_usage, env::predecessor_account_id());
    }
    pub fn get_balance(&self, account_id: AccountId) -> Balance {
        self.assert_user_exists(account_id.clone());
        let user = self.users.get(&account_id).unwrap();
        user.get_balance()
    }
    pub fn get_fee_total(&self, pairs: Vec<String>, providers: Vec<AccountId>) -> u128 {
        self.assert_same_length(pairs, providers);
        let mut fee_total: u128 = 0;
        for i in 0..pairs.len() {
            self.assert_user_pair_exists(pairs[i], providers[i]);
            fee_total += self.users.get(&providers[i]).unwrap().get_fee();
        }
        fee_total
    }
    
    // TODO see if flow of methods makes sense and secure
    // TODO does this need to be payable?
    // pub fn claim_earnings(&mut self) -> Promise {
    //     // TODO see if this withdraws and executes transfer atomically
    //     fungible_token_transfer(self.payment_token.clone(), env::predecessor_account_id(), self.withdraw_balance(env::predecessor_account_id()))
    // }

    fn get_user_exists(&self, account_id: &AccountId) -> bool {
        self.users.get(account_id).is_some()
    }

    pub fn get_pair_exists(&self, pair: String, user: AccountId) -> bool {
        self.get_user(&user)
            .pairs
            .get(&pair)
            .is_some()
    }

    #[payable]
    pub fn push_data(&mut self, pair: String, price: U128) {
        self.assert_user_pair_exists(pair, env::predecessor_account_id());
        let initial_storage_usage = env::storage_usage();
        let mut user = self.users.get(&env::predecessor_account_id()).unwrap();
        user.set_price(pair, u128::from(price));
        self.users
            .insert(&env::predecessor_account_id(), &user);
        helpers::refund_storage(initial_storage_usage, env::predecessor_account_id());
    }

    // pub fn set_fee(&mut self, fee: U128) {
    //     let mut user = self.users.get(&env::predecessor_account_id()).unwrap();
    //     user.set_fee(u128::from(fee));
    //     self.users.insert(&env::predecessor_account_id(), &user);
    // }

    fn get_entry(&mut self, pair: String, user: AccountId, amount: u128) -> Option<PriceEntry> {
        match self.users.get(&user).unwrap().get_entry(&pair, amount) {
            Some(entry) => {
                self.add_balance(&user, amount);
                Some(entry)
            }
            None => None
        }
    }
    fn aggregate_avg(
        &mut self,
        pairs: Vec<String>,
        users: Vec<AccountId>,
        min_last_update: WrappedTimestamp
    ) -> Option<PriceEntry> {

        let min_last_update: u64 = min_last_update.into();
        let mut amount_of_users = users.len();

        let cum = 0.0_f64;
        for i in 0..pairs.len() {
            let entry = self.get_entry(pairs[i], users[i], self.get_user(&users[i]).get_fee()).unwrap();
            let price_decimals = u128::from(entry.price) as f64 / 10_i32.pow(entry.decimals) as f64;
            cum += price_decimals;
        }
        let avg = cum / users.len() as f64;
        let decimals = helpers::precision(avg).unwrap_or(0);
        Some(PriceEntry {
            price: (avg * 10_i32.pow(decimals) as f64) as u128,
            decimals: decimals,
            last_update: min_last_update
        })
    }

    pub fn aggregate_collect(
        &mut self,
        pairs: Vec<String>,
        users: Vec<AccountId>,
        min_last_update: WrappedTimestamp,
    ) -> Option<Vec<PriceEntry>> {

        let min_last_update: u64 = min_last_update.into();
        pairs
            .iter()
            .enumerate()
            .map(|(i, account_id)| {
                return self.get_entry(pairs[i], users[i], self.get_user(&users[i]).get_fee())
            })
            .collect()
    }
}