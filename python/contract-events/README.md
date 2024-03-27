# Reading Contract Events on Aleph Zero


## Example

1. Install python dependencies `pip install -r requirements.txt`
2. Run `python get_events_block_range.py` to print events from the example contracts configured in `config.json`.
3. Run `python subscribe_events.py` to subscribe to events in on-going blocks.
4. You can experiment with setting the `simplified` flag to `True` in this piece of code:
`tracker.get_contract_events_in_block(block_hash, simplified=True)`


## Instructions for tracking any contract events

1. Place `.json` (or `.contract`) files (these are analogous to ABI files from Solidity) in the `contract_metadata` folder. These are obtained when compiling your ink! contract (available in `/target/ink`). 
2. Configure the `ws_endpoint` in the `config.json` file. This should be `ws://localhost:9944` for local node, `wss://ws.test.azero.dev` for Aleph Zero testnet, and `wss://ws.azero.dev` for Aleph Zero Mainnet. 
3. Configure the contract addresses you want to track in the `config.json` file.
3. Note that the metadata files (in `contract_metadata`) include the `hash` of the contract wasm binary, and it is used to link by our python script to link a contract address to the metadata file. In case the on-chain contract you want to track comes from the same code, but for some reason has a hash incompatible with what's in the metadata file (this can happen if you recompile the contract after deploying to chain) the script will give an error that no metadata was found for the given contract. In this case you can manually "override" the metadata for a contract, by just specifying the name of the metadata file in the `contract_metadata` directory which should be used to decode the events emitted by this contract. If you do that, we cannot unfortunately verify if the override is correct, and if it isn't, the script can crash or produce incorrect results.
4. Run `python subscribe_events.py` or customize and run the `get_events_block_range.py` script to your needs.



## FAQ

### Q: Where do I get the contract metadata from?
A: If you are the author of the contract, or you have access to the source code, you just need to copy the json file from `/target/ink/` after you compile the contract. Otherwise, you should ask the person/project who deployed the contract to provide you with the metadata. You can also check here -- https://smartcontracthub.tech/ -- maybe the contract you are interested in is available.

### Q: Can multiple contracts use the same metadata file?
A: Yes. In fact this is the case for the example. These contracts have different addresses but the same code.

### Q: I don't have the metadata. Can I still track the events?
A: Short answer: No. Long answer: in some cases yes, but it's hard. You should get the metadata.

### Q: Do I need to fill in config.json? Could I instead track everything for which there is metadata in `contract_metadata`?
A: It is possible to track everything, it's a simple modification of the provided scripts.

### Q: Is it a problem if I have metadata files in `contract_metadata` that are not used by the tracked contracts (in `config.json`)?
A: No. In fact it's probably best to put EVERY single contract metada file you can find in the `contract_metadata` file. No harm in that.

### Q: How to track events in JS?
A: It should be possible to write a similar script using `polkadot-js`

### Q: How to track events in Rust?
A: It should be possible to write a similar script using `subxt` and the transcoder from https://github.com/paritytech/cargo-contract

### Q: Something doesn't work for me?
A: It's best if you write a question on the builders channel on Aleph Zero discord. Or on  ink! Developer Group on telegram (https://t.me/inkathon).
