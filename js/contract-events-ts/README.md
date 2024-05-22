## Example

1. Install dependencies `npm install`
2. Run `npm run getEventsBlockRange` to print events from the example contracts configured in `config.json`.
3. Run `npm run subscribeEvents` to subscribe to events in on-going blocks.

## Instructions for tracking any contract events

1. Place `.json` (or `.contract`) files (these are analogous to ABI files from Solidity) in the `contract_metadata` folder. These are obtained when compiling your ink! contract (available in `/target/ink`). 
2. Configure the `ws_endpoint` in the `config.json` file. This should be `ws://localhost:9944` for local node, `wss://ws.test.azero.dev` for Aleph Zero testnet, and `wss://ws.azero.dev` for Aleph Zero Mainnet. 
3. Configure the contract addresses you want to track in the `config.json` file.
3. Note that the metadata files (in `contract_metadata`) include the `hash` of the contract wasm binary, and it is used to link by our script to link a contract address to the metadata file. In case the on-chain contract you want to track comes from the same code, but for some reason has a hash incompatible with what's in the metadata file (this can happen if you recompile the contract after deploying to chain) the script will give an error that no metadata was found for the given contract. In this case you can manually "override" the metadata for a contract, by just specifying the name of the metadata file in the `contract_metadata` directory which should be used to decode the events emitted by this contract. If you do that, we cannot unfortunately verify if the override is correct, and if it isn't, the script can crash or produce incorrect results.
4. Run `npm run subscribeEvents` or customize and run the `npm run getEventsBlockRange` script to your needs.