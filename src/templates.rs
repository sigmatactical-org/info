use askama::Template;

use crate::content::{DocEntry, sorted_entries};
use sigma_theme::copyright_years;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<NavEntry>,
    copyright_years: String,
}

#[derive(Template)]
#[template(path = "doc.html")]
struct DocTemplate {
    slug: String,
    title: String,
    body: String,
    nav: Vec<NavEntry>,
    copyright_years: String,
}

#[derive(Clone)]
struct NavEntry {
    slug: String,
    title: String,
}

fn nav_entries() -> Vec<NavEntry> {
    sorted_entries()
        .into_iter()
        .map(|d| NavEntry {
            slug: d.slug.clone(),
            title: d.title.clone(),
        })
        .collect()
}

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_index_html() -> Result<String, askama::Error> {
    IndexTemplate {
        entries: nav_entries(),
        copyright_years: copyright_years(),
    }
    .render()
}

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_doc_html(doc: &DocEntry) -> Result<String, askama::Error> {
    DocTemplate {
        slug: doc.slug.clone(),
        title: doc.title.clone(),
        body: doc.body_html.clone(),
        nav: nav_entries(),
        copyright_years: copyright_years(),
    }
    .render()
}
