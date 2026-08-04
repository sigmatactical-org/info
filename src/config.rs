//! Environment-driven configuration (public peer URLs and content sources).
//!
//! Required values are declared in the [`sigma_config::service!`] block and
//! checked by [`validate`] at startup.

sigma_config::service! {
    prefix = "INFO";
    role = "info";
    urls {
        /// Public base URL of this info service.
        public_base_url = "PUBLIC_BASE_URL" => "http://127.0.0.1:8080/";
        /// Public base URL of the identity BFF.
        identity_public_base_url = "IDENTITY_PUBLIC_URL" => "http://127.0.0.1:3000/";
        /// Public base URL of the contact service for navbar links.
        contact_public_base_url = "CONTACT_PUBLIC_URL" => "http://127.0.0.1:8083/";
        /// Public base URL of the cart service for navbar links.
        cart_public_base_url = "CART_PUBLIC_URL" => "http://127.0.0.1:8084/";
        /// Public base URL of the store for links back to product pages.
        store_public_base_url = "STORE_PUBLIC_URL" => "http://127.0.0.1:8082/";
    }
}

/// Browser origin of the identity BFF for CSP `connect-src` (no trailing slash).
#[must_use]
pub fn identity_public_origin() -> String {
    sigma_config::origin_of(&identity_public_base_url())
}

/// Storefront URL for a product detail page (`/products/{lowercase-sku}`).
#[must_use]
pub fn store_product_url(sku_code: &str) -> String {
    format!(
        "{}/products/{}",
        store_public_base_url().trim_end_matches('/'),
        sku_code.to_lowercase()
    )
}

/// GitHub repository (`owner/name`) for informational content markdown.
#[must_use]
pub fn content_repo() -> (String, String) {
    let value = SERVICE
        .opt_str("CONTENT_REPO")
        .unwrap_or_else(|| "sigmatactical-org/info".to_string());
    parse_github_repo(&value)
        .unwrap_or_else(|| ("sigmatactical-org".to_string(), "info".to_string()))
}

/// Path within the content repo holding the `.md` documents.
#[must_use]
pub fn content_repo_path() -> String {
    SERVICE
        .opt_str("CONTENT_PATH")
        .unwrap_or_else(|| "content".to_string())
}

/// Git ref (branch, tag, or commit) for informational content.
#[must_use]
pub fn content_ref() -> String {
    SERVICE
        .opt_str("CONTENT_REF")
        .unwrap_or_else(|| "main".to_string())
}

/// In-memory cache TTL for fetched content documents (default 1 hour).
#[must_use]
pub fn content_cache_ttl() -> std::time::Duration {
    const DEFAULT_SECS: u64 = 60 * 60;
    SERVICE
        .opt_str("CONTENT_CACHE_TTL_SECS")
        .and_then(|value| value.parse().ok())
        .map(std::time::Duration::from_secs)
        .unwrap_or_else(|| std::time::Duration::from_secs(DEFAULT_SECS))
}

/// GitHub repository (`owner/name`) for SIGMA-RACER build specs markdown.
#[must_use]
pub fn racer_specs_repo() -> (String, String) {
    let value = SERVICE
        .opt_str("RACER_SPECS_REPO")
        .unwrap_or_else(|| "sigmatactical-org/sigma-racer-specs".to_string());
    parse_github_repo(&value).unwrap_or_else(|| {
        (
            "sigmatactical-org".to_string(),
            "sigma-racer-specs".to_string(),
        )
    })
}

/// Git ref (branch, tag, or commit) for racer specs.
#[must_use]
pub fn racer_specs_ref() -> String {
    SERVICE
        .opt_str("RACER_SPECS_REF")
        .unwrap_or_else(|| "main".to_string())
}

/// In-memory cache TTL for fetched racer specs (default 30 minutes).
#[must_use]
pub fn racer_specs_cache_ttl() -> std::time::Duration {
    const DEFAULT_SECS: u64 = 30 * 60;
    SERVICE
        .opt_str("RACER_SPECS_CACHE_TTL_SECS")
        .and_then(|value| value.parse().ok())
        .map(std::time::Duration::from_secs)
        .unwrap_or_else(|| std::time::Duration::from_secs(DEFAULT_SECS))
}

fn parse_github_repo(value: &str) -> Option<(String, String)> {
    let value = value.trim().trim_start_matches("https://github.com/");
    let (owner, repo) = value.split_once('/')?;
    let repo = repo.trim_end_matches('/').trim_end_matches(".git");
    if owner.is_empty() || repo.is_empty() {
        return None;
    }
    Some((owner.to_string(), repo.to_string()))
}
