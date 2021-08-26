# Requestor Template Contract

Interested in integrating with the Flux Oracle? **Request Interfaces** act as access points into the oracle and are allowed to create data requests -- here you will find a template contract to create a Request Interface.

On Flux testnet, anyone can test a custom Request Interface and connect it to the testnet oracle without requiring a successful proposal and execution by the Flux DAO (as on mainnet). That means anyone or any protocol can experiment directly with the Flux Oracle as a data requestor to put any kind of data on-chain to be resolved by our pool of testnet validators.

[Please visit the documentation](https://docs.fluxprotocol.org/docs/getting-started/data-requestors) for more information on getting set up as a data requestor!

## Options

### Whitelist

Request Interfaces are encouraged to create mechanisms to ensure domain-specific and high-quality (with definite answers, not spammy, etc.) requests are sent to the Flux Oracle to encourage validators to participate in data resolution.

One option is to whitelist the account(s) allowed to call `create_data_request()` by deploying the contract with an array of account IDs for the `whitelist` parameter in the `init()` method. If left empty, any account will be able to call `create_data_request()`, so another mechanism to limit the number of requests sent to the oracle (e.g. time-based limits, governance controls) is encouraged.