use alloy::network::EthereumWallet;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::LocalSigner;

#[tokio::main]
async fn main() {
    let eth_http_rpc = std::env::var("ETH_HTTP_RPC").expect("ETH_HTTP_RPC is not set");
    // TODO: Update your own wallet here
    let signer = LocalSigner::random();
    let wallet = EthereumWallet::new(signer.clone());

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .on_http(eth_http_rpc.parse().unwrap());

    let block_number = provider.get_block_number().await.unwrap();
    println!("Current block number: {}", block_number);
}
