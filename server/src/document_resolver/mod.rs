mod resolve_hierarchy;
mod resolve_document;
use anyhow::Error;
use fs_backend::{Backend, Hierarchy};

// DocumentResolver is an abstraction over hierarchy and cache
pub struct DocumentResolver {
    hierarchy: Hierarchy,
}

#[derive(thiserror::Error, Debug)]
pub enum Errors {
    #[error("file not found: {0}")]
    FileNotFound(Error),
    
    #[error("hierarchy requested but the leaf is not directory: {0}")]
    InvalidHierarchy(String)
}

impl DocumentResolver {
    pub fn new(backend: Backend) -> anyhow::Result<Self> {
        let hierarchy = backend.construct_hierarchy()?;
        Ok(Self {
            hierarchy,
        })
    }
}
