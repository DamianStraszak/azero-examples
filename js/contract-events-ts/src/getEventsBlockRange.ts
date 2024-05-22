import { ApiPromise, WsProvider } from "@polkadot/api";
import { Abi } from "@polkadot/api-contract";
import type { EventRecord } from "@polkadot/types/interfaces";

// configuration file containing ws endpoint and contracts addresses
import config from "../config.json";

import { loadContractAbis, loadContractList, logEventInHumanFriendlyWay } from "./utils";

async function main() {
  const provider = new WsProvider(config.ws_endpoint);

  // Create the API and wait until ready
  const api = await ApiPromise.create({ provider });
  const tracked_contracts  = await loadContractList(api);
  const abis  = await loadContractAbis();

  // blocks range
  const startNum = 58079720;
  const stopNum = startNum + 20;

  for (let blockNumber = startNum; blockNumber <= stopNum; ++blockNumber) {
    await getEventsByBlock(api, abis, tracked_contracts, blockNumber);
  }
}

main()
  .catch(console.error)
  .finally(() => process.exit());

async function getEventsByBlock(
  api: ApiPromise,
  abis: { [codeHash: string]: Abi },
  tracked_contracts: { [address: string]: { name: string; codeHash: string } },
  blockNumber: number
) {
  const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
  const allRecords: EventRecord[] = await api.query.system.events.at(blockHash);
  // filter the specific events based on the phase and then the
  // index of our extrinsic in the block
  allRecords
    .filter(({ event }) => api.events.contracts.ContractEmitted.is(event))
    .forEach(async (record) => {
      // get contract address from the event
      const contractAddress = record.event.data[0].toString();
      const contractAbi = abis[tracked_contracts[contractAddress]?.codeHash];
      if (contractAbi) {
        console.log(
          "\x1b[36m%s\x1b[0m",
          `Emitted events in block ${blockNumber} by ${tracked_contracts[contractAddress].name} (${contractAddress}):`
        );
        logEventInHumanFriendlyWay(contractAbi.decodeEvent(record));
      }
    });
}
