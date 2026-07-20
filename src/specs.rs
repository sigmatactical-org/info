//! Build spec sheets fetched from the racer GitHub repository at request time.

mod spec_document_view;
mod spec_source;
pub use spec_document_view::SpecDocumentView;
pub(crate) use spec_source::SpecSource;

use std::sync::Arc;

use sigma_theme::cache::TtlCache;
use sigma_theme::content::{
    ContentError, download_markdown_files, list_repo_dir, markdown_to_html,
};

use crate::config;

/// User agent identifying this service to the GitHub API.
const USER_AGENT: &str = "sigma-info";

/// Preferred tab order; unknown documents are appended alphabetically by label.
const SPEC_TAB_ORDER: &[&str] = &[
    "overview",
    "build",
    "engine",
    "chassis",
    "bodywork",
    "electrical",
    "electronics",
    "efi",
    "emissions",
];

/// Root markdown that is not a build specification (legal, meta, tooling).
const NON_SPEC_MARKDOWN: &[&str] = &[
    "BRANDING.md",
    "CHANGELOG.md",
    "CODE_OF_CONDUCT.md",
    "CONTRIBUTING.md",
    "LICENSE.md",
    "SECURITY.md",
];

/// TTL cache of the GitHub-backed spec listing (single-flight, stale-on-error).
static SPECS: TtlCache<Vec<SpecDocumentView>> = TtlCache::new();

/// Load SIGMA-RACER build specs from the racer repository, cached for
/// [`config::racer_specs_cache_ttl`]. Serves an empty list when nothing is
/// cached and the fetch fails (the template renders an unavailable notice).
pub async fn sigma_racer_specs() -> Arc<Vec<SpecDocumentView>> {
    SPECS
        .get_or_fetch(config::racer_specs_cache_ttl(), fetch_racer_specs)
        .await
        .unwrap_or_default()
}

async fn fetch_racer_specs() -> Result<Vec<SpecDocumentView>, ContentError> {
    let (owner, repo) = config::racer_specs_repo();
    let client = sigma_pg::clients::http::client();
    let entries = list_repo_dir(
        client,
        &owner,
        &repo,
        "",
        &config::racer_specs_ref(),
        USER_AGENT,
    )
    .await?;
    let spec_entries = entries
        .into_iter()
        .filter(|entry| is_spec_markdown(&entry.name))
        .collect();
    let files = download_markdown_files(client, spec_entries, USER_AGENT).await?;

    let mut sources: Vec<SpecSource> = files
        .into_iter()
        .map(|file| SpecSource {
            id: spec_id_from_filename(&file.name),
            label: spec_label_from_filename(&file.name),
            markdown: file.markdown,
        })
        .collect();
    sources.sort_by(compare_spec_sources);

    Ok(sources
        .into_iter()
        .map(|source| SpecDocumentView {
            id: source.id,
            label: source.label,
            html: markdown_to_html(&source.markdown),
        })
        .collect())
}

fn is_spec_markdown(filename: &str) -> bool {
    if !filename.ends_with(".md") {
        return false;
    }
    !NON_SPEC_MARKDOWN
        .iter()
        .any(|name| name.eq_ignore_ascii_case(filename))
}

fn spec_id_from_filename(filename: &str) -> String {
    match filename {
        "README.md" => "overview".to_string(),
        "emissions_certification.md" => "emissions".to_string(),
        other => other.trim_end_matches(".md").to_string(),
    }
}

fn spec_label_from_filename(filename: &str) -> String {
    match filename {
        "README.md" => "Overview".to_string(),
        "emissions_certification.md" => "Emissions".to_string(),
        other => title_from_snake_case(other.trim_end_matches(".md")),
    }
}

fn title_from_snake_case(value: &str) -> String {
    value
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn compare_spec_sources(left: &SpecSource, right: &SpecSource) -> std::cmp::Ordering {
    let left_rank = SPEC_TAB_ORDER
        .iter()
        .position(|id| *id == left.id)
        .unwrap_or(usize::MAX);
    let right_rank = SPEC_TAB_ORDER
        .iter()
        .position(|id| *id == right.id)
        .unwrap_or(usize::MAX);
    left_rank
        .cmp(&right_rank)
        .then_with(|| left.label.cmp(&right.label))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_repo_filenames_to_tab_ids() {
        assert_eq!(spec_id_from_filename("README.md"), "overview");
        assert_eq!(spec_id_from_filename("chassis.md"), "chassis");
        assert_eq!(
            spec_id_from_filename("emissions_certification.md"),
            "emissions"
        );
        assert_eq!(spec_label_from_filename("efi.md"), "Efi");
    }

    #[test]
    fn skips_non_spec_root_markdown() {
        assert!(!is_spec_markdown("BRANDING.md"));
        assert!(!is_spec_markdown("branding.md"));
        assert!(!is_spec_markdown("LICENSE.md"));
        assert!(!is_spec_markdown("CONTRIBUTING.md"));
        assert!(!is_spec_markdown("Cargo.toml"));
        assert!(is_spec_markdown("README.md"));
        assert!(is_spec_markdown("engine.md"));
        assert!(is_spec_markdown("emissions_certification.md"));
    }

    #[test]
    fn orders_known_tabs_before_unknown_alphabetically() {
        let mut sources = [
            SpecSource {
                id: "zzz".into(),
                label: "Zzz".into(),
                markdown: String::new(),
            },
            SpecSource {
                id: "engine".into(),
                label: "Engine".into(),
                markdown: String::new(),
            },
            SpecSource {
                id: "overview".into(),
                label: "Overview".into(),
                markdown: String::new(),
            },
        ];
        sources.sort_by(compare_spec_sources);
        assert_eq!(
            sources.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(),
            vec!["overview", "engine", "zzz"]
        );
    }
}
