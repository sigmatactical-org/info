use askama::Template;

use crate::content::{DocEntry, sorted_entries};
use crate::specs::SpecDocumentView;
use sigma_theme::copyright_years;
use sigma_theme::nav::{Breadcrumb, SiteHeader};
use sigma_theme::site_nav::{AppSiteNav, render_app_site_nav};

fn page_header(brand: &str) -> SiteHeader {
    SiteHeader::new(brand)
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
        show_contact_us: true,
        leading_html: "",
    })
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    projects: Vec<ProjectEntry>,
    site_header: SiteHeader,
    site_nav: String,
    copyright_years: String,
}

#[derive(Clone)]
struct ProjectEntry {
    title: String,
    href: String,
    description: String,
}

/// Product pages this site serves, linked from the landing page.
fn projects() -> Vec<ProjectEntry> {
    vec![ProjectEntry {
        title: "SIGMA-RACER".to_string(),
        href: "/products/sigma-racer".to_string(),
        description: "Build specifications and engineering documents.".to_string(),
    }]
}

#[derive(Template)]
#[template(path = "doc.html")]
struct DocTemplate {
    slug: String,
    title: String,
    body: String,
    nav: Vec<NavEntry>,
    site_header: SiteHeader,
    site_nav: String,
    copyright_years: String,
}

#[derive(Template)]
#[template(path = "sigma-racer.html")]
struct SigmaRacerTemplate {
    spec_documents: Vec<SpecDocumentView>,
    site_header: SiteHeader,
    site_nav: String,
    copyright_years: String,
}

#[derive(Clone)]
struct NavEntry {
    slug: String,
    title: String,
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
        site_header: page_header("Sigma Info"),
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
        site_header: page_header("Sigma Info")
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
        site_header: page_header("Sigma Info")
            .with_breadcrumb(Breadcrumb::link(&store_url, "Store"))
            .with_breadcrumb(Breadcrumb::link(&product_url, "SIGMA-RACER"))
            .with_breadcrumb(Breadcrumb::current("Build specifications")),
        site_nav: site_nav("/products/sigma-racer")?,
        copyright_years: copyright_years(),
    }
    .render()
}
