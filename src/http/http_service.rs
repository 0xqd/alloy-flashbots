use alloy::hex;
use alloy::primitives::keccak256;
use std::task::{Context, Poll};

use alloy::rpc::json_rpc::{RequestPacket, ResponsePacket};
use alloy::signers::Signer;
use alloy::transports::http::Http;
use alloy::transports::{TransportError, TransportErrorKind, TransportFut};
use reqwest::Url;
use tower::Service;

#[derive(Clone)]
pub struct MEVHttpService<T, S> {
    http: Http<T>,
    mev_share_url: Url,
    signer: S,
}

impl<T, S> MEVHttpService<T, S>
where
    S: Signer + Clone + Send + Sync + 'static,
{
    pub fn new(http: Http<T>, mev_share_url: Url, signer: S) -> Self {
        Self {
            http,
            mev_share_url,
            signer,
        }
    }
}

impl<S: Signer + Clone + Send + Sync + 'static> MEVHttpService<reqwest::Client, S> {
    fn request_to_mev_share(&self, req: RequestPacket) -> TransportFut<'static> {
        let this = self.clone();

        Box::pin(async move {
            let body = serde_json::to_vec(&req).map_err(TransportError::ser_err)?;

            let signature = this.signer
                .sign_message(format!("{:?}", keccak256(&body)).as_bytes())
                .await
                .map_err(TransportErrorKind::custom)?;

            let flashbot_sig = format!(
                "{:?}:0x{}",
                this.signer.address(),
                hex::encode(signature.as_bytes())
            );
            let resp = this
                .http
                .client()
                .post(this.mev_share_url)
                .header("X-Flashbots-Signature", flashbot_sig.as_str())
                .body(body)
                .send()
                .await
                .map_err(TransportErrorKind::custom)?
                .json::<ResponsePacket>()
                .await
                .map_err(TransportErrorKind::custom)?;

            Ok(resp)
        })
    }
}

/// Implement Tower Service to handle request with mev_ prefix to mev share rpc endpoint.
/// Otherwise, request to node rpc endpoint.
impl<S> Service<RequestPacket> for MEVHttpService<reqwest::Client, S>
where
    S: Signer + Clone + Send + Sync + 'static,
{
    type Response = ResponsePacket;
    type Error = TransportError;
    type Future = TransportFut<'static>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, req: RequestPacket) -> Self::Future {
        match req {
            RequestPacket::Single(single) => match single.method() {
                m if m.starts_with("mev_") => self.request_to_mev_share(single.into()),
                _ => self.http.call(single.into()),
            },
            other => self.http.call(other),
        }
    }
}
