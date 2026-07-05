use askama::Template;

use crate::content::{DocEntry, sorted_entries};
use crate::specs::SpecDocumentView;
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

#[derive(Template)]
#[template(path = "sigma-racer.html")]
struct SigmaRacerTemplate {
    product_url: String,
    spec_documents: Vec<SpecDocumentView>,
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

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_sigma_racer_html(
    spec_documents: Vec<SpecDocumentView>,
) -> Result<String, askama::Error> {
    let store_base = crate::config::store_public_base_url();
    SigmaRacerTemplate {
        product_url: format!(
            "{}/products/SIGMA-RACER",
            store_base.trim_end_matches('/')
        ),
        spec_documents,
        copyright_years: copyright_years(),
    }
    .render()
}
