use crate::*;

use near_sdk::{ext_contract, json_types::U128, Gas, Promise};

#[ext_contract(oracle)]
pub trait Oracle {
    fn dr_new(
        &mut self,
        sender: AccountId,
        amount: U128,
        payload: NewDataRequestArgs,
    ) -> Promise;
}

const GAS_BASE_TRANSFER: Gas = 5_000_000_000_000;
const DR_NEW_GAS: Gas = 200_000_000_000_000;


impl RequesterContract {
    pub fn dr_new(
        &self,
        payload: NewDataRequestArgs,
    ) -> Promise {
        oracle::dr_new(
            env::current_account_id(),
            U128(0),
            payload,
            // Near params
            &self.oracle,
            1,
            GAS_BASE_TRANSFER,
        )
    }
}
