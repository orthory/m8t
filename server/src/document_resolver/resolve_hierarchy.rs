use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use fs_backend::Hierarchy;
use crate::document_resolver::{DocumentResolver, Errors};

#[derive(Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "file")]
    File {
        path: String,
        title: Option<String>,
    },

    #[serde(rename = "directory")]
    Directory {
        path: String,
        children_count: usize
    },
}

type ResolveHierarchyResponse = Vec<ItemType>;

impl DocumentResolver {
    // resolve_hierarchy resolves direct descendants from path_segments
    // it's like `ls`
    pub fn resolve_hierarchy(&self, path_segments: &PathBuf) -> anyhow::Result<ResolveHierarchyResponse, Errors> {
        let hierarchy = self.hierarchy.resolve(path_segments)
            .map_err(Errors::FileNotFound)?;

        let Hierarchy::Directory(descendants) = hierarchy else {
            return Err(Errors::InvalidHierarchy(String::from(path_segments.to_string_lossy())))
        };

        // iterate over descendants, collect each item as ItemType
        let descendants: Vec<ItemType> = descendants
            .iter()
            .map(|(path, item) | {
                match item {
                    Hierarchy::File(document) => ItemType::File {
                        path: path.clone(),
                        title: document.title(),
                    },
                    Hierarchy::Directory(children) => ItemType::Directory {
                        path: path.clone(),
                        children_count: children.len(),
                    },
                }
            })
            .collect();

        Ok(descendants)
    }
}