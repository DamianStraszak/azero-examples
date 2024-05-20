import time
import json
import os
import argparse
from substrateinterface import SubstrateInterface, ContractMetadata, ContractEvent
from scalecodec.base import ScaleBytes

CONFIG_FILE = "config.json"


def hash_from_num(chain, num):
    return chain.get_block(block_number=num)['header']['hash']

def num_from_hash(chain, block_hash):
    return chain.get_block(block_hash=block_hash)['header']['number']

def get_block_by_num(chain, num):
    return chain.get_block(block_number=num)


def get_current_block_hash(chain):
    return chain.get_block()['header']['hash']


def get_current_block_number(chain):
    return chain.get_block()['header']['number']

def get_finalized_block_number(chain):
    finalized_hash = chain.get_chain_finalised_head()
    return num_from_hash(chain, finalized_hash)

def get_contract_info_of(chain, address, block_hash=None):
    res = chain.query(module='Contracts',storage_function='ContractInfoOf',params=[address],block_hash=block_hash)
    return res.serialize()

def get_all_contract_info_of(chain, block_hash=None):
    res = chain.query_map(module='Contracts',storage_function='ContractInfoOf',block_hash=block_hash)
    return {str(k):v.value for (k,v) in res}

def get_contract_to_code_hash_map(chain, block_hash=None):
    contract_infos = get_all_contract_info_of(chain, block_hash)
    return {acc:v['code_hash'] for (acc, v) in contract_infos.items()}

def get_contract_code_hash(chain, address):
    return get_contract_info_of(chain, address)['code_hash']


def decode_event(chain, event_data, contract_metadata, simplified = False):
    contract_event_obj = ContractEvent(
                            data=ScaleBytes(event_data),
                            runtime_config=chain.runtime_config,
                            contract_metadata=contract_metadata
                        )
    event = contract_event_obj.decode()
    if simplified:
        new_event = {}
        new_event['name'] = event['name']
        new_args = []
        for arg in event['args']:
            new_arg = {}
            new_arg['label'] = arg['label']
            new_arg['value'] = arg['value']
            new_args.append(new_arg)
        new_event['args'] = new_args
        event = new_event
    return event


class DecodedEvent:
    def __init__(self, event, emitter_address, emitter_name, block_hash):
        self.event = event
        self.emitter_address = emitter_address
        self.emitter_name = emitter_name
        self.block_hash = block_hash

    def __str__(self):
        return f"Event {self.event} emitted by {self.emitter_name} ({self.emitter_address}) at block {self.block_hash}"

class LoadedMetadata:
    def __init__(self, chain, file_path):
        self.metadata_file = os.path.basename(file_path)
        self.metadata = ContractMetadata.create_from_file(file_path, chain)
        self.code_hash = self.metadata.metadata_dict['source']['hash']
        self.chain = chain

    def decode_event(self, event_data, emitter_address, emitter_name, block_hash, simplified = False):
        event = decode_event(self.chain, event_data, self.metadata, simplified)
        return DecodedEvent(event, emitter_address, emitter_name, block_hash, simplified)
        


def load_contract_metadata(chain, metadata_dir):
    contract_metadata = {}
    for file in os.listdir(metadata_dir):
        if file.endswith(".json") or file.endswith(".contract"):
            metadata = LoadedMetadata(chain, os.path.join(metadata_dir, file))
            if metadata.code_hash in contract_metadata:
                raise ValueError(f"Duplicate code hash found: {metadata.code_hash} in file {file}")
            contract_metadata[metadata.code_hash] = metadata
        else:
            print(f"Skipping file {file} in metadata directory. The file must have a .json or .contract extension.")
    return contract_metadata



def load_contract_list(contract_list_config_file):
     
    with open(contract_list_config_file, 'r') as f:
        config = json.load(f)
    ws_endpoint = config['ws_endpoint']
    print(f"Connecting to {ws_endpoint}")
    metadata_dir = config['metadata_dir']
    contract_list = config['contracts']
    chain = SubstrateInterface(url=ws_endpoint)
    tracked_contracts = {}
    for address in contract_list:
        if address in tracked_contracts:
            raise ValueError(f"Duplicate address found: {address}")
        info = contract_list[address]
        tracked_info = {}
        tracked_info['name'] = info['name']
        override = info.get('metadata_override', None)
        try:
            on_chain_code_hash = get_contract_code_hash(chain, address)
        except Exception as e:
            print(f"Error getting code hash for contract {address}: {e}. Aborting")
            raise ValueError(f"Error getting code hash for contract {address}: {e}. Aborting")
        print(f"Code hash of contract {info['name']} ({address}) on chain: {on_chain_code_hash}")
        if override is not None:
            print(f"Overriding metadata for contract {address} with file {override}")
            override = LoadedMetadata(chain, os.path.join(metadata_dir, override))
            print(f"Code hash of new metadata: {override.code_hash}")
            tracked_info['code_hash'] = override.code_hash
        else:
            tracked_info['code_hash'] = on_chain_code_hash
        tracked_contracts[address] = tracked_info
    return (chain, tracked_contracts, metadata_dir)


def is_hex_string(s):
    # Check for the '0x' prefix
    if s.startswith('0x'):
        # Remove the prefix
        hex_part = s[2:]
        # Check if the remaining characters are all hexadecimal digits
        return all(c in '0123456789abcdefABCDEF' for c in hex_part)
    return False




class ContractTracker:
    def __init__(self, config_file):
        (chain, tracked_contracts, metadata_dir) = load_contract_list(config_file)
        self.contract_metadata = load_contract_metadata(chain, metadata_dir)
        self.chain = chain
        self.tracked_contracts = tracked_contracts

        print(f"Tracking:\n {json.dumps(tracked_contracts, indent=2)}")
        

    def get_contract_events_in_block(self, block_hash, simplified = False):
        events = self.chain.get_events(block_hash)
        events = list(events)
        decoded_events = []
        for e in events:
            e = e.value
            module = e['event']['module_id']
            event_id = e['event']["event_id"]
            if module == 'Contracts' and event_id == 'ContractEmitted':
                emitter = e['event']['attributes']['contract']
                data = e['event']['attributes']['data']
                if not is_hex_string(data):
                    data = "0x"+(e['event']['attributes']['data'].encode().hex())
                tracked = self.tracked_contracts.get(emitter, None)
                if tracked is None:
                    continue
                tracked_hash = tracked['code_hash']
                tracked_name = tracked['name']
                metadata = self.contract_metadata[tracked_hash].metadata
                d = decode_event(self.chain, data, metadata, simplified = simplified)
                decoded_events.append(DecodedEvent(d, emitter, tracked_name, block_hash))
        return decoded_events


