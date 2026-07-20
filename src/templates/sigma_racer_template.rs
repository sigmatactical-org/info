//! [`SigmaRacerTemplate`].

use askama::Template;
use sigma_theme::nav::SiteHeader;

use crate::specs::SpecDocumentView;

#[derive(Template)]
#[template(path = "sigma-racer.html")]
pub(crate) struct SigmaRacerTemplate<'a> {
    pub(crate) spec_documents: &'a [SpecDocumentView],
    pub(crate) site_header: SiteHeader,
    pub(crate) site_nav: String,
    pub(crate) copyright_years: String,
}
