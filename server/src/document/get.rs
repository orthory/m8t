use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::state::{Document, document_resolver::DocumentResolver};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GetResponse {
    frontmatter: serde_json::Value,
    body: serde_json::Value,
}

pub(crate) async fn get<S: crate::state::AppState>(
    State(s): State<S>,
    Path(document_path): Path<String>,
) -> (StatusCode, Json<GetResponse>) {
    let document_resolver = s.document_resolver();
    let f = document_resolver
        .read_file(document_path)
        .expect("file not found");

    (
        StatusCode::OK,
        Json(GetResponse{
            frontmatter: f.get_frontmatter(),
            body: f.get_body()
        })
    )
}


#[cfg(test)]
mod tests {
    use std::io::{Cursor};
    use vfs::FileSystem;
    use crate::{Document, DocumentResolver};


}
