pub mod document_resolver;
pub mod document_parser;

use std::io::Cursor;

pub trait AppState {
    fn document_resolver(&self) -> impl document_resolver::DocumentResolver;
}

// ---------------------------------------
// document resolvers
// ---------------------------------------

pub trait Document {
    fn get_body(&self) -> serde_json::Value;
    fn get_frontmatter(&self) -> serde_json::Value;
    fn get_comments(&self) -> serde_json::Value;
}
