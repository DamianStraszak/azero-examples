#![allow(missing_docs)]
use subxt::{OnlineClient, PolkadotConfig, config::PolkadotExtrinsicParamsBuilder};
use subxt_signer::sr25519::dev;
use serde::Deserialize;
use subxt::{backend::{rpc::RpcClient, legacy::LegacyRpcMethods}};

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let dest = dev::bob().public_key().into();
    let balance_transfer_tx = polkadot::tx().balances().transfer_allow_death(dest, 10_00000000000);

    let from = dev::alice();
    let other_params = PolkadotExtrinsicParamsBuilder::<PolkadotConfig>::new()
        .build();
    let tx = api.tx().create_signed(&balance_transfer_tx, &from, other_params).await.unwrap();



    let rpc_client = RpcClient::from_url("ws://localhost:9944").await?;
    let rpc = LegacyRpcMethods::<PolkadotConfig>::new(rpc_client);
    let at = None;
    
    let dry_run_bytes = rpc.dry_run(tx.encoded(), at).await.unwrap();
    //let decoded: Result<(), TransactionValidityError> = Decode::decode(&mut &dry_run_bytes.0[..]).unwrap();
    let dry_run = dry_run_bytes.into_dry_run_result(&api.metadata()).unwrap();
    println!("Dry run: {:?}", dry_run);



    let events = tx.submit_and_watch().await?.wait_for_finalized_success().await?;

    for e in events.iter() {
        let e = e.unwrap().as_root_event::<polkadot::Event>().unwrap();
        println!("Event: {:?}", e);
    }



    // let events = api
    //     .tx()
    //     .sign_and_submit_then_watch_default(&balance_transfer_tx, &from)
    //     .await?
    //     .wait_for_finalized_success()
    //     .await?;

    // // Find a Transfer event and print it.
    // let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;
    // if let Some(event) = transfer_event {
    //     println!("Balance transfer success: {event:?}");
    // }

    Ok(())
}
