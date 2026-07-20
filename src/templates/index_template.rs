//! [`IndexTemplate`].

use askama::Template;
use sigma_theme::nav::SiteHeader;

use super::ProjectEntry;

#[derive(Template)]
#[template(path = "index.html")]
pub(crate) struct IndexTemplate {
    pub(crate) projects: &'static [ProjectEntry],
    pub(crate) site_header: SiteHeader,
    pub(crate) site_nav: String,
    pub(crate) copyright_years: String,
}
