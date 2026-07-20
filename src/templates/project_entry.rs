//! [`ProjectEntry`].

/// A product page linked from the landing page.
pub(crate) struct ProjectEntry {
    pub(crate) title: &'static str,
    pub(crate) href: &'static str,
    pub(crate) description: &'static str,
}
