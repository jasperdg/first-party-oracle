use super::*;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::serde::Serialize;
use near_sdk::{Promise};

/// Price per 1 byte of storage from mainnet config after `0.18` release and protocol version `42`.
/// It's 10 times lower than the genesis price.

pub const STORAGE_MINIMUM_BALANCE: Balance = 10_000_000_000_000_000_000_000;

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageBalance {
    total: U128,
    available: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Copy, Clone, Serialize, Deserialize)]
pub struct AccountStorageBalance {
    pub total: u128,
    pub available: u128,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageBalanceBounds {
    min: U128,
    max: Option<U128>,
}

pub trait StorageManager {
    fn storage_deposit(&mut self, account_id: Option<ValidAccountId>) -> StorageBalance;

    fn storage_withdraw(&mut self, amount: U128) -> StorageBalance;

    fn storage_balance_bounds(&self) -> StorageBalanceBounds;

    fn storage_balance_of(&self, account_id: ValidAccountId) -> Option<StorageBalance>;
}

fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yoctoNEAR"
    )
}

#[near_bindgen]
impl StorageManager for FirstPartyOracle {
    #[payable]
    fn storage_deposit(&mut self, account_id: Option<ValidAccountId>) -> StorageBalance {
        let amount = env::attached_deposit();
        let account_id = account_id
            .map(|a| a.into())
            .unwrap_or_else(|| env::predecessor_account_id());
        
        let mut balance = self.get_storage_account_balance(&account_id);
        assert!(
            amount >= STORAGE_MINIMUM_BALANCE,
            "Deposit must be at least {} yoctoNEAR",
            STORAGE_MINIMUM_BALANCE
        );

        balance.available += amount;
        balance.total += amount;

        let storage_balance = StorageBalance {
            total: U128(balance.total),
            available: U128(balance.available),
        };

        self.set_storage_account_balance(&account_id, balance);

        // self.providers.insert(&account_id, &balance);

        storage_balance
    }

    #[payable]
    fn storage_withdraw(&mut self, amount: U128) -> StorageBalance {
        assert_one_yocto();
        let amount: Balance = amount.into();
        let account_id = env::predecessor_account_id();
        let mut balance = self.get_storage_account_balance(&account_id);

        assert!(amount <= balance.available, "Not enough storage available");

        balance.available -= amount;
        balance.total -= amount;

        let storage_balance = StorageBalance {
            total: U128(balance.total),
            available: U128(balance.available),
        };

        self.set_storage_account_balance(&account_id, balance);

        Promise::new(account_id).transfer(amount);

        storage_balance
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        StorageBalanceBounds {
            min: U128(STORAGE_MINIMUM_BALANCE),
            max: None,
        }
    }

    fn storage_balance_of(&self, account_id: ValidAccountId) -> Option<StorageBalance> {
        Some(self.providers
            .get(account_id.as_ref())
            .unwrap()
            .get_balance())
            .map(|balance| StorageBalance {
                total: U128(balance.total),
                available: U128(balance.available),
            })
    }
}

impl FirstPartyOracle {
    pub fn get_storage_account_balance(&self, account_id: &AccountId) -> AccountStorageBalance {
        let provider = self.providers
            .get(account_id)
            .unwrap_or(Provider::new());
        provider.get_balance()
    }

    pub fn set_storage_account_balance(&mut self, account_id: &AccountId, balance: AccountStorageBalance) {
        let mut provider = self.providers
            .get(account_id)
            .unwrap_or(Provider::new());
        provider.set_balance(balance);
        self.providers.insert(account_id, &provider);
    }

    pub fn use_storage(
        &mut self,
        sender_id: &AccountId,
        initial_storage_usage: u64,
        initial_available_balance: u128,
    ) {
        if env::storage_usage() >= initial_storage_usage {
            // used more storage, deduct from balance
            let difference: u128 = u128::from(env::storage_usage() - initial_storage_usage);
            let mut balance = self.get_storage_account_balance(sender_id);
            let cost = difference * env::STORAGE_PRICE_PER_BYTE;
            assert!(
                cost <= initial_available_balance,
                "{} has {} deposited, {} is required for this transaction",
                sender_id,
                initial_available_balance,
                cost
            );
            balance.available =
                initial_available_balance - difference * env::STORAGE_PRICE_PER_BYTE;

            self.set_storage_account_balance(sender_id, balance);
        } else {
            // freed up storage, add to balance
            let difference: u128 = u128::from(initial_storage_usage - env::storage_usage());
            let mut balance = self.get_storage_account_balance(sender_id);
            balance.available =
                initial_available_balance + difference * env::STORAGE_PRICE_PER_BYTE;

            self.set_storage_account_balance(sender_id, balance);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod mock_token_basic_tests {
    use super::*;
    use near_sdk::{testing_env, MockedBlockchain, VMContext};
    use std::convert::TryInto;

    fn alice() -> AccountId {
        "alice.near".to_string()
    }

    fn bob() -> AccountId {
        "bob.near".to_string()
    }

    fn token() -> AccountId {
        "token.near".to_string()
    }

    fn to_valid(account: AccountId) -> ValidAccountId {
        account.try_into().expect("invalid account")
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContext {
            current_account_id: token(),
            signer_account_id: bob(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 1000 * 10u128.pow(24),
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit: 1000 * 10u128.pow(24),
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn storage_manager_deposit() {
        testing_env!(get_context(token()));
        let mut contract = FirstPartyOracle::new(token());

        let balance = contract.get_storage_account_balance(&alice());
        assert_eq!(balance.available, 0);

        let amount = 10u128.pow(24);

        //deposit
        let mut c: VMContext = get_context(alice());
        c.attached_deposit = amount;
        testing_env!(c);
        contract.storage_deposit(Some(to_valid(alice())));

        let balance = contract.get_storage_account_balance(&alice());
        assert_eq!(balance.available, amount);

        //deposit again
        let mut c: VMContext = get_context(alice());
        c.attached_deposit = amount;
        testing_env!(c);
        contract.storage_deposit(Some(to_valid(alice())));

        let balance = contract.get_storage_account_balance(&alice());
        assert_eq!(balance.available, amount * 2);
    }

    #[test]
    fn storage_manager_withdraw() {
        testing_env!(get_context(token()));
        let mut contract = FirstPartyOracle::new(token());

        let balance = contract.get_storage_account_balance(&alice());
        assert_eq!(balance.available, 0);

        let amount = 10u128.pow(24);

        //deposit
        let mut c: VMContext = get_context(alice());
        c.attached_deposit = amount;
        testing_env!(c);
        contract.storage_deposit(Some(to_valid(alice())));

        // withdraw
        let mut c: VMContext = get_context(alice());
        c.attached_deposit = 1;
        testing_env!(c);

        contract.storage_withdraw(U128(amount / 2));
        let account = contract.get_storage_account_balance(&alice());
        assert_eq!(account.available, amount / 2);
    }

    #[test]
    #[should_panic(expected = "Not enough storage available")]
    fn storage_manager_withdraw_too_much() {
        testing_env!(get_context(token()));
        let mut contract = FirstPartyOracle::new(token());

        let balance = contract.get_storage_account_balance(&alice());
        assert_eq!(balance.available, 0);

        let amount = 10u128.pow(24);

        //deposit
        let mut c: VMContext = get_context(alice());
        c.attached_deposit = amount;
        testing_env!(c);
        contract.storage_deposit(Some(to_valid(alice())));

        // withdraw
        let mut c: VMContext = get_context(alice());
        c.attached_deposit = 1;
        testing_env!(c);

        contract.storage_withdraw(U128(amount * 2));
    }
}