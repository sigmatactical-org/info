mod doc_template;
mod index_template;
mod project_entry;
mod sigma_racer_template;
pub(crate) use doc_template::DocTemplate;
pub(crate) use index_template::IndexTemplate;
pub(crate) use project_entry::ProjectEntry;
pub(crate) use sigma_racer_template::SigmaRacerTemplate;

use askama::Template;

use crate::config;
use crate::content::{DocEntry, sorted_entries};
use crate::specs::SpecDocumentView;
use sigma_theme::copyright_years;
use sigma_theme::nav::{Breadcrumb, NavEntry};
use sigma_theme::site_nav::SiteChrome;

fn chrome() -> SiteChrome {
    SiteChrome {
        title: "Info".to_string(),
        identity_base: config::identity_public_base_url(),
        app_base: config::public_base_url(),
        contact_base: config::contact_public_base_url(),
        cart_url: config::cart_public_base_url(),
        show_cart: true,
    }
}

/// Product pages this site serves, linked from the landing page.
const PROJECTS: &[ProjectEntry] = &[ProjectEntry {
    title: "SIGMA-RACER",
    href: "/products/sigma-racer",
    description: "Build specifications and engineering documents.",
}];

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_index_html() -> Result<String, askama::Error> {
    let chrome = chrome();
    IndexTemplate {
        projects: PROJECTS,
        site_header: chrome.page_header(None),
        site_nav: chrome.site_nav("/", 0)?,
        copyright_years: copyright_years(),
    }
    .render()
}

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_doc_html(doc: &DocEntry, all_docs: &[DocEntry]) -> Result<String, askama::Error> {
    let chrome = chrome();
    let return_path = format!("/doc/{}", doc.slug);
    let nav = sorted_entries(all_docs)
        .into_iter()
        .map(|d| NavEntry {
            slug: d.slug.clone(),
            title: d.title.clone(),
        })
        .collect();
    DocTemplate {
        slug: &doc.slug,
        title: &doc.title,
        body: &doc.body_html,
        nav,
        site_header: chrome
            .page_header(None)
            .with_breadcrumb(Breadcrumb::link("/", "Info"))
            .with_breadcrumb(Breadcrumb::current(doc.title.as_str())),
        site_nav: chrome.site_nav(&return_path, 0)?,
        copyright_years: copyright_years(),
    }
    .render()
}

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_sigma_racer_html(
    spec_documents: &[SpecDocumentView],
) -> Result<String, askama::Error> {
    let chrome = chrome();
    let store_url = config::store_public_base_url();
    let product_url = config::store_product_url("sigma-racer");
    SigmaRacerTemplate {
        spec_documents,
        site_header: chrome
            .page_header(None)
            .with_breadcrumb(Breadcrumb::link(store_url, "Store"))
            .with_breadcrumb(Breadcrumb::link(product_url, "SIGMA-RACER"))
            .with_breadcrumb(Breadcrumb::current("Build specifications")),
        site_nav: chrome.site_nav("/products/sigma-racer", 0)?,
        copyright_years: copyright_years(),
    }
    .render()
}
