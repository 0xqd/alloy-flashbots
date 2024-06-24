use crate::http::http_service::MEVHttpService;
use alloy::transports::http::Http;
use reqwest::Url;
use tower::Layer;

#[derive(Debug, Clone)]
pub struct MEVHttpLayer<S> {
    pub mev_share_url: Option<Url>,
    pub signer: Option<S>,
}

impl<S> Default for MEVHttpLayer<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> MEVHttpLayer<S> {
    pub fn new() -> Self {
        Self {
            mev_share_url: None,
            signer: None,
        }
    }
}

// Implement tower layer.
impl<S: Clone> Layer<Http<reqwest::Client>> for MEVHttpLayer<S> {
    type Service = MEVHttpService<reqwest::Client, S>;

    fn layer(&self, service: Http<reqwest::Client>) -> Self::Service {
        let mev_share_url = self
            .mev_share_url
            .clone()
            .expect("MEV share URL is required");
        MEVHttpService::new(service, mev_share_url, self.signer.clone())
    }
}
