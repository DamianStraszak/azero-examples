import fs from "fs";

import { ApiPromise } from "@polkadot/api";
import { Abi } from "@polkadot/api-contract";

// configuration file containing ws endpoint and contracts addresses
import config from "../config.json";
import { DecodedEvent } from "@polkadot/api-contract/types";

function readMetadata(path: string) {
  const contract = fs.readFileSync(path, { encoding: "utf8", flag: "r" });
  return JSON.parse(contract);
}

/// Loads contracts listed in the config.json file
export async function loadContractList(
  api: ApiPromise
): Promise<{ [address: string]: { name: string; codeHash: string } }> {
  const contracts = config.contracts;
  const tracked_contracts = {};
  Object.keys(contracts).forEach(async (contractAddress) => {
    tracked_contracts[contractAddress] = {
      name: config.contracts[contractAddress].name,
      codeHash: "",
    };
    if (contracts[contractAddress].metadata_override) {
      // use codeHash from the config file
      const abi_raw = readMetadata(
        config.metadata_dir + "/" + contracts[contractAddress].metadata_override
      );
      const codeHash = abi_raw.source.hash;
      tracked_contracts[contractAddress]["codeHash"] = codeHash;
    } else {
      // use on-chain codeHash
      const contract_info = await api.query.contracts.contractInfoOf(
        contractAddress
      );
      const onChainCodeHash = contract_info.toJSON()["codeHash"];
      tracked_contracts[contractAddress]["codeHash"] = onChainCodeHash;
    }
  });
  return tracked_contracts;
}

export async function loadContractAbis(): Promise<{ [codeHash: string]: Abi }> {
  const abis = {};
  fs.readdir(config.metadata_dir, (_err, files) => {
    files.forEach((file) => {
      const abi_raw = readMetadata(config.metadata_dir + "/" + file);
      const codeHash = abi_raw.source.hash;
      const abi = new Abi(abi_raw);
      abis[codeHash] = abi;
    });
  });
  return abis;
}

export function logEventInHumanFriendlyWay(decodedEvent: DecodedEvent) : void {
  const name = decodedEvent.event.identifier.toString();
  const args = [];
  decodedEvent.event.args.forEach((eventParam, index) => {
    args[index] = {
      label: eventParam.name,
      value: decodedEvent.args[index].toHuman(),
    };
  });
  console.log({
    name,
    args,
  });
}
