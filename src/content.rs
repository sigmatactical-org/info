//! Load Markdown documents from a GitHub content repository at request time.

use std::collections::BTreeMap;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;
use thiserror::Error;
use tokio::sync::RwLock;

use crate::config;

#[derive(Debug, Clone)]
pub struct DocEntry {
    pub slug: String,
    pub title: String,
    pub order: i32,
    pub body_html: String,
}

#[derive(Debug, Error)]
enum ContentError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("content request failed: {0}")]
    Request(String),
}

#[derive(Debug, Deserialize)]
struct GithubContentEntry {
    name: String,
    download_url: Option<String>,
}

struct CacheState {
    docs: Option<Vec<DocEntry>>,
    fetched_at: Option<Instant>,
}

impl CacheState {
    const fn empty() -> Self {
        Self {
            docs: None,
            fetched_at: None,
        }
    }

    fn is_fresh(&self, ttl: Duration) -> bool {
        self.docs
            .as_ref()
            .is_some_and(|_| self.fetched_at.is_some_and(|at| at.elapsed() < ttl))
    }
}

struct ContentCache {
    client: reqwest::Client,
    state: RwLock<CacheState>,
}

impl ContentCache {
    fn global() -> &'static ContentCache {
        static CACHE: OnceLock<ContentCache> = OnceLock::new();
        CACHE.get_or_init(|| ContentCache {
            client: reqwest::Client::new(),
            state: RwLock::new(CacheState::empty()),
        })
    }
}

/// All documents from the content repository, cached for [`config::content_cache_ttl`].
/// Built-in pages (e.g. Terms) are always merged in so checkout works offline / in kind.
pub async fn docs() -> Vec<DocEntry> {
    let cache = ContentCache::global();
    let ttl = config::content_cache_ttl();

    {
        let state = cache.state.read().await;
        if state.is_fresh(ttl) {
            return state.docs.clone().unwrap_or_default();
        }
    }

    let (owner, repo) = config::content_repo();
    let mut docs = match fetch_docs(
        &cache.client,
        &owner,
        &repo,
        &config::content_repo_path(),
        &config::content_ref(),
    )
    .await
    {
        Ok(docs) => docs,
        Err(_) => {
            let state = cache.state.read().await;
            state.docs.clone().unwrap_or_default()
        }
    };
    merge_builtin_docs(&mut docs);

    let mut state = cache.state.write().await;
    state.docs = Some(docs.clone());
    state.fetched_at = Some(Instant::now());
    docs
}

fn merge_builtin_docs(docs: &mut Vec<DocEntry>) {
    let builtins = [parse_doc("terms.md", include_str!("../content/terms.md"))];
    for builtin in builtins {
        if let Some(existing) = docs.iter_mut().find(|d| d.slug == builtin.slug) {
            *existing = builtin;
        } else {
            docs.push(builtin);
        }
    }
}

/// All documents sorted by `order` then title.
pub async fn sorted_entries() -> Vec<DocEntry> {
    let mut entries = docs().await;
    entries.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.title.cmp(&b.title)));
    entries
}

pub async fn get(slug: &str) -> Option<DocEntry> {
    docs().await.into_iter().find(|doc| doc.slug == slug)
}

async fn fetch_docs(
    client: &reqwest::Client,
    owner: &str,
    repo: &str,
    path: &str,
    git_ref: &str,
) -> Result<Vec<DocEntry>, ContentError> {
    let list_url =
        format!("https://api.github.com/repos/{owner}/{repo}/contents/{path}?ref={git_ref}");
    let response = client
        .get(&list_url)
        .header("accept", "application/vnd.github+json")
        .header("user-agent", "sigma-info")
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(ContentError::Request(format!("list {status}: {body}")));
    }

    let entries: Vec<GithubContentEntry> = response.json().await?;
    let mut docs = Vec::new();

    for entry in entries {
        if !entry.name.ends_with(".md") {
            continue;
        }
        let Some(download_url) = entry.download_url else {
            continue;
        };
        let source = client
            .get(download_url)
            .header("user-agent", "sigma-info")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        docs.push(parse_doc(&entry.name, &source));
    }

    Ok(docs)
}

fn parse_doc(filename: &str, source: &str) -> DocEntry {
    let slug = filename.trim_end_matches(".md").to_string();
    let (meta, markdown) = split_front_matter(source);
    let title = meta
        .get("title")
        .cloned()
        .unwrap_or_else(|| slug.replace('-', " "));
    let order = meta
        .get("order")
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);
    let body_html = markdown_to_html(markdown);
    DocEntry {
        slug,
        title,
        order,
        body_html,
    }
}

fn split_front_matter(source: &str) -> (BTreeMap<String, String>, &str) {
    let mut meta = BTreeMap::new();
    let Some(rest) = source.strip_prefix("---") else {
        return (meta, source);
    };
    let Some((header, body)) = rest.split_once("\n---") else {
        return (meta, source);
    };
    let body = body.trim_start_matches('\n');
    for line in header.lines() {
        if let Some((key, value)) = line.split_once(':') {
            meta.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    (meta, body)
}

fn markdown_to_html(markdown: &str) -> String {
    let mut html_out = String::new();
    let parser = Parser::new_ext(
        markdown,
        Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES,
    );
    html::push_html(&mut html_out, parser);
    html_out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_front_matter_title_and_order() {
        let doc = parse_doc(
            "welcome.md",
            "---\ntitle: Welcome\norder: 1\n---\n\nHello.\n",
        );
        assert_eq!(doc.slug, "welcome");
        assert_eq!(doc.title, "Welcome");
        assert_eq!(doc.order, 1);
        assert!(doc.body_html.contains("<p>Hello.</p>"));
    }

    #[test]
    fn falls_back_to_slug_title_and_default_order_without_front_matter() {
        let doc = parse_doc("getting-started.md", "No front matter here.\n");
        assert_eq!(doc.title, "getting started");
        assert_eq!(doc.order, 100);
    }

    #[test]
    fn builtin_terms_has_title() {
        let mut docs = Vec::new();
        merge_builtin_docs(&mut docs);
        let terms = docs.iter().find(|d| d.slug == "terms").expect("terms");
        assert_eq!(terms.title, "Terms and Conditions");
        assert!(terms.body_html.contains("Deposit"));
    }
}
