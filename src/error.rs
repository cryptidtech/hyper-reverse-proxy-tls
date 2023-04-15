use thiserror::Error;
use hyper::http::header::{InvalidHeaderValue, ToStrError};

#[derive(Error, Debug)]
pub enum ProxyError {
    #[error("invalid uri")]
    InvalidUri(#[from] hyper::http::uri::InvalidUri),
    #[error("hyper error")]
    HyperError(#[from] hyper::Error),
    #[error("forward header error")]
    ForwardHeaderError,
    #[error("upgrade error: {0}")]
    UpgradeError(String),
}

impl From<ToStrError> for ProxyError {
    fn from(_err: ToStrError) -> ProxyError {
        ProxyError::ForwardHeaderError
    }
}

impl From<InvalidHeaderValue> for ProxyError {
    fn from(_err: InvalidHeaderValue) -> ProxyError {
        ProxyError::ForwardHeaderError
    }
}

pub type HyperReverseProxyResult<T> = anyhow::Result<T, ProxyError>;
