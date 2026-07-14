//! [`DocTemplate`].

#[allow(unused_imports)]
use super::*;
use askama::Template;
use sigma_theme::nav::SiteHeader;

#[derive(Template)]
#[template(path = "doc.html")]
pub(crate) struct DocTemplate {
    pub(crate) slug: String,
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) nav: Vec<NavEntry>,
    pub(crate) site_header: SiteHeader,
    pub(crate) site_nav: String,
    pub(crate) copyright_years: String,
}
