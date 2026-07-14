//! [`SigmaRacerTemplate`].

#[allow(unused_imports)]
use super::*;
use crate::specs::SpecDocumentView;
use askama::Template;
use sigma_theme::nav::SiteHeader;

#[derive(Template)]
#[template(path = "sigma-racer.html")]
pub(crate) struct SigmaRacerTemplate {
    pub(crate) spec_documents: Vec<SpecDocumentView>,
    pub(crate) site_header: SiteHeader,
    pub(crate) site_nav: String,
    pub(crate) copyright_years: String,
}
