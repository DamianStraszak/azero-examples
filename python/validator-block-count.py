from substrateinterface import SubstrateInterface

TESTNET_WS = "wss://ws.test.azero.dev"


def get_block_author(chain, block_hash):
    b = chain.get_block(block_hash=block_hash)
    if b['header']['number'] == 0:
        return None
    parent_hash = b['header']['parentHash']
    digests = b['header']['digest']['logs']
    slot_bytes = bytes.fromhex(digests[0].data.to_hex()[2:])[6:]
    slot = int.from_bytes(slot_bytes, byteorder='little')
    authorities = chain.query('Session', 'Validators', block_hash=parent_hash)
    return authorities[slot % len(authorities)].value


def get_session_produced_counts(chain, session):
    last_num = session*900 + 900
    last_block = chain.get_block(block_number=last_num)
    last_block_hash = last_block['header']['hash']
    prev_last_block_hash = last_block['header']['parentHash']
    res = chain.query_map(
        'Elections', 'SessionValidatorBlockCount', block_hash=prev_last_block_hash)
    res = {str(acc): int(cnt.value) for (acc, cnt) in res}
    last_author = str(get_block_author(chain, last_block_hash))
    res[last_author] = res.get(last_author, 0) + 1
    return res


def main():
    testnet = SubstrateInterface(url=TESTNET_WS, ss58_format=42)
    session = 31780
    counts = get_session_produced_counts(testnet, session)

    print(
        f"Block produced count by validator in Session {session} on Aleph Zero testnet")

    for acc, cnt in counts.items():
        print(f"{acc} : {cnt}")

    total_cnt = sum([cnt for (acc, cnt) in counts.items()])
    print(f"Total: {total_cnt} blocks")


if __name__ == "__main__":
    main()
