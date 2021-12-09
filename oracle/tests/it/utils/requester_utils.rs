use crate::utils::*;
pub struct RequesterUtils {
    pub contract: ContractAccount<RequesterContract>,
}

impl RequesterUtils {
    pub fn new(account_id: &TestAccount) -> Self {
        // deploy token
        let contract = deploy!(
            // Contract Proxy
            contract: RequesterContract,
            // Contract account id
            contract_id: REQUESTER_CONTRACT_ID,
            // Bytes of contract
            bytes: &REQUESTER_CONTRACT_WASM_BYTES,
            // User deploying the contract,
            signer_account: account_id.account,
            deposit: to_yocto("1000"),
            // init method
            init_method: new(
                ORACLE_CONTRACT_ID.to_string()
            )
        );

        storage_deposit(
            TOKEN_CONTRACT_ID,
            &account_id.account,
            SAFE_STORAGE_AMOUNT,
            Some(REQUESTER_CONTRACT_ID.to_string()),
        );
        storage_deposit(
            ORACLE_CONTRACT_ID,
            &account_id.account,
            10000000000000000000000,
            Some(REQUESTER_CONTRACT_ID.to_string()),
        );

        Self { contract }
    }
}
