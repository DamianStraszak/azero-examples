import json
import argparse
from contract_tracker import ContractTracker, hash_from_num, get_finalized_block_number

CONFIG_FILE = "config.json"


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('--config', type=str, help='Path to the config file', default='config.json')
    args = parser.parse_args()
    config_path = args.config
    print(f"Reading config from {config_path}")
    tracker = ContractTracker(config_path)
    chain = tracker.chain
    current_block_num = get_finalized_block_number(chain)
    while True:
        finalized_block_num = get_finalized_block_number(chain)
        while current_block_num < finalized_block_num:
            current_block_num += 1
            if current_block_num % 100 == 0:
                print(f"Processing block {current_block_num}")
            block_hash = hash_from_num(chain, current_block_num)
            events = tracker.get_contract_events_in_block(block_hash, simplified=True)
            for e in events:
                print(f"Emitted event in block {current_block_num} by {e.emitter_name} ({e.emitter_address}):")
                print(json.dumps(e.event, indent=2))


