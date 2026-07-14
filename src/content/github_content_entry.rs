//! [`GithubContentEntry`].

#[allow(unused_imports)]
use super::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct GithubContentEntry {
    pub(crate) name: String,
    pub(crate) download_url: Option<String>,
}
