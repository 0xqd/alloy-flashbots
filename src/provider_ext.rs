use alloy::network::eip2718::Encodable2718;
use alloy::network::Network;
use alloy::providers::fillers::{FillProvider, TxFiller};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::transports::{Transport, TransportErrorKind, TransportResult};

use crate::mev_provider_builder::MEVProviderBuilder;
use crate::rpc::mev::{BundleItem, SendBundleRequest, SendBundleResponse, SimBundleOverrides};

/// Extension for Provider Builder
pub trait MEVProviderBuilderExt<L, F, N, S> {
    fn with_mev(self) -> MEVProviderBuilder<L, F, N, S>;
}

impl<L, F, N, S> MEVProviderBuilderExt<L, F, N, S> for ProviderBuilder<L, F, N> {
    fn with_mev(self) -> MEVProviderBuilder<L, F, N, S> {
        MEVProviderBuilder::new(self)
    }
}

/// Extension trait for FillProvider to support MEV operations.
#[async_trait::async_trait]
pub trait MEVProviderExt<N: Network> {
    /// Build a bundle item from a transaction request.
    async fn build_bundle_item(
        &self,
        tx: N::TransactionRequest,
        can_revert: bool,
    ) -> TransportResult<BundleItem>;

    /// Submit a bundle to the MEV provider.
    async fn send_bundle(&self, bundle: SendBundleRequest) -> TransportResult<SendBundleResponse>;

    /// Simulate a bundle on the MEV provider.
    async fn simulate_bundle(
        &self,
        bundle: SendBundleRequest,
        simulate_overrides: SimBundleOverrides,
    ) -> TransportResult<SendBundleResponse>;
}

#[async_trait::async_trait]
impl<F, P, T, N> MEVProviderExt<N> for FillProvider<F, P, T, N>
where
    F: TxFiller<N>,
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
    <N as Network>::TxEnvelope: Encodable2718 + Clone,
{
    async fn build_bundle_item(
        &self,
        tx: N::TransactionRequest,
        can_revert: bool,
    ) -> TransportResult<BundleItem> {
        let sendable = self.fill(tx).await?;
        if let Some(envelope) = sendable.as_envelope() {
            Ok(BundleItem::Tx {
                tx: envelope.encoded_2718().into(),
                can_revert,
            })
        } else {
            Err(TransportErrorKind::custom_str(
                "cannot convert transaction to envelope",
            ))
        }
    }

    async fn send_bundle(&self, bundle: SendBundleRequest) -> TransportResult<SendBundleResponse> {
        self.client().request("mev_sendBundle", (bundle,)).await
    }

    async fn simulate_bundle(
        &self,
        bundle: SendBundleRequest,
        simulate_overrides: SimBundleOverrides,
    ) -> TransportResult<SendBundleResponse> {
        self.client()
            .request("mev_simBundle", (bundle, simulate_overrides))
            .await
    }
}
