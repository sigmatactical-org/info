//! [`DocEntry`].

/// One informational document: slug, title, sort order, and rendered body.
#[derive(Debug, Clone)]
pub struct DocEntry {
    pub slug: String,
    pub title: String,
    pub order: i32,
    pub body_html: String,
}
