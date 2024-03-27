import json
import argparse
from contract_tracker import ContractTracker, hash_from_num, get_current_block_number

CONFIG_FILE = "config.json"


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('--config', type=str, help='Path to the config file', default='config.json')
    args = parser.parse_args()
    config_path = args.config
    print(f"Reading config from {config_path}")
    tracker = ContractTracker(config_path)
    chain = tracker.chain
    current_block_num = get_current_block_number(chain)
    start_num = 58079720
    stop_num = start_num + 20
    for num in range(start_num, stop_num):
        if num % 100 == 0:
            print(f"Processing block {num}")
        block_hash = hash_from_num(chain, num)
        events = tracker.get_contract_events_in_block(block_hash, simplified=True)
        for e in events:
            print(f"Emitted event in block {num} by {e.emitter_name} ({e.emitter_address}):")
            print(json.dumps(e.event, indent=2))


