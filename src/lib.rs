//! Sigma Info: Markdown-fed information site.

#![forbid(unsafe_code)]

mod config;
mod content;
mod specs;
mod templates;

use std::convert::Infallible;
use std::sync::OnceLock;

use sigma_theme::warp::TemplateError;
use warp::{Filter, Rejection, Reply};

fn index_page() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    static INDEX_HTML: OnceLock<String> = OnceLock::new();
    sigma_theme::warp::cached_page(&INDEX_HTML, templates::render_index_html)
}

fn doc_page() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("doc" / String)
        .and(warp::get())
        .and_then(|slug: String| async move {
            let docs = content::docs().await;
            let Some(doc) = content::get(&docs, &slug) else {
                return Err(warp::reject::not_found());
            };
            templates::render_doc_html(doc, &docs)
                .map(warp::reply::html)
                .map_err(|_| warp::reject::custom(TemplateError))
        })
}

fn sigma_racer_page() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("products" / "sigma-racer")
        .and(warp::get())
        .and_then(|| async {
            let spec_documents = specs::sigma_racer_specs().await;
            templates::render_sigma_racer_html(&spec_documents)
                .map(warp::reply::html)
                .map_err(|_| warp::reject::custom(TemplateError))
        })
}

/// Identity BFF origin for CSP `connect-src`, resolved once per process
/// (the theme's `security_headers` requires a `'static` borrow).
fn identity_origin() -> &'static str {
    static ORIGIN: OnceLock<String> = OnceLock::new();
    ORIGIN.get_or_init(config::identity_public_origin)
}

/// Site routes: index, `/doc/{slug}`, `/products/sigma-racer`, `/up`, sigma-pg
/// health routes, theme static assets, and themed error recovery — wrapped in
/// the shared security header set.
pub fn routes()
-> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone + Send + Sync + 'static {
    sigma_theme::warp::security_headers(
        sigma_theme::warp::site_routes(
            index_page().or(doc_page()).or(sigma_racer_page()),
            sigma_pg::health::warp::health_routes("info", None),
        ),
        identity_origin(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::StatusCode;

    #[tokio::test]
    async fn index_lists_projects() {
        let res = warp::test::request()
            .method("GET")
            .path("/")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), 200);
        let body = std::str::from_utf8(res.body()).unwrap();
        assert!(body.contains("aria-label=\"Cart\""));
        // info's chrome renders without the contact-us header button (see SiteChrome).
        assert!(!body.contains("Contact us"));
        assert!(body.contains("SIGMA-RACER"));
        assert!(body.contains("href=\"/products/sigma-racer\""));
    }

    #[tokio::test]
    async fn doc_page_renders() {
        let res = warp::test::request()
            .method("GET")
            .path("/doc/welcome")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), 200);
        let body = std::str::from_utf8(res.body()).unwrap();
        assert!(body.contains("<h1>Welcome</h1>"));
    }

    #[tokio::test]
    async fn sigma_racer_page_renders() {
        let res = warp::test::request()
            .method("GET")
            .path("/products/sigma-racer")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), 200);
        let body = std::str::from_utf8(res.body()).unwrap();
        assert!(body.contains("Build specifications"));
    }

    #[tokio::test]
    async fn unknown_doc_is_404() {
        let res = warp::test::request()
            .method("GET")
            .path("/doc/no-such-page")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn up_returns_ok() {
        let res = warp::test::request()
            .method("GET")
            .path("/up")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}
