//! [`SpecsError`].

#[allow(unused_imports)]
use super::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum SpecsError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("racer specs request failed: {0}")]
    Request(String),
}
