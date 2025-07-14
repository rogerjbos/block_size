use subxt::{OnlineClient, PolkadotConfig};
use parity_scale_codec::Encode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a public Polkadot node
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io").await?;

    // Fetch the latest block
    let block = api.blocks().at_latest().await?;

    // Get the header and extrinsics (body)
    let header = block.header().clone();
    let block_number = u32::from(header.number); 
    let extrinsics = block.extrinsics().await?;
    let mut body = Vec::new();
    let mut ex_iter = extrinsics.iter();
    while let Some(ex) = ex_iter.next() {
        body.push(ex.bytes().to_vec());
    }

    let encoded = (header, body).encode();

    println!(
        "Polkadot block {} SCALE-encoded size: {} bytes",
        block_number,
        encoded.len()
    );

    Ok(())
}