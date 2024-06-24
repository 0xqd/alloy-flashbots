use alloy::network::Network;
use alloy::providers::fillers::TxFiller;
use alloy::providers::{ProviderBuilder, ProviderLayer, RootProvider};
use alloy::rpc::client::ClientBuilder;
use alloy::signers::Signer;
use alloy::transports::Transport;
use reqwest::Url;

use crate::http::MEVHttpService;
use crate::MEVHttpLayer;

/// Wrapper around [`ProviderBuilder`] to add MEV endpoints.
#[derive(Debug)]
pub struct MEVProviderBuilder<L, F, N, S> {
    provider_builder: ProviderBuilder<L, F, N>,
    layer: MEVHttpLayer<S>,
}

impl<L, F, N, S> MEVProviderBuilder<L, F, N, S> {
    /// Creates a new MEV provider builder.
    pub fn new(provider_builder: ProviderBuilder<L, F, N>) -> Self {
        Self {
            provider_builder,
            layer: MEVHttpLayer::<S>::new(),
        }
    }

    /// Sets the MEV share URL.
    pub fn with_mev_share_url(mut self, mev_share_url: Url) -> Self {
        self.layer.mev_share_url = Some(mev_share_url);
        self
    }

    /// Sets the signer.
    pub fn with_signer(mut self, signer: S) -> Self {
        self.layer.signer = Some(signer);
        self
    }

    /// Build this provider with an reqwest HTTP transport.
    pub fn on_http(self, url: Url) -> F::Provider
    where
        L: ProviderLayer<
            RootProvider<MEVHttpService<reqwest::Client, S>, N>,
            MEVHttpService<reqwest::Client, S>,
            N,
        >,
        F: TxFiller<N> + ProviderLayer<L::Provider, MEVHttpService<reqwest::Client, S>, N>,
        MEVHttpService<reqwest::Client, S>: Transport + Clone,
        S: Signer + Clone + Send + Sync,
        N: Network,
    {
        let client = ClientBuilder::default().layer(self.layer).http(url);
        self.provider_builder.on_client(client)
    }
}
