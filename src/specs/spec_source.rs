//! [`SpecSource`].

/// One downloaded spec document before rendering: tab id, label, and raw Markdown.
pub(crate) struct SpecSource {
    pub(crate) id: String,
    pub(crate) label: String,
    pub(crate) markdown: String,
}
