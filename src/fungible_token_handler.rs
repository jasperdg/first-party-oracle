use flux_sdk::consts::{GAS_BASE_TRANSFER};
use near_sdk::{ext_contract, json_types::U128, AccountId, Promise, PromiseOrValue};

#[ext_contract(fungible_token)]
pub trait FungibleToken {
    fn ft_transfer(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    ) -> Promise;
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> Promise;
    fn ft_balance_of(&self, account_id: AccountId) -> Promise;
}

pub trait FungibleTokenReceiver {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

pub fn fungible_token_transfer(
    token_account_id: AccountId,
    receiver_id: AccountId,
    value: u128,
) -> Promise {
    fungible_token::ft_transfer(
        receiver_id,
        U128(value),
        None,
        // Near params
        &token_account_id,
        1,
        GAS_BASE_TRANSFER,
    )
}