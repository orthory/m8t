use std::path::PathBuf;
use crate::state;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Serialize};

pub(crate) async fn get_document(
    State(s): State<state::AppState>,
    Path((fragment_type, document_path)): Path<(String, PathBuf)>,
) -> (StatusCode, Json<Option<impl Serialize>>) {
    let dr = s.document_resolver.read().await;
    let document = dr.resolve_document(&document_path).unwrap();
    
    let fragment = document
        .fragments()
        .get(fragment_type.as_str())
        .cloned();

    (
        StatusCode::OK,
        Json(fragment)
    )
}

// TODO: FIXME
#[cfg(test)]
mod tests {
    use crate::document_resolver::*;
    use crate::state;
    use std::io::Write;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use vfs::FileSystem;
    use fs_backend::Backend;
    use super::*;

    fn setup(test_path: &str) -> state::AppState {
        let vfs = vfs::MemoryFS::new();
        let mut f = vfs.create_file(test_path).unwrap();
        f
            .write_all(testutil::fixtures::TestDocumentBuffer.as_bytes())
            .unwrap();

        let state = state::AppState {
            document_resolver: Arc::new(RwLock::new(DocumentResolver::new(Backend::Vfs(vfs)))),
        };

        state
    }

    #[tokio::test]
    async fn get_body_works() {
        let test_path = "/test_path.md";
        let fragment = "frontmatter";
        let state = setup(test_path);
        let (status_code, ret) = get_document(
            State(state),
            Path((test_path.to_string(), PathBuf::from(fragment))),
        )
        .await;

        assert_eq!(status_code, StatusCode::OK);
        let kk = ret.0.unwrap();
        let asdf = serde_json::to_value(kk).unwrap();
        dbg!(asdf.to_string());
    }
}
