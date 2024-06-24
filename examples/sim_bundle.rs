use alloy::network::EthereumWallet;
use alloy::primitives::{address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::LocalSigner;

use alloy_flashbots::{MEVProviderBuilderExt, MEVProviderExt};

#[tokio::main]
async fn main() {
    let eth_http_rpc = std::env::var("ETH_HTTP_RPC").expect("ETH_HTTP_RPC is not set");
    // TODO: Update your own wallet here
    let signer = LocalSigner::random();
    let wallet = EthereumWallet::new(signer.clone());

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .with_mev()
        .with_signer(signer.clone())
        .on_http(eth_http_rpc.parse().unwrap());

    let block_number = provider.get_block_number().await.unwrap();
    println!("Current block number: {}", block_number);

    // simulate a bundle
    let tx = TransactionRequest::default()
        .from(signer.address())
        .to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"))
        .value(U256::from(1_000_000_000));

    let built_bundle_item = provider.build_bundle_item(tx, false).await.unwrap();
    dbg!(&built_bundle_item);

    // let sim = provider.simulate_bundle(vec![built_bundle_item]).await.unwrap();
}
