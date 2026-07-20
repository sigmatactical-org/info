//! [`DocTemplate`].

use askama::Template;
use sigma_theme::nav::{NavEntry, SiteHeader};

#[derive(Template)]
#[template(path = "doc.html")]
pub(crate) struct DocTemplate<'a> {
    pub(crate) slug: &'a str,
    pub(crate) title: &'a str,
    pub(crate) body: &'a str,
    pub(crate) nav: Vec<NavEntry>,
    pub(crate) site_header: SiteHeader,
    pub(crate) site_nav: String,
    pub(crate) copyright_years: String,
}
