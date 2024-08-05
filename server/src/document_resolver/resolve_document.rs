use std::path::PathBuf;
use anyhow::anyhow;
use fs_backend::Hierarchy;
use crate::document_resolver::DocumentResolver;

type DocumentResolverResponse = document::Document;

impl DocumentResolver {
    // resolve_document resolves a single document
    pub fn resolve_document(&self, path_segments: &PathBuf) -> anyhow::Result<&DocumentResolverResponse> {
        let resolved = self.hierarchy
            .resolve(path_segments)
            .expect("failed to resolve file");

        let Hierarchy::File(doc) = resolved else {
            return Err(anyhow!("not a file"))
        };

        Ok(doc)
    }
}