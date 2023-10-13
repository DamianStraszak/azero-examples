from substrateinterface.contracts import ContractInstance, ContractMetadata
from substrateinterface import SubstrateInterface, Keypair

TESTNET_WS = "wss://ws.test.azero.dev"
MAINNET_WS = "wss://ws.azero.dev"
LOCAL_WS = "ws://localhost:9944"
PSP22_METADATA_FILE = "psp22.json"

alice = Keypair.create_from_uri('//Alice')

def contract_from_address(address, chain):
    psp22_metadata = ContractMetadata.create_from_file(PSP22_METADATA_FILE, chain)
    return ContractInstance(address, psp22_metadata, substrate=chain)


def token_name(address, chain):
    contract = contract_from_address(address, chain)
    return contract.read(alice, 'PSP22Metadata::token_name', args = {}).contract_result_data.value['Ok']

def token_symbol(address, chain):
    contract = contract_from_address(address, chain)
    return contract.read(alice, 'PSP22Metadata::token_symbol', args = {}).contract_result_data.value['Ok']

def token_decimals(address, chain):
    contract = contract_from_address(address, chain)
    return contract.read(alice, 'PSP22Metadata::token_decimals', args = {}).contract_result_data.value['Ok']


def token_total_supply(address, chain):
    contract = contract_from_address(address, chain)
    return contract.read(alice, 'PSP22::total_supply', args = {}).contract_result_data.value['Ok']


if __name__ == '__main__':
    chain = SubstrateInterface(url=MAINNET_WS)
    # Below is the Cyberiad token from the button game https://the-button.azero.dev/play
    example_token = "5Dvb5E8zKU4E9c7YxfNL5VC8YQj4VAFUTCGYY9ayFLnnY3UA"
    print(f"Token name: {token_name(example_token, chain)}")
    print(f"Token symbol: {token_symbol(example_token, chain)}")
    print(f"Token decimals: {token_decimals(example_token, chain)}")
    print(f"Token Supply: {token_total_supply(example_token, chain)}")