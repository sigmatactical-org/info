//! [`SpecSource`].

#[allow(unused_imports)]
use super::*;

/// One GitHub repo/path the spec listing is drawn from.
pub(crate) struct SpecSource {
    pub(crate) id: String,
    pub(crate) label: String,
    pub(crate) markdown: String,
}
