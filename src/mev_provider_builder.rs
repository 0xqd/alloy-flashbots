use alloy::network::Network;
use alloy::providers::fillers::TxFiller;
use alloy::providers::{ProviderBuilder, ProviderLayer, ReqwestProvider};
use alloy::rpc::client::ClientBuilder;
use reqwest::Url;

/// Wrapper around [`ProviderBuilder`] to add MEV endpoints.
#[derive(Debug)]
pub struct MEVProviderBuilder<L, F, N, S> {
    provider_builder: ProviderBuilder<L, F, N>,

    mev_share_url: Option<Url>,
    signer: Option<S>,
}

impl<L, F, N, S> MEVProviderBuilder<L, F, N, S> {
    /// Creates a new MEV provider builder.
    pub fn new(provider_builder: ProviderBuilder<L, F, N>) -> Self {
        Self {
            provider_builder,
            signer: None,
            mev_share_url: None,
        }
    }

    /// Sets the MEV share URL.
    pub fn with_mev_share_url(mut self, mev_share_url: Url) -> Self {
        self.mev_share_url = Some(mev_share_url);
        self
    }

    /// Sets the signer.
    pub fn with_signer(mut self, signer: S) -> Self {
        self.signer = Some(signer);
        self
    }

    /// Build this provider with an Reqwest HTTP transport.
    pub fn on_http(self, url: Url) -> F::Provider
    where
        L: ProviderLayer<ReqwestProvider<N>, alloy::transports::http::Http<reqwest::Client>, N>,
        F: TxFiller<N>
            + ProviderLayer<L::Provider, alloy::transports::http::Http<reqwest::Client>, N>,
        N: Network,
    {
        let client = ClientBuilder::default().http(url);
        self.provider_builder.on_client(client)
    }
}
