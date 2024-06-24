use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::primitives::{address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use eyre::Result;

use alloy_flashbots::rpc::mev::{Inclusion, SendBundleRequest, SimBundleOverrides};
use alloy_flashbots::{MEVProviderBuilderExt, MEVProviderExt};

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().try_spawn()?;
    let rpc_url = anvil.endpoint_url();

    // Set up signer from the first default Anvil account (Alice).
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer.clone());

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .with_mev()
        .with_signer(signer.clone())
        .with_mev_share_url("https://relay.flashbots.net".parse().unwrap())
        .on_http(rpc_url.clone());

    let block_number = provider.get_block_number().await.unwrap();
    println!("Current block number: {}", block_number);

    // simulate a bundle
    let tx = TransactionRequest::default()
        .from(signer.address())
        .to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"))
        .value(U256::from(1_000_000_000));

    let built_bundle_item = provider.build_bundle_item(tx, false).await?;
    dbg!(&built_bundle_item);

    // build bundle request
    let bundle = SendBundleRequest {
        bundle_body: vec![built_bundle_item],
        inclusion: Inclusion::at_block(20163696),
        ..Default::default()
    };

    let resp = provider
        .simulate_bundle(bundle, SimBundleOverrides::default())
        .await?;
    dbg!(&resp);

    // let sim = provider.simulate_bundle(vec![built_bundle_item]).await.unwrap();
    Ok(())
}
