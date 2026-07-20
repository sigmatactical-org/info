//! Load Markdown documents from a GitHub content repository at request time.

mod doc_entry;
pub use doc_entry::DocEntry;

use std::sync::{Arc, OnceLock};

use sigma_theme::cache::TtlCache;
use sigma_theme::content::{
    ContentError, fetch_markdown_files, markdown_to_html, split_front_matter,
};

use crate::config;

/// User agent identifying this service to the GitHub API.
const USER_AGENT: &str = "sigma-info";

/// TTL cache of the GitHub-backed doc listing (single-flight, stale-on-error).
static DOCS: TtlCache<Vec<DocEntry>> = TtlCache::new();

/// All documents from the content repository, cached for [`config::content_cache_ttl`].
/// Built-in pages (e.g. Terms) are always merged in so checkout works offline / in kind.
pub async fn docs() -> Arc<Vec<DocEntry>> {
    DOCS.get_or_fetch(config::content_cache_ttl(), fetch_docs)
        .await
        .unwrap_or_else(|_| Arc::new(builtin_docs().to_vec()))
}

/// Entries of `docs` sorted by `order` then title.
pub fn sorted_entries(docs: &[DocEntry]) -> Vec<&DocEntry> {
    let mut entries: Vec<&DocEntry> = docs.iter().collect();
    entries.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.title.cmp(&b.title)));
    entries
}

/// Look up a document of `docs` by slug.
pub fn get<'a>(docs: &'a [DocEntry], slug: &str) -> Option<&'a DocEntry> {
    docs.iter().find(|doc| doc.slug == slug)
}

async fn fetch_docs() -> Result<Vec<DocEntry>, ContentError> {
    let (owner, repo) = config::content_repo();
    let files = fetch_markdown_files(
        sigma_pg::clients::http::client(),
        &owner,
        &repo,
        &config::content_repo_path(),
        &config::content_ref(),
        USER_AGENT,
    )
    .await?;
    let mut docs: Vec<DocEntry> = files
        .iter()
        .map(|file| parse_doc(&file.name, &file.markdown))
        .collect();
    merge_builtin_docs(&mut docs);
    Ok(docs)
}

/// Documents embedded in the binary, parsed once per process.
fn builtin_docs() -> &'static [DocEntry] {
    static BUILTINS: OnceLock<Vec<DocEntry>> = OnceLock::new();
    BUILTINS.get_or_init(|| vec![parse_doc("terms.md", include_str!("../content/terms.md"))])
}

fn merge_builtin_docs(docs: &mut Vec<DocEntry>) {
    for builtin in builtin_docs() {
        if let Some(existing) = docs.iter_mut().find(|d| d.slug == builtin.slug) {
            *existing = builtin.clone();
        } else {
            docs.push(builtin.clone());
        }
    }
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

    #[test]
    fn sorted_entries_orders_by_order_then_title() {
        let docs = [
            parse_doc("b.md", "---\ntitle: Bravo\norder: 2\n---\nb"),
            parse_doc("a.md", "---\ntitle: Alpha\norder: 2\n---\na"),
            parse_doc("z.md", "---\ntitle: Zulu\norder: 1\n---\nz"),
        ];
        let sorted: Vec<&str> = sorted_entries(&docs)
            .into_iter()
            .map(|d| d.slug.as_str())
            .collect();
        assert_eq!(sorted, vec!["z", "a", "b"]);
    }
}
