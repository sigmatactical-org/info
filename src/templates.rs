mod doc_template;
mod index_template;
mod nav_entry;
mod project_entry;
mod sigma_racer_template;
pub(crate) use doc_template::DocTemplate;
pub(crate) use index_template::IndexTemplate;
pub(crate) use nav_entry::NavEntry;
pub(crate) use project_entry::ProjectEntry;
pub(crate) use sigma_racer_template::SigmaRacerTemplate;

use askama::Template;

use crate::content::{DocEntry, sorted_entries};
use crate::specs::SpecDocumentView;
use sigma_theme::copyright_years;
use sigma_theme::nav::{Breadcrumb, SiteHeader, site_menu};
use sigma_theme::site_nav::{AppSiteNav, render_app_site_nav};

fn page_header() -> SiteHeader {
    SiteHeader::new("Info").with_menu(site_menu(None))
}

fn site_nav(return_path: &str) -> Result<String, askama::Error> {
    render_app_site_nav(&AppSiteNav {
        identity_base: &crate::config::identity_public_base_url(),
        app_base: &crate::config::public_base_url(),
        contact_base: &crate::config::contact_public_base_url(),
        cart_url: &crate::config::cart_public_base_url(),
        cart_count: 0,
        return_path,
        show_cart: true,
        show_contact_us: false,
        leading_html: "",
    })
}

/// Product pages this site serves, linked from the landing page.
fn projects() -> Vec<ProjectEntry> {
    vec![ProjectEntry {
        title: "SIGMA-RACER".to_string(),
        href: "/products/sigma-racer".to_string(),
        description: "Build specifications and engineering documents.".to_string(),
    }]
}

async fn nav_entries() -> Vec<NavEntry> {
    sorted_entries()
        .await
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
        projects: projects(),
        site_header: page_header(),
        site_nav: site_nav("/")?,
        copyright_years: copyright_years(),
    }
    .render()
}

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub async fn render_doc_html(doc: &DocEntry) -> Result<String, askama::Error> {
    let return_path = format!("/doc/{}", doc.slug);
    DocTemplate {
        slug: doc.slug.clone(),
        title: doc.title.clone(),
        body: doc.body_html.clone(),
        nav: nav_entries().await,
        site_header: page_header()
            .with_breadcrumb(Breadcrumb::link("/", "Info"))
            .with_breadcrumb(Breadcrumb::current(doc.title.clone())),
        site_nav: site_nav(&return_path)?,
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
    let store_url = crate::config::store_public_base_url();
    let product_url = crate::config::store_product_url("sigma-racer");
    SigmaRacerTemplate {
        spec_documents,
        site_header: page_header()
            .with_breadcrumb(Breadcrumb::link(&store_url, "Store"))
            .with_breadcrumb(Breadcrumb::link(&product_url, "SIGMA-RACER"))
            .with_breadcrumb(Breadcrumb::current("Build specifications")),
        site_nav: site_nav("/products/sigma-racer")?,
        copyright_years: copyright_years(),
    }
    .render()
}
