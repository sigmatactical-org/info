//! [`SpecDocumentView`].

#[allow(unused_imports)]
use super::*;

/// One racer repo document rendered for the specs page.
#[derive(Debug, Clone)]
pub struct SpecDocumentView {
    pub id: String,
    pub label: String,
    pub html: String,
}
