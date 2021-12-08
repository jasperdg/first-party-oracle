use crate::utils::*;
use flux_sdk::{
    consts::MAX_GAS,
};
pub fn init_balance() -> u128 {
    to_yocto("100000")
}

pub struct TestAccount {
    pub account: UserAccount,
}

impl TestAccount {
    pub fn new(master_account: Option<&UserAccount>, account_id: Option<&str>) -> Self {
        match master_account {
            Some(master_account) => {
                let account = master_account.create_user(
                    account_id.expect("expected account id").to_string(),
                    init_balance(),
                );
                storage_deposit(
                    TOKEN_CONTRACT_ID,
                    &master_account,
                    SAFE_STORAGE_AMOUNT,
                    Some(account.account_id()),
                );
                storage_deposit(
                    ORACLE_CONTRACT_ID,
                    &master_account,
                    46800000000000000000000,
                    Some(account.account_id()),
                );
                near_deposit(&account, init_balance() / 2);
                Self { account }
            }
            None => Self {
                account: init_simulator(None),
            },
        }
    }

    /*** Setters ***/
    pub fn claim_earnings(&self) -> ExecutionResult {
        let res = self.account.call(
            ORACLE_CONTRACT_ID.to_string(),
            "claim_earnings",
            &[],
            MAX_GAS,
            1000000000000000000000,
        );
        res.assert_success();
        res
    }

    pub fn create_pair(&self, pair: String, decimals: u16, initial_price: U128) -> ExecutionResult {
        let res = self.account.call(
            ORACLE_CONTRACT_ID.to_string(),
            "create_pair",
            json!({
                "pair": pair,
                "decimals": decimals,
                "initial_price": initial_price
            })
            .to_string()
            .as_bytes(),
            MAX_GAS,
            1000000000000000000000,
        );
        res.assert_success();
        res
    }

    pub fn push_data(&self, pair: String, price: U128) -> ExecutionResult {
        let res = self.account.call(
            ORACLE_CONTRACT_ID.to_string(),
            "push_data",
            json!({
                "pair": pair,
                "price": price
            })
            .to_string()
            .as_bytes(),
            MAX_GAS,
            1000000000000000000000,
        );
        res.assert_success();
        res
    }

    pub fn set_fee(&self, fee: U128) -> ExecutionResult {
        let res = self.account.call(
            ORACLE_CONTRACT_ID.to_string(),
            "set_fee",
            json!({
                "fee": fee
            })
            .to_string()
            .as_bytes(),
            MAX_GAS,
            1000000000000000000000,
        );
        res.assert_success();
        res
    }

    fn ft_transfer_call(&self, receiver: &str, amount: u128, msg: String) -> ExecutionResult {
        let res = self.account.call(
            TOKEN_CONTRACT_ID.to_string(),
            "ft_transfer_call",
            json!({
                "receiver_id": receiver,
                "amount": U128(amount),
                "msg": msg,
                "memo": "".to_string()
            })
            .to_string()
            .as_bytes(),
            MAX_GAS,
            1,
        );

        assert!(res.is_ok(), "ft_transfer_call failed with res: {:?}", res);
        res
    }

    pub fn ft_transfer(&self, receiver: &str, amount: u128) -> ExecutionResult {
        let res = self.account.call(
            TOKEN_CONTRACT_ID.to_string(),
            "ft_transfer",
            json!({
                "receiver_id": receiver,
                "amount": U128(amount),
            })
            .to_string()
            .as_bytes(),
            MAX_GAS,
            1,
        );

        assert!(res.is_ok(), "ft_transfer failed with res: {:?}", res);
        res
    }

     /*** Getters ***/
     pub fn get_token_balance(&self, account_id: Option<String>) -> u128 {
        let account_id = match account_id {
            Some(account_id) => account_id,
            None => self.account.account_id(),
        };

        let res: U128 = self
            .account
            .view(
                TOKEN_CONTRACT_ID.to_string(),
                "ft_balance_of",
                json!({ "account_id": account_id }).to_string().as_bytes(),
            )
            .unwrap_json();

        res.into()
    }

    pub fn get_entry(&self, pair: String, provider: AccountId) -> PriceEntry {
        let res: PriceEntry = self.account.view(
            ORACLE_CONTRACT_ID.to_string(),
            "get_entry",
            json!({
                "pair": pair,
                "provider": provider
            })
            .to_string()
            .as_bytes()
        ).unwrap_json();
        res.into()
    }

    pub fn aggregate_avg(&self, pairs: Vec<String>, providers: Vec<AccountId>, min_last_update: WrappedTimestamp) -> PriceEntry {
        let res: PriceEntry = self.account.view(
            ORACLE_CONTRACT_ID.to_string(),
            "aggregate_avg",
            json!({
                "pairs": pairs,
                "providers": providers,
                "min_last_update": min_last_update
            })
            .to_string()
            .as_bytes()
        ).unwrap_json();
        res.into()
    }

    pub fn aggregate_call(&self, pairs: Vec<String>, providers: Vec<AccountId>, min_last_update: WrappedTimestamp) -> PriceEntry {
        let res: PriceEntry = self.account.view(
            ORACLE_CONTRACT_ID.to_string(),
            "aggregate_call",
            json!({
                "pairs": pairs,
                "providers": providers,
                "min_last_update": min_last_update
            })
            .to_string()
            .as_bytes()
        ).unwrap_json();
        res.into()
    }
}
