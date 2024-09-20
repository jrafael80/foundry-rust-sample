use bindings::multi_send::MultiSend;

//use ethers::{providers::test_provider::SEPOLIA, types::{Address, U256}};
use ethers::prelude::*; 

use eyre::Result;
use std::sync::Arc;

const RPC_URL: &str = "http://localhost:8545";
const SIGNING_KEY: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from(RPC_URL)?; //SEPOLIA.provider();
    let signer = SIGNING_KEY.parse::<LocalWallet>()?;
    let provider = provider.with_signer(signer);
    let provider = Arc::new(provider);

    let address = "0x5FbDB2315678afecb367f032d93F642f64180aa3".parse::<Address>()?;
    let to = vec!["0x70997970C51812dc3A010C7d01b50e0d17dc79C8".parse::<Address>()?, 
            "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC".parse::<Address>()?];
    let value = vec!["10000".parse::<U256>()?, "20000".parse::<U256>()?];
    let multi_send = MultiSend::new(address, provider);
    multi_send.send_many(to, value).await?;
    println!("Send done!");
    
    Ok(())
}