//! [`ContentError`].

#[allow(unused_imports)]
use super::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ContentError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("content request failed: {0}")]
    Request(String),
}
