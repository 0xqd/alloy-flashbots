use alloy::network::EthereumWallet;
use alloy::primitives::{address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;

use alloy_flashbots::rpc::mev::{Inclusion, SendBundleRequest, SimBundleOverrides};
use alloy_flashbots::{MEVProviderBuilderExt, MEVProviderExt};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = std::env::var("ETH_HTTP_RPC")
        .expect("ETH_HTTP_RPC is not set")
        .parse()?;
    let private_key = std::env::var("ETH_PK").expect("ETH_PK is not set");
    let signer: PrivateKeySigner = private_key.parse()?;
    let wallet = EthereumWallet::new(signer.clone());
    let mev_share_url = "https://relay-sepolia.flashbots.net".parse()?;

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .with_mev()
        .with_signer(signer.clone())
        .with_mev_share_url(mev_share_url)
        .on_http(rpc_url);

    let block_number = provider.get_block_number().await.unwrap();

    // print info, current rpc url, block number and mev_share_url
    println!("MEV Share URL: https://relay-sepolia.flashbots.net");
    println!("Current block number: {}", block_number);

    // simulate a bundle
    let tx = TransactionRequest::default()
        .from(signer.address())
        .to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"))
        .value(U256::from(1_000_000_000));

    let built_bundle_item = provider.build_bundle_item(tx, false).await.unwrap();
    dbg!(&built_bundle_item);

    // build bundle request
    let bundle = SendBundleRequest {
        bundle_body: vec![built_bundle_item],
        inclusion: Inclusion::at_block(block_number + 1),
        ..Default::default()
    };

    let resp = provider
        .simulate_bundle(bundle, SimBundleOverrides::default())
        .await?;
    dbg!(&resp);

    Ok(())
}
