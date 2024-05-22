import { ApiPromise, WsProvider } from "@polkadot/api";
import { Abi } from "@polkadot/api-contract";
import type { EventRecord } from "@polkadot/types/interfaces";

// configuration file containing ws endpoint and contracts addresses
import config from "../config.json";

import { loadContractAbis, loadContractList, logEventInHumanFriendlyWay} from "./utils";


async function main() {
  const provider = new WsProvider(config.ws_endpoint);

  // Create the API and wait until ready
  const api = await ApiPromise.create({ provider });
  const tracked_contracts  = await loadContractList(api);
  const abis  = await loadContractAbis();

  subscribeContractsEvents(api, abis, tracked_contracts);
}

// main().catch(console.error).finally(() => process.exit());
main().catch((error) => {
  console.error(error);
  process.exit(-1);
});

function subscribeContractsEvents(
  api: ApiPromise,
  abis: { [codeHash: string]: Abi },
  tracked_contracts: { [address: string]: { name: string; codeHash: string } }
) {
  api.query.system.events((allEvents: EventRecord[]) => {
    // filter for contract emitted events
    allEvents
      .filter(({ event }) => api.events.contracts.ContractEmitted.is(event))
      .forEach(async (record) => {
        // get contract address from the event
        const contractAddress = record.event.data[0].toString();
        const contractAbi = abis[tracked_contracts[contractAddress]?.codeHash];
        if (contractAbi) {
          const blockNumber = (await api.rpc.chain.getHeader()).number;
          console.log(
            '\x1b[36m%s\x1b[0m', `Emitted events in block ${blockNumber} by ${tracked_contracts[contractAddress].name} (${contractAddress}):`
          );
          logEventInHumanFriendlyWay(contractAbi.decodeEvent(record));
        }
      });
  });
}
