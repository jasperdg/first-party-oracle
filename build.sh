#!/bin/bash
# Set testnet as near environment
env NEAR_ENV=testnet

# Register account with wNEAR contract and Oracle contract, give 20 NEAR to store with oracle to allow for multiple Data Requests to be made
near call v2.wnear.flux-dev storage_deposit '{"account_id": "YOUR_TESTNET_ACCOUNT_ID"}' --accountId YOUR_TESTNET_ACCOUNT_ID --amount 0.00125 --gas=300000000000000
near call 05.oracle.flux-dev storage_deposit '{"account_id": "YOUR_TESTNET_ACCOUNT_ID"}' --accountId YOUR_TESTNET_ACCOUNT_ID --amount 20 --gas=300000000000000

# Deposit 20 NEAR to get 20 wNEAR tokens to use in your contract
near call v2.wnear.flux-dev near_deposit "{}" --accountId YOUR_TESTNET_ACCOUNT_ID --amount 20 --gas=300000000000000

# Build and deploy your requestor-sample-contract
cargo build --target wasm32-unknown-unknown --release
cp ./target/wasm32-unknown-unknown/release/request_interface.wasm  ./res
near deploy YOUR_TESTNET_ACCOUNT_ID ./res/request_interface.wasm new --initArgs '{"oracle": "05.oracle.flux-dev", "stake_token": "v2.wnear.flux-dev"}'