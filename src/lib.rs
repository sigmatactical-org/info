//! Sigma Info: Markdown-fed information site.

mod config;
mod content;
mod specs;
mod templates;

use std::convert::Infallible;

use warp::Filter;
use warp::{Rejection, Reply};

pub use content::{DocEntry, get, sorted_entries};
pub use sigma_theme::{copyright_years, current_year};

/// Resolve listen address from **`PORT`** (default **8080**).
#[must_use]
pub fn listen_socket_addr_from_env() -> std::net::SocketAddr {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port)
}

fn index_page() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path::end().and(warp::get()).and_then(|| async {
        templates::render_index_html()
            .map(warp::reply::html)
            .map_err(|_| warp::reject::not_found())
    })
}

fn doc_page() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("doc" / String)
        .and(warp::get())
        .and_then(|slug: String| async move {
            let Some(doc) = get(&slug).await else {
                return Err(warp::reject::not_found());
            };
            templates::render_doc_html(&doc)
                .await
                .map(warp::reply::html)
                .map_err(|_| warp::reject::not_found())
        })
}

fn sigma_racer_page() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("products" / "sigma-racer")
        .and(warp::get())
        .and_then(|| async {
            let spec_documents = specs::sigma_racer_specs().await;
            templates::render_sigma_racer_html(spec_documents)
                .map(warp::reply::html)
                .map_err(|_| warp::reject::not_found())
        })
}

fn content_security_policy() -> String {
    let identity_origin = config::identity_public_origin();
    format!(
        "default-src 'self'; base-uri 'self'; object-src 'none'; frame-ancestors 'none'; \
         img-src 'self' data:; style-src 'self' 'unsafe-inline'; script-src 'self'; \
         font-src 'self'; connect-src 'self' {identity_origin}; form-action 'self'"
    )
}

/// Site routes: index, `/doc/{slug}`, `/products/sigma-racer`, `/up`, theme static assets, error recovery.
pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone + Send + 'static
{
    use warp::reply::with::header;

    warp::path("up")
        .and(warp::get())
        .map(|| warp::reply::with_status("up", warp::http::StatusCode::OK))
        .or(sigma_pg::health::warp::health_routes("info", None))
        .or(index_page())
        .or(doc_page())
        .or(sigma_racer_page())
        .or(sigma_theme::warp::static_files())
        .or(sigma_theme::warp::favicon())
        .recover(sigma_theme::warp::handle_rejection)
        .with(header("content-security-policy", content_security_policy()))
        .with(header("x-content-type-options", "nosniff"))
        .with(header("x-frame-options", "DENY"))
        .with(header("referrer-policy", "strict-origin-when-cross-origin"))
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
        // info's chrome renders with show_contact_us: false (see templates.rs).
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
