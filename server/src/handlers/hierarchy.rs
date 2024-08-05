use std::path::PathBuf;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use crate::state;

pub(crate) async fn get_hierarchy(
    State(s): State<state::AppState>,
    Path(segments): Path<PathBuf>
) -> (StatusCode, Json<impl Serialize>) {
    let dr = s.document_resolver.read().await;
    let hierarchy = dr.resolve_hierarchy(&segments).unwrap();

    (
        StatusCode::OK,
        Json(hierarchy)
    )
}