fn normalize_base_url(url: &str) -> String {
    let mut url = url.trim().to_string();
    if !url.ends_with('/') {
        url.push('/');
    }
    url
}

/// Public base URL of the store for links back to product pages.
#[must_use]
pub fn store_public_base_url() -> String {
    std::env::var("INFO_STORE_PUBLIC_URL")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| normalize_base_url(&s))
        .unwrap_or_else(|| "http://127.0.0.1:8082/".to_string())
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

/// GitHub repository (`owner/name`) for SIGMA-RACER build specs markdown.
#[must_use]
pub fn racer_specs_repo() -> (String, String) {
    let value = std::env::var("INFO_RACER_SPECS_REPO")
        .unwrap_or_else(|_| "sigmatactical-org/racer".to_string());
    parse_github_repo(&value)
        .unwrap_or_else(|| ("sigmatactical-org".to_string(), "racer".to_string()))
}

/// Git ref (branch, tag, or commit) for racer specs.
#[must_use]
pub fn racer_specs_ref() -> String {
    std::env::var("INFO_RACER_SPECS_REF")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "main".to_string())
}

/// In-memory cache TTL for fetched racer specs (default 30 minutes).
#[must_use]
pub fn racer_specs_cache_ttl() -> std::time::Duration {
    const DEFAULT_SECS: u64 = 30 * 60;
    std::env::var("INFO_RACER_SPECS_CACHE_TTL_SECS")
        .ok()
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
