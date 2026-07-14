//! [`DocEntry`].

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone)]
pub struct DocEntry {
    pub slug: String,
    pub title: String,
    pub order: i32,
    pub body_html: String,
}
