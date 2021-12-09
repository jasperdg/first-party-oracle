use crate::utils::*;
pub struct OracleUtils {
    pub contract: ContractAccount<FirstPartyOracleContract>,
}

impl OracleUtils {
    pub fn new(master_account: &TestAccount) -> Self {
        // deploy token
        let contract = deploy!(
            // Contract Proxy
            contract: FirstPartyOracleContract,
            // Contract account id
            contract_id: ORACLE_CONTRACT_ID,
            // Bytes of contract
            bytes: &ORACLE_CONTRACT_WASM_BYTES,
            // User deploying the contract,
            signer_account: master_account.account,
            deposit: to_yocto("1000"),
            // init method
            init_method: new(
                master_account.account.account_id().to_string(),
                TOKEN_CONTRACT_ID.to_string()
            )
        );

        storage_deposit(
            TOKEN_CONTRACT_ID,
            &master_account.account,
            SAFE_STORAGE_AMOUNT,
            Some(ORACLE_CONTRACT_ID.to_string()),
        );
        Self { contract }
    }
}
