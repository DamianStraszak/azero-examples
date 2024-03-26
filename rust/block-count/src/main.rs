use std::str::FromStr;

use subxt::dynamic::{At, Value};
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://ws.azero.dev:443")
        .await
        .unwrap();
    //let connection_context =
    //    ConnectionContext(Arc::new(Mutex::new(Connection::new("wss://ws.azero.dev:430").await)));
    //let mut guard = connection_context.0.lock().await;

    let account =
        AccountId32::from_str("5CoxXHzXRZrnVkmdDhYVHz5b6PJdCRmR5yGSQmQwzasHyzaw").unwrap();
    let storage_query = subxt::dynamic::storage(
        "Elections",
        "SessionValidatorBlockCount",
        vec![Value::from_bytes(account)],
    );

    //let at_block_hash = PolkadotConfig::Hash::from_str("0x2e675599e2da64628b57e3ea6a9abfcd7db15289436ab58b00b13a8473760561").unwrap();
    let block_hash = api
                .rpc()
                .block_hash(Some((59583 * 900 + 899u32).into())).await.unwrap().unwrap();
    let result = api
        .storage()
        .at(block_hash)
        .fetch(&storage_query)
        .await
        .unwrap();
    let value = result.unwrap().to_value().unwrap();
    //let value = result.unwrap().to_value()?;

    println!("{:?}", value);
    Ok(())
}
