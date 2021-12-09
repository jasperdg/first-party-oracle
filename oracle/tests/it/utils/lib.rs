use near_sdk::{
  json_types::{
      U128,
      WrappedTimestamp
  },
  serde_json::json,
  AccountId
};
use near_sdk_sim::{
    deploy, init_simulator, to_yocto, call, view,
    ContractAccount, UserAccount, ExecutionResult
};
mod account_utils;
mod deposit;
mod oracle_utils;
mod requester_utils;
mod token_utils;

extern crate oracle;
use deposit::*;
use account_utils::*;
pub use oracle::*;
use requester;
use token;

// use oracle::FirstPartyOracleContract;

const TOKEN_CONTRACT_ID: &str = "token";
pub const ORACLE_CONTRACT_ID: &str = "oracle";
pub const REQUESTER_CONTRACT_ID: &str = "requester";
pub const SAFE_STORAGE_AMOUNT: u128 = 1250000000000000000000;

type OracleContract = oracle::FirstPartyOracleContract;
type RequesterContract = requester::RequesterContract;
type TokenContract = token::TokenContractContract;


near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
  ORACLE_CONTRACT_WASM_BYTES => "../res/oracle.wasm",
  TOKEN_CONTRACT_WASM_BYTES => "../res/token.wasm",
  REQUESTER_CONTRACT_WASM_BYTES => "../res/requester.wasm"
}

pub struct TestUtils {
  pub master_account: TestAccount,
  pub oracle_contract: ContractAccount<OracleContract>,
  pub token_contract: ContractAccount<TokenContract>,
  pub alice: account_utils::TestAccount,
  pub bob: account_utils::TestAccount,
  pub jack: account_utils::TestAccount,
  // pub requester_contract: ContractAccount<RequesterContract>,
}


impl TestUtils {
  pub fn init() -> Self {
    println!("Setting up Master account...");
    let master_account = TestAccount::new(None, None);
    println!("Master account set up. Deploying token...");
    let token_init_res = token_utils::TokenUtils::new(&master_account);
    println!("Token deployed. Deploying oracle...");
    let oracle_init_res = oracle_utils::OracleUtils::new(&master_account); 
    println!("Oracle deployed. Setting up alice...");
    let alice = TestAccount::new(Some(&master_account.account), Some("alice"));
    println!("Alice set up. Setting up bob...");
    let bob = TestAccount::new(Some(&master_account.account), Some("bob"));
    println!("Bob set up. Setting up jack...");
    let jack = TestAccount::new(Some(&master_account.account), Some("jack"));
    println!("Jack set up. Deploying requester for alice...");
    // let requester_init_res = requester_utils::RequesterUtils::new(&alice);
    // println!("Requester for alice set up");
    Self {
        master_account: master_account,
        token_contract: token_init_res.contract,
        oracle_contract: oracle_init_res.contract,
        alice: alice,
        bob: bob,
        jack: jack,
        // requester_contract: requester_init_res.contract,
    }
  }
}
